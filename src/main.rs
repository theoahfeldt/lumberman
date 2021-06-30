use glfw::{Action, Context as _, Key, WindowEvent};
use lumber::game::{Game, PlayerAction};
use lumber::game_graphics::{self, GameModels};
use lumber::object;
use lumber::object::{Object, Transform};
use lumber::semantics::{Semantics, ShaderInterface};
use luminance_front::context::GraphicsContext;
use luminance_front::pipeline::PipelineState;
use luminance_front::render_state::RenderState;
use luminance_glfw::GlfwSurface;
use luminance_windowing::{WindowDim, WindowOpt};
use nalgebra::{Matrix4, Point3, RealField, Translation3, UnitQuaternion, Vector3};
use std::process::exit;
use std::time::Instant;

const VS_STR: &str = include_str!("vs.glsl");
const FS_STR: &str = include_str!("fs.glsl");

const FOVY: f32 = std::f32::consts::FRAC_PI_2;
const Z_NEAR: f32 = 0.1;
const Z_FAR: f32 = 10.;

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
    let mut ctxt = surface.context;
    let events = surface.events_rx;
    let back_buffer = ctxt.back_buffer().expect("back buffer");
    let start_t = Instant::now();

    let mut program = ctxt
        .new_shader_program::<Semantics, (), ShaderInterface>()
        .from_strings(VS_STR, None, None, FS_STR)
        .unwrap()
        .ignore_warnings();

    let cylinder = object::cylinder(1., 0.5, 20).to_tess(&mut ctxt).unwrap();
    let log = Object {
        mesh: &cylinder,
        transform: Transform {
            translation: None,
            scale: None,
            orientation: Some(UnitQuaternion::from_axis_angle(
                &Vector3::<f32>::x_axis(),
                RealField::frac_pi_2(),
            )),
        },
        texture: None,
    };
    let branch = Object {
        mesh: &cylinder,
        transform: Transform {
            translation: Some(Translation3::new(0.9, 0., 0.)),
            scale: Some([0.2, 0.2, 1.]),
            orientation: Some(UnitQuaternion::from_axis_angle(
                &Vector3::<f32>::y_axis(),
                RealField::frac_pi_2(),
            )),
        },
        texture: None,
    };
    let log2 = log.clone();
    let models = GameModels {
        log: vec![log],
        branch_log: vec![log2, branch],
    };

    let mut game = Game::new();
    let mut action: Option<PlayerAction> = None;

    let [width, height] = back_buffer.size();
    let projection = Matrix4::new_perspective(width as f32 / height as f32, FOVY, Z_NEAR, Z_FAR);

    let view = Matrix4::look_at_rh(
        &Point3::new(0., 1., 3.),
        &Point3::new(0., 1., 0.),
        &Vector3::y_axis(),
    );

    'app: loop {
        // handle events
        ctxt.window.glfw.poll_events();
        for (_, event) in glfw::flush_messages(&events) {
            match event {
                WindowEvent::Close | WindowEvent::Key(Key::Escape, _, Action::Release, _) => {
                    break 'app
                }
                WindowEvent::Key(Key::Left, _, Action::Press, _) => {
                    action = Some(PlayerAction::ChopLeft)
                }
                WindowEvent::Key(Key::Right, _, Action::Press, _) => {
                    action = Some(PlayerAction::ChopRight)
                }
                _ => (),
            }
        }

        if let Some(a) = action {
            game.update(a);
            action = None;
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
                            game_graphics::to_scene(&game, &models)
                                .iter()
                                .try_for_each(|m| {
                                    iface.set(&uni.model_transform, m.transform.to_matrix().into());
                                    m.model.iter().try_for_each(|o| {
                                        iface.set(&uni.local_transform, o.get_transform().into());
                                        tess_gate.render(o.mesh)
                                    })
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
