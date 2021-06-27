use cgmath::{perspective, EuclideanSpace, Matrix4, Point3, Rad, Vector3};
use glfw::{Action, Context as _, Key, WindowEvent};
use itertools::Itertools;
use luminance_derive::{Semantics, UniformInterface, Vertex};
use luminance_front::context::GraphicsContext;
use luminance_front::pipeline::PipelineState;
use luminance_front::render_state::RenderState;
use luminance_front::shader::Uniform;
use luminance_front::tess::{Interleaved, Mode, Tess, TessError};
use luminance_front::Backend;
use luminance_glfw::GlfwSurface;
use luminance_windowing::{WindowDim, WindowOpt};
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::Read as _;
use std::path::Path;
use std::process::exit;
use std::time::Instant;
use try_guard::verify;
use wavefront_obj::obj;

const VS_STR: &str = include_str!("vs.glsl");
const FS_STR: &str = include_str!("fs.glsl");

const FOVY: Rad<f32> = Rad(std::f32::consts::FRAC_PI_2);
const Z_NEAR: f32 = 0.1;
const Z_FAR: f32 = 10.;

#[derive(Debug, UniformInterface)]
struct ShaderInterface {
	#[uniform(unbound)]
	projection: Uniform<[[f32; 4]; 4]>,
	#[uniform(unbound)]
	view: Uniform<[[f32; 4]; 4]>,
	#[uniform(unbound)]
	local_transform: Uniform<[[f32; 4]; 4]>,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Semantics)]
pub enum Semantics {
	#[sem(name = "position", repr = "[f32; 3]", wrapper = "VertexPosition")]
	Position,
	#[sem(name = "normal", repr = "[f32; 3]", wrapper = "VertexNormal")]
	Normal,
	#[sem(name = "in_position", repr = "[f32; 3]", wrapper = "InstancePosition")]
	InstancePosition,
	// reference vertex size in vertex shaders (used for vertex instancing)
	#[sem(name = "weight", repr = "f32", wrapper = "InstanceWeight")]
	Weight,
}

#[derive(Clone, Copy, Debug, Vertex)]
#[vertex(sem = "Semantics")]
struct Vertex {
	position: VertexPosition,
	normal: VertexNormal,
}

#[derive(Clone, Copy, Debug, PartialEq, Vertex)]
#[vertex(sem = "Semantics", instanced = "true")]
struct Instance {
	pos: InstancePosition,
	w: InstanceWeight,
}

type VertexIndex = u32;

struct Mesh {
	vertices: Vec<Vertex>,
	indices: Vec<VertexIndex>,
}

struct Object<'a> {
	mesh: &'a Tess<Vertex, VertexIndex, (), Interleaved>,
	position: Vector3<f32>,
	scale: f32,
	orientation: Vector3<f32>,
}

impl Object<'_> {
	fn get_transform(&self) -> Matrix4<f32> {
		let local_transform = Matrix4::<f32>::from_translation(self.position);
		local_transform
	}
}

struct InObj<'a> {
	source: &'a Mesh,
	instances: Vec<Instance>,
}

impl InObj<'_> {
	fn to_tess<C>(
		self,
		ctxt: &mut C,
	) -> Result<Tess<Vertex, VertexIndex, Instance, Interleaved>, TessError>
	where
		C: GraphicsContext<Backend = Backend>,
	{
		ctxt.new_tess()
			.set_mode(Mode::Triangle)
			.set_vertices(&self.source.vertices[..])
			.set_indices(&self.source.indices[..])
			.set_instances(&self.instances[..])
			.build()
	}
}

fn cylinder(height: f32, radius: f32, res: u32) -> Mesh {
	let co2 = (0..res)
		.map(|n| std::f32::consts::PI * 2. * (n as f32) / (res as f32))
		.map(|a| (a.cos() * radius, a.sin() * radius));
	let (co2_top, co2_bot) = co2.tee();
	let top = height / 2.;
	let bot = -top;
	let top_verts = co2_top.map(|(x, y)| {
		Vertex::new(
			VertexPosition::new([x, y, top]),
			VertexNormal::new([x, y, 0.]),
		)
	});
	let bot_verts = co2_bot.map(|(x, y)| {
		Vertex::new(
			VertexPosition::new([x, y, bot]),
			VertexNormal::new([x, y, 0.]),
		)
	});
	let vertices = top_verts.chain(bot_verts).collect();
	let indices = (0..res)
		.flat_map(|n| {
			vec![
				n,
				(n + 1) % res,
				n + res,
				n + res,
				(n + 1) % res + res,
				(n + 1) % res,
			]
		})
		.map(|i| i as VertexIndex)
		.collect();
	Mesh { vertices, indices }
}

impl Mesh {
	fn to_tess<C>(
		self,
		ctxt: &mut C,
	) -> Result<Tess<Vertex, VertexIndex, (), Interleaved>, TessError>
	where
		C: GraphicsContext<Backend = Backend>,
	{
		ctxt.new_tess()
			.set_mode(Mode::Triangle)
			.set_vertices(self.vertices)
			.set_indices(self.indices)
			.build()
	}

