use crate::{
    animation::Animation,
    game::{Game, PlayerPos},
    geometry,
    menu::Menu,
    object::{Model, Object, ResourceManager, TessResource, TextureResource},
    text, transform,
};
use image::{imageops, io::Reader};
use luminance::context::GraphicsContext;
use luminance_front::Backend;
use rapier3d::na::{Matrix4, RealField, UnitQuaternion, Vector3};
use std::collections::HashMap;

pub struct GameObject {
    pub model: Model,
    pub transform: Matrix4<f32>,
}

pub struct GameResources {
    pub log: Model,
    pub branch_left: Model,
    pub branch_right: Model,
    pub unit_quad: TessResource,
}

pub struct UIResources {
    pub char_textures: HashMap<char, TextureResource>,
    pub unit_quad: TessResource,
    pub start: Model,
    pub quit: Model,
}

impl UIResources {
    fn char_to_texture(
        rm: &mut ResourceManager,
        ctxt: &mut impl GraphicsContext<Backend = Backend>,
        c: char,
    ) -> TextureResource {
        let img = text::make_char(c);
        rm.make_texture(ctxt, &img)
    }

    pub fn new(
        rm: &mut ResourceManager,
        ctxt: &mut impl GraphicsContext<Backend = Backend>,
    ) -> Self {
        let unit_quad = rm.make_tess(ctxt, geometry::quad(1., 1.));

        let char_textures = (b'0'..=b'z')
            .map(|c| c as char)
            .map(|c| (c, Self::char_to_texture(rm, ctxt, c)))
            .collect();

        let start_txt = rm.make_texture(ctxt, &text::make_text("Start"));
        let start = vec![Object {
            tess: unit_quad,
            texture: start_txt,
            transform: transform::scale2(0.8, 0.3),
        }];

        let quit_txt = rm.make_texture(ctxt, &text::make_text("Quit"));
        let quit = vec![Object {
            tess: unit_quad,
            texture: quit_txt,
            transform: transform::scale2(0.8, 0.3),
        }];

        Self {
            char_textures,
            unit_quad,
            start,
            quit,
        }
    }
}

impl GameResources {
    pub fn new(
        rm: &mut ResourceManager,
        ctxt: &mut impl GraphicsContext<Backend = Backend>,
    ) -> Self {
        let cylinder = rm.make_tess(ctxt, geometry::cylinder(1., 0.5, 20));

        let bark_img = Reader::open("textures/log_texture.png")
            .unwrap()
            .decode()
            .unwrap()
            .into_rgba8();
        let bark = rm.make_texture(ctxt, &bark_img);

        let angle: f32 = RealField::frac_pi_2();
        let log_obj = Object {
            tess: cylinder,
            texture: bark,
            transform: UnitQuaternion::from_axis_angle(&Vector3::<f32>::x_axis(), -angle)
                .to_homogeneous(),
        };
        let rot_scale = Matrix4::from_axis_angle(&Vector3::<f32>::y_axis(), RealField::frac_pi_2())
            * transform::scale3(0.2, 0.2, 1.);
        let mut branch = Object {
            tess: cylinder,
            texture: bark,
            transform: transform::translation3(-0.9, 0., 0.) * rot_scale,
        };
        let log: Vec<Object> = vec![log_obj.clone()];
        let branch_left: Vec<Object> = vec![log_obj.clone(), branch.clone()];
        branch.transform = transform::translation3(0.9, 0., 0.) * rot_scale;
        let branch_right: Vec<Object> = vec![log_obj, branch];

        let unit_quad = rm.make_tess(ctxt, geometry::quad(1., 1.));
        Self {
            log,
            branch_left,
            branch_right,
            unit_quad,
        }
    }
}

fn make_text_object(
    resources: &UIResources,
    text: String,
    height: f32,
    width: f32,
    x: f32,
    y: f32,
) -> GameObject {
    let model = text
        .chars()
        .enumerate()
        .map(|(i, c)| Object {
            tess: resources.unit_quad,
            texture: *resources.char_textures.get(&c).unwrap(),
            transform: transform::translation3(i as f32, 0., 0.),
        })
        .collect();
    let length = text.len() as f32;
    let centering = transform::translation2(0.5 - length / 2., 0.);
    let scale = transform::scale2(width / length, height);
    GameObject {
        model,
        transform: transform::translation2(x, y) * scale * centering,
    }
}

pub fn make_ui(game: &Game, resources: &UIResources) -> Vec<GameObject> {
    let text = game.get_score().to_string();
    let len = text.len() as f32;
    let score = make_text_object(resources, text, 0.4, 0.2 * len, 0., 0.75);
    vec![score]
}

pub fn make_menu(menu: &Menu, resources: &UIResources) -> Vec<GameObject> {
    let buttons = [resources.start.clone(), resources.quit.clone()];
    let selected = menu.selected_idx;
    let start_pos = 0.5;
    buttons
        .iter()
        .enumerate()
        .map(|(i, m)| GameObject {
            model: m.clone(),
            transform: {
                let mut transform = transform::translation3(0., start_pos - i as f32, 0.);
                if i == selected {
                    transform *= transform::scale2(1.2, 1.2);
                }
                transform
            },
        })
        .collect()
}

pub fn make_player(game: &Game, resources: &GameResources, chop: &Animation) -> GameObject {
    let mut pos_x = -1.1;
    let mut transform = transform::scale3(1.2, 1.2, 1.);
    if game.get_player_pos() == PlayerPos::Right {
        pos_x *= -1.;
        transform *= transform::reflect_x();
    }
    GameObject {
        model: vec![Object {
            tess: resources.unit_quad,
            texture: chop.get_current_texture(),
            transform,
        }],
        transform: transform::translation2(pos_x, 0.5),
    }
}

pub fn make_background(
    rm: &mut ResourceManager,
    ctxt: &mut impl GraphicsContext<Backend = Backend>,
) -> Object {
    let img = Reader::open("textures/forest-background.jpg")
        .unwrap()
        .decode()
        .unwrap()
        .into_rgba8();
    let texture = rm.make_texture(ctxt, &imageops::flip_vertical(&img));
    let tess = rm.make_tess(ctxt, geometry::quad(2., 2.));
    Object {
        tess,
        texture,
        transform: transform::translation3(0., 0., -1.),
    }
}

pub fn make_game_over_ui(game: &Game, resources: &UIResources) -> Vec<GameObject> {
    let text = game.get_score().to_string();
    let len = text.len() as f32;
    let score = make_text_object(resources, text, 0.4, 0.2 * len, 0., 0.);
    let text = "SCORE".to_string();
    let len = text.len() as f32;
    let text = make_text_object(resources, text, 0.4, 0.2 * len, 0., 0.5);
    vec![text, score]
}
