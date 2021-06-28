use crate::semantics::{Vertex, VertexNormal, VertexPosition};
use cgmath::{Matrix4, Vector3};
use itertools::Itertools;
use luminance_front::context::GraphicsContext;
use luminance_front::tess::{Interleaved, Mode, Tess, TessError};
use luminance_front::Backend;

pub type VertexIndex = u32;

pub struct Mesh {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<VertexIndex>,
}

impl Mesh {
    pub fn to_tess<C>(
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
}

pub struct Transform {
    pub position: Vector3<f32>,
    pub scale: f32,
    pub orientation: Vector3<f32>,
}

impl Transform {
    pub fn to_matrix(&self) -> Matrix4<f32> {
        let local_transform = Matrix4::<f32>::from_translation(self.position);
        local_transform
    }

    pub fn new() -> Self {
        let position = Vector3::new(0., 0., 0.);
        let scale = 1.;
        let orientation = Vector3::new(0., 0., 0.);
        Self {
            position,
            scale,
            orientation,
        }
    }
}

pub struct Object<'a> {
    pub mesh: &'a Tess<Vertex, VertexIndex, (), Interleaved>,
    pub transform: Transform,
}

impl Object<'_> {
    pub fn get_transform(&self) -> Matrix4<f32> {
        let local_transform = self.transform.to_matrix();
        local_transform
    }
}

pub fn cylinder(height: f32, radius: f32, res: u32) -> Mesh {
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