	fn load<P>(path: P) -> Result<Self, String>
	where
		P: AsRef<Path>,
	{
		let file_content = {
			let mut file = File::open(path).map_err(|e| format!("cannot open file: {}", e))?;
			let mut content = String::new();
			file.read_to_string(&mut content).unwrap();
			content
		};
		let obj_set = obj::parse(file_content).map_err(|e| format!("cannot parse: {:?}", e))?;
		let objects = obj_set.objects;

		verify!(objects.len() == 1).ok_or("expecting a single object".to_owned())?;

		let object = objects.into_iter().next().unwrap();

		verify!(object.geometry.len() == 1).ok_or("expecting a single geometry".to_owned())?;

		let geometry = object.geometry.into_iter().next().unwrap();

		println!("loading {}", object.name);
		println!("{} vertices", object.vertices.len());
		println!("{} shapes", geometry.shapes.len());

		// build up vertices; for this to work, we remove duplicated vertices by putting them in a
		// map associating the vertex with its ID
		let mut vertex_cache: HashMap<obj::VTNIndex, VertexIndex> = HashMap::new();
		let mut vertices: Vec<Vertex> = Vec::new();
		let mut indices: Vec<VertexIndex> = Vec::new();

		for shape in geometry.shapes {
			if let obj::Primitive::Triangle(a, b, c) = shape.primitive {
				for key in &[a, b, c] {
					if let Some(vertex_index) = vertex_cache.get(key) {
						indices.push(*vertex_index);
					} else {
						let p = object.vertices[key.0];
						let n = object.normals
							[key.2.ok_or("missing normal for a vertex".to_owned())?];
						let position = VertexPosition::new([p.x as f32, p.y as f32, p.z as f32]);
						let normal = VertexNormal::new([n.x as f32, n.y as f32, n.z as f32]);
						let vertex = Vertex { position, normal };
						let vertex_index = vertices.len() as VertexIndex;

						vertex_cache.insert(*key, vertex_index);
						vertices.push(vertex);
						indices.push(vertex_index);
					}
				}
			} else {
				return Err("unsupported non-triangle shape".to_owned());
			}
		}

		Ok(Mesh { vertices, indices })
	}
}

fn main() {
	let dim = WindowDim::Windowed {
		width: 960,
		height: 540,
	};
	let surface = GlfwSurface::new_gl33("Hello, world!", WindowOpt::default().set_dim(dim));

	match surface {
		Ok(surface) => {
			eprintln!("graphics surface created");
			main_loop(surface);
		}

		Err(e) => {
			eprintln!("cannot create graphics surface:\n{}", e);
			exit(1);
		}
	}
}

fn main_loop(surface: GlfwSurface) {
	let path = env::args()
		.skip(1)
		.next()
		.expect("first argument must be the path of the .obj file to view");
	println!("loading {}", path);

	let mut ctxt = surface.context;
	let events = surface.events_rx;
	let back_buffer = ctxt.back_buffer().expect("back buffer");
	let start_t = Instant::now();

	//let mesh = Obj::load(path).unwrap().to_tess(&mut ctxt).unwrap();
	let source = cylinder(1., 0.5, 20);
	// let object = InObj {
	// 	source : &source,
	// 	instances : vec![Instance::new([0.5, 0., 0.].into(), (1.).into(), [0., 0., 1.,].into())]
	// };
	let mesh = source.to_tess(&mut ctxt).unwrap();
	let object1 = Object {
		mesh: &mesh,
		position: Vector3::<f32>::new(0., 0., 0.),
		scale: 1.,
		orientation: Vector3::<f32>::new(0., 0., 0.),
	};
	let object2 = Object {
		mesh: &mesh,
		position: Vector3::<f32>::new(0., 1., 0.),
		scale: 1.,
		orientation: Vector3::<f32>::new(0., 0., 0.),
	};

	let objects = vec![object1, object2];

	let mut program = ctxt
		.new_shader_program::<Semantics, (), ShaderInterface>()
		.from_strings(VS_STR, None, None, FS_STR)
		.unwrap()
		.ignore_warnings();

	let [width, height] = back_buffer.size();
	let projection = perspective(FOVY, width as f32 / height as f32, Z_NEAR, Z_FAR);

	let view =
		Matrix4::<f32>::look_at(Point3::new(0., 0., 4.), Point3::origin(), Vector3::unit_y());

	'app: loop {
		// handle events
		ctxt.window.glfw.poll_events();
		for (_, event) in glfw::flush_messages(&events) {
			match event {
				WindowEvent::Close | WindowEvent::Key(Key::Escape, _, Action::Release, _) => {
					break 'app
				}
				_ => (),
			}
		}

		// rendering code goes here
		// get the current time and create a color based on the time
		let t = start_t.elapsed().as_millis() as f32 * 1e-3;
		let color = [t.cos(), t.sin(), 0.5, 1.];

		let render = ctxt
			.new_pipeline_gate()
			.pipeline(
				&back_buffer,
				&PipelineState::default().set_clear_color(color),
				|_, mut shd_gate| {
					shd_gate.shade(&mut program, |mut iface, uni, mut rdr_gate| {
						iface.set(&uni.projection, projection.into());
						iface.set(&uni.view, view.into());

						rdr_gate.render(&RenderState::default(), |mut tess_gate| {
							objects.iter().try_for_each(|obj| {
								iface.set(&uni.local_transform, obj.get_transform().into());
								tess_gate.render(obj.mesh)
							})
						})
					})
				},
			)
			.assume();

		// swap buffer chains
		if render.is_ok() {
			ctxt.window.swap_buffers();
		} else {
			break 'app;
		}
	}
}
