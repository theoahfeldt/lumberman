use glfw::{Action, Context as _, Key, WindowEvent};
use lumber::{
    game::{Game, PlayerAction},
    game_graphics::{self, GameResources, UIResources},
    object,
    semantics::{Semantics, ShaderInterface},
};
use luminance_front::{
    blending::{Blending, Equation, Factor},
    context::GraphicsContext,
    pipeline::PipelineState,
    render_state::RenderState,
};
use luminance_glfw::GlfwSurface;
use luminance_windowing::{WindowDim, WindowOpt};
use nalgebra::{Matrix4, Point3, Vector3};
use std::{process::exit, time::Instant};

const VS_STR: &str = include_str!("vs.glsl");
const FS_STR: &str = include_str!("fs.glsl");

const UI_VS_STR: &str = include_str!("ui_vs.glsl");
const UI_FS_STR: &str = include_str!("ui_fs.glsl");

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

    let mut ui_program = ctxt
        .new_shader_program::<Semantics, (), ShaderInterface>()
        .from_strings(UI_VS_STR, None, None, UI_FS_STR)
        .unwrap()
        .ignore_warnings();

    let render_st = &RenderState::default().set_blending(Blending {
        equation: Equation::Additive,
        src: Factor::SrcAlpha,
        dst: Factor::Zero,
    });

    let mut rm = object::ResourceManager::new();
    let game_resources = GameResources::new(&mut rm, &mut ctxt);
    let ui_resources = UIResources::new(&mut rm, &mut ctxt);

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
            game_graphics::update_ui_resources(&game, &ui_resources, &mut rm, &mut ctxt);
            action = None;
        }

        // rendering code goes here
        // get the current time and create a color based on the time
        let t = start_t.elapsed().as_millis() as f32 * 1e-3;
        let color = [t.cos(), t.sin(), 0.5, 1.];

        let ui_objects = game_graphics::make_ui(&ui_resources);
        let game_objects = game_graphics::make_scene(&game, &game_resources);
        let render = ctxt
            .new_pipeline_gate()
            .pipeline(
                &back_buffer,
                &PipelineState::default().set_clear_color(color),
                |pipeline, mut shd_gate| {
                    shd_gate.shade(&mut ui_program, |mut iface, uni, mut rdr_gate| {
                        rdr_gate.render(&RenderState::default(), |mut tess_gate| {
                            ui_objects.iter().try_for_each(|ui| {
                                iface.set(&uni.model_transform, ui.transform.to_matrix().into());
                                rm.get_model2(&ui.model).clone().iter().try_for_each(|o| {
                                    let bound_tex =
                                        pipeline.bind_texture(rm.get_texture(&o.texture))?;
                                    iface.set(&uni.tex, bound_tex.binding());
                                    iface.set(&uni.local_transform, o.get_transform().into());
                                    tess_gate.render(rm.get_tess(&o.tess))
                                })
                            })
                        })
                    })?;

                    shd_gate.shade(&mut program, |mut iface, uni, mut rdr_gate| {
                        iface.set(&uni.projection, projection.into());
                        iface.set(&uni.view, view.into());
                        rdr_gate.render(&render_st, |mut tess_gate| {
                            game_objects.iter().try_for_each(|gm| {
                                iface.set(&uni.model_transform, gm.transform.to_matrix().into());
                                rm.get_model(&gm.model).clone().iter().try_for_each(|o| {
                                    let bound_tex =
                                        pipeline.bind_texture(rm.get_texture(&o.texture))?;
                                    iface.set(&uni.tex, bound_tex.binding());
                                    iface.set(&uni.local_transform, o.get_transform().into());
                                    tess_gate.render(rm.get_tess(&o.tess))
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
