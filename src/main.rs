use glfw::{Action, Context as _, Key, WindowEvent};
use lumber::{
    animation::GameAnimations,
    audio::AudioResources,
    game_graphics::{self, GameResources, UIResources},
    game_state::{GameAction, GameRunner},
    object,
    semantics::{Semantics, ShaderInterface},
};
use luminance_front::{
    blending::{Blending, Equation, Factor},
    context::GraphicsContext,
    depth_test::DepthWrite,
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
        dst: Factor::SrcAlphaComplement,
    });

    let mut rm = object::ResourceManager::new();
    let game_resources = GameResources::new(&mut rm, &mut ctxt);
    let ui_resources = UIResources::new(&mut rm, &mut ctxt);
    let game_animations = GameAnimations::new(&mut rm, &mut ctxt);
    let audio_resources = AudioResources::new();

    let mut runner = GameRunner::new(game_animations);
    let mut action: Option<GameAction> = None;

    // Background
    let background_object = game_graphics::make_background(&mut rm, &mut ctxt);

    let [width, height] = back_buffer.size();
    let projection = Matrix4::new_perspective(width as f32 / height as f32, FOVY, Z_NEAR, Z_FAR);

    let view = Matrix4::look_at_rh(
        &Point3::new(0., 1.2, 2.5),
        &Point3::new(0., 1.4, 0.),
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
                WindowEvent::Key(Key::Left, _, Action::Press, _) => action = Some(GameAction::Left),
                WindowEvent::Key(Key::Right, _, Action::Press, _) => {
                    action = Some(GameAction::Right)
                }
                WindowEvent::Key(Key::Up, _, Action::Press, _) => action = Some(GameAction::Up),
                WindowEvent::Key(Key::Down, _, Action::Press, _) => action = Some(GameAction::Down),
                WindowEvent::Key(Key::Enter, _, Action::Press, _) => {
                    action = Some(GameAction::Enter)
                }
                _ => (),
            }
        }

        if runner.update(action) {
            break 'app;
        }
        runner.play_audio(&audio_resources);
        action = None;

        // rendering code goes here
        // get the current time and create a color based on the time
        let t = start_t.elapsed().as_millis() as f32 * 1e-3;
        let color = [t.cos(), t.sin(), 0.5, 1.];

        let ui_objects = runner.make_ui(&ui_resources);
        let game_objects = runner.make_scene(&game_resources);
        let render = ctxt
            .new_pipeline_gate()
            .pipeline(
                &back_buffer,
                &PipelineState::default().set_clear_color(color),
                |pipeline, mut shd_gate| {
                    shd_gate.shade(&mut ui_program, |mut iface, uni, mut rdr_gate| {
                        rdr_gate.render(
                            &RenderState::default().set_depth_write(DepthWrite::Off),
                            |mut tess_gate| {
                                iface.set(&uni.model_transform, Matrix4::identity().into());

                                let bound_tex = pipeline
                                    .bind_texture(rm.get_texture(&background_object.texture))?;
                                iface.set(&uni.tex, bound_tex.binding());
                                iface.set(&uni.local_transform, background_object.transform.into());
                                tess_gate.render(rm.get_tess(&background_object.tess))
                            },
                        )
                    })?;
                    shd_gate.shade(&mut program, |mut iface, uni, mut rdr_gate| {
                        iface.set(&uni.projection, projection.into());
                        iface.set(&uni.view, view.into());
                        rdr_gate.render(&render_st, |mut tess_gate| {
                            game_objects.iter().try_for_each(|gm| {
                                iface.set(&uni.model_transform, gm.transform.into());
                                gm.model.clone().iter().try_for_each(|o| {
                                    let bound_tex =
                                        pipeline.bind_texture(rm.get_texture(&o.texture))?;
                                    iface.set(&uni.tex, bound_tex.binding());
                                    iface.set(&uni.local_transform, o.transform.into());
                                    tess_gate.render(rm.get_tess(&o.tess))
                                })
                            })
                        })
                    })?;
                    shd_gate.shade(&mut ui_program, |mut iface, uni, mut rdr_gate| {
                        rdr_gate.render(&render_st, |mut tess_gate| {
                            ui_objects.iter().try_for_each(|ui| {
                                iface.set(&uni.model_transform, ui.transform.into());
                                ui.model.clone().iter().try_for_each(|o| {
                                    let bound_tex =
                                        pipeline.bind_texture(rm.get_texture(&o.texture))?;
                                    iface.set(&uni.tex, bound_tex.binding());
                                    iface.set(&uni.local_transform, o.transform.into());
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
