use crate::{
    game::Game,
    geometry,
    menu::Menu,
    object::{Model, Model2, Object, Object2, ResourceManager, TessResource, TextureResource},
    text,
    transform::{Transform, Transform2},
};
use luminance::context::GraphicsContext;
use luminance_front::Backend;
use rapier3d::na::{RealField, Translation3, UnitQuaternion, Vector3};
use std::collections::HashMap;

pub struct GameObject {
    pub model: Model,
    pub transform: Transform,
}

pub struct UIObject {
    pub model: Model2,
    pub transform: Transform2,
}

pub struct GameResources {
    pub log: Model,
    pub branch_left: Model,
    pub branch_right: Model,
}

pub struct UIResources {
    pub char_textures: HashMap<char, TextureResource>,
    pub unit_quad: TessResource,
    pub start: Model2,
    pub quit: Model2,
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
        let start = vec![Object2 {
            tess: unit_quad.clone(),
            texture: start_txt,
            transform: Transform2 {
                scale: Some([0.8, 0.3]),
                rotation: None,
                translation: None,
            },
        }];

        let quit_txt = rm.make_texture(ctxt, &text::make_text("Quit"));
        let quit = vec![Object2 {
            tess: unit_quad.clone(),
            texture: quit_txt,
            transform: Transform2 {
                scale: Some([0.8, 0.3]),
                rotation: None,
                translation: None,
            },
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

        let bark_img = image::io::Reader::open("textures/log_texture.png")
            .unwrap()
            .decode()
            .unwrap()
            .into_rgba8();
        let bark = rm.make_texture(ctxt, &bark_img);

        let angle: f32 = RealField::frac_pi_2();
        let log_obj = Object {
            tess: cylinder.clone(),
            texture: bark.clone(),
            transform: Transform {
                translation: None,
                scale: None,
                rotation: Some(UnitQuaternion::from_axis_angle(
                    &Vector3::<f32>::x_axis(),
                    -angle,
                )),
            },
        };
        let mut branch = Object {
            tess: cylinder,
            texture: bark,
            transform: Transform {
                translation: Some(Translation3::new(-0.9, 0., 0.)),
                scale: Some([0.2, 0.2, 1.]),
                rotation: Some(UnitQuaternion::from_axis_angle(
                    &Vector3::<f32>::y_axis(),
                    RealField::frac_pi_2(),
                )),
            },
        };
        let log: Vec<Object> = vec![log_obj.clone()];
        let branch_left: Vec<Object> = vec![log_obj.clone(), branch.clone()];
        branch.transform.translation = Some(Translation3::new(0.9, 0., 0.));
        let branch_right: Vec<Object> = vec![log_obj, branch];
        Self {
            log,
            branch_left,
            branch_right,
        }
    }
}

pub fn make_ui(game: &Game, resources: &UIResources) -> Vec<UIObject> {
    let model = game
        .get_score()
        .to_string()
        .chars()
        .enumerate()
        .map(|(i, c)| Object2 {
            tess: resources.unit_quad.clone(),
            texture: resources.char_textures.get(&c).unwrap().clone(),
            transform: Transform2 {
                scale: None,
                rotation: None,
                translation: Some(Translation3::new(i as f32, 0., 0.)),
            },
        })
        .collect();
    let score = UIObject {
        model,
        transform: Transform2 {
            scale: Some([0.25, 0.5]),
            rotation: None,
            translation: Some(Translation3::new(-0.8, 0.7, 0.)),
        },
    };
    vec![score]
}

pub fn make_menu(menu: &Menu, resources: &UIResources) -> Vec<UIObject> {
    let buttons = [resources.start.clone(), resources.quit.clone()];
    let selected = menu.selected_idx;
    let start_pos = 0.5;
    buttons
        .iter()
        .enumerate()
        .map(|(i, m)| UIObject {
            model: m.clone(),
            transform: Transform2 {
                scale: if i == selected {
                    Some([1.1, 1.1])
                } else {
                    None
                },
                rotation: None,
                translation: Some(Translation3::new(0., start_pos - i as f32, 0.)),
            },
        })
        .collect()
}
