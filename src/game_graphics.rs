use crate::{
    game::{Branch, Game},
    geometry,
    menu::Menu,
    object::{Model, Model2, Object, Object2, ResourceManager, TessResource, TextureResource},
    transform::{Transform, Transform2},
};
use image::{imageops, DynamicImage, GenericImage, ImageBuffer, Rgb, Rgba, RgbaImage};
use luminance::context::GraphicsContext;
use luminance_front::Backend;
use rapier3d::na::{RealField, Translation3, UnitQuaternion, Vector3};
use rusttype::{point, Font, Scale};
use std::{collections::HashMap, iter::FromIterator};

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
        let img = make_char(c);
        rm.make_texture(ctxt, &img)
    }

    pub fn new(
        rm: &mut ResourceManager,
        ctxt: &mut impl GraphicsContext<Backend = Backend>,
    ) -> Self {
        let unit_quad = rm.make_tess(ctxt, geometry::quad(1., 1.));

        let char_textures = HashMap::from_iter(
            (b'0'..=b'z')
                .map(|c| c as char)
                .map(|c| (c, Self::char_to_texture(rm, ctxt, c))),
        );

        let start_txt = rm.make_texture(ctxt, &make_text("Start"));
        let start = vec![Object2 {
            tess: unit_quad.clone(),
            texture: start_txt.clone(),
            transform: Transform2 {
                scale: Some([0.8, 0.3]),
                rotation: None,
                translation: None,
            },
        }];

        let quit_txt = rm.make_texture(ctxt, &make_text("Quit"));
        let quit = vec![Object2 {
            tess: unit_quad.clone(),
            texture: quit_txt.clone(),
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

        let bark_img = image::io::Reader::open("textures/pine-tree-bark-texture.jpg")
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

pub fn make_char_image(c: char, font: Font, scale: Scale, color: Rgb<u8>) -> RgbaImage {
    let glyph = font.glyph(c).scaled(scale).positioned(point(0., 0.));
    let bounding_box = glyph.pixel_bounding_box().unwrap();

    let mut image =
        DynamicImage::new_rgba8(bounding_box.width() as u32, bounding_box.height() as u32);

    let [r, g, b] = color.0;

    glyph.draw(|x, y, v| image.put_pixel(x as u32, y as u32, Rgba([r, g, b, (v * 255.) as u8])));

    imageops::flip_vertical(&image)
}

pub fn make_char(c: char) -> RgbaImage {
    let scale = rusttype::Scale::uniform(256.0);
    let font_data = include_bytes!("../fonts/Courier New.ttf");
    let font = rusttype::Font::try_from_bytes(font_data as &[u8]).expect("Constructing font");
    make_char_image(c, font, scale, Rgb([150, 0, 0]))
}

pub fn make_text_image(text: &str, font: Font, scale: Scale, color: Rgb<u8>) -> RgbaImage {
    let v_metrics = font.v_metrics(scale);

    // layout the glyphs in a line with 20 pixels padding
    let glyphs: Vec<_> = font
        .layout(text, scale, point(20.0, 20.0 + v_metrics.ascent))
        .collect();

    // work out the layout size
    let glyphs_height = (v_metrics.ascent - v_metrics.descent).ceil() as u32;
    let glyphs_width = {
        let min_x = glyphs
            .first()
            .map(|g| g.pixel_bounding_box().unwrap().min.x)
            .unwrap();
        let max_x = glyphs
            .last()
            .map(|g| g.pixel_bounding_box().unwrap().max.x)
            .unwrap();
        (max_x - min_x) as u32
    };

    // Create a new rgb image with some padding
    let mut image = ImageBuffer::from_pixel(
        glyphs_width + 40,
        glyphs_height + 40,
        Rgba([255, 255, 255, 0]),
    );

    let [r, g, b] = color.0;

    // Loop through the glyphs in the text, positing each one on a line
    for glyph in glyphs {
        if let Some(bounding_box) = glyph.pixel_bounding_box() {
            // Draw the glyph into the image per-pixel by using the draw closure
            glyph.draw(|x, y, v| {
                image.put_pixel(
                    // Offset the position by the glyph bounding box
                    x + bounding_box.min.x as u32 - 20,
                    y + bounding_box.min.y as u32,
                    // Turn the coverage into an alpha value
                    Rgba([r, g, b, (v * 255.) as u8]),
                )
            });
        }
    }
    image.save("test.png");
    imageops::flip_vertical(&image)
}

pub fn make_text(text: &str) -> RgbaImage {
    let scale = rusttype::Scale::uniform(256.0);
    let font_data = include_bytes!("../fonts/Courier New.ttf");
    let font = rusttype::Font::try_from_bytes(font_data as &[u8]).expect("Constructing font");
    make_text_image(text, font, scale, Rgb([150, 0, 0]))
}

pub fn make_scene(game: &Game, resources: &GameResources) -> Vec<GameObject> {
    game.tree
        .clone()
        .iter()
        .enumerate()
        .map(|(i, val)| {
            let model = match val {
                Branch::None => resources.log.clone(),
                Branch::Left => resources.branch_left.clone(),
                Branch::Right => resources.branch_right.clone(),
            };
            let rotation = None;
            let transform = Transform {
                scale: None,
                rotation,
                translation: Some(Translation3::new(0., i as f32, 0.)),
            };
            GameObject { model, transform }
        })
        .collect()
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
