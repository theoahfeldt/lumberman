use crate::{
    game::{Branch, Game},
    object::{self, DefaultTess, Object, RgbTexture, Transform},
};
use image::{imageops, ImageBuffer, Rgb, RgbImage};
use luminance_front::{context::GraphicsContext, Backend};
use nalgebra::{RealField, Translation3, UnitQuaternion, Vector3};
use rusttype::{point, Font, Scale};
use std::collections::HashMap;

pub type Model = Vec<Object>;

pub struct GameObject<'a> {
    pub model: &'a Model,
    pub transform: Transform,
}

pub struct GameModels {
    pub log: Model,
    pub branch_log: Model,
}

pub fn load_textures(
    ctxt: &mut impl GraphicsContext<Backend = Backend>,
) -> HashMap<String, RgbTexture> {
    let img = image::io::Reader::open("../textures/pine-tree-bark-texture.jpg")
        .unwrap()
        .decode()
        .unwrap()
        .into_rgb8();
    let bark = object::make_texture(ctxt, &img);
    let mut textures = HashMap::new();
    textures.insert("bark".to_string(), bark);
    textures
}

pub fn load_tesses(
    ctxt: &mut impl GraphicsContext<Backend = Backend>,
) -> HashMap<String, DefaultTess> {
    let cylinder = object::cylinder(1., 0.5, 20).to_tess(ctxt).unwrap();
    let mut objects = HashMap::new();
    objects.insert("cylinder".to_string(), cylinder);
    objects
}

pub fn load_models() -> GameModels {
    let angle: f32 = RealField::frac_pi_2();
    let log = Object {
        tess: "cylinder".to_string(),
        transform: Transform {
            translation: None,
            scale: None,
            orientation: Some(UnitQuaternion::from_axis_angle(
                &Vector3::<f32>::x_axis(),
                -angle,
            )),
        },
        texture: "bark".to_string(),
    };
    let branch = Object {
        tess: "cylinder".to_string(),
        transform: Transform {
            translation: Some(Translation3::new(0.9, 0., 0.)),
            scale: Some([0.2, 0.2, 1.]),
            orientation: Some(UnitQuaternion::from_axis_angle(
                &Vector3::<f32>::y_axis(),
                RealField::frac_pi_2(),
            )),
        },
        texture: "bark".to_string(),
    };
    let log2 = log.clone();

    GameModels {
        log: vec![log],
        branch_log: vec![log2, branch],
    }
}

pub fn make_text_image(text: &str, font: Font, scale: Scale) -> RgbImage {
    let bg_color = Vector3::new(255., 255., 255.);
    let color = Vector3::new(150., 0., 0.);

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
        Rgb([bg_color.x as u8, bg_color.y as u8, bg_color.z as u8]),
    );

    // Loop through the glyphs in the text, positing each one on a line
    for glyph in glyphs {
        if let Some(bounding_box) = glyph.pixel_bounding_box() {
            // Draw the glyph into the image per-pixel by using the draw closure
            glyph.draw(|x, y, v| {
                let color_vec = bg_color * (1. - v) + color * v;
                image.put_pixel(
                    // Offset the position by the glyph bounding box
                    x + bounding_box.min.x as u32,
                    y + bounding_box.min.y as u32,
                    // Turn the coverage into an alpha value
                    Rgb([color_vec.x as u8, color_vec.y as u8, color_vec.z as u8]),
                )
            });
        }
    }

    imageops::flip_vertical(&image)
}

pub fn make_text(text: String) -> () {}

pub fn to_scene<'a>(game: &Game, models: &'a GameModels) -> Vec<GameObject<'a>> {
    game.tree
        .clone()
        .iter()
        .enumerate()
        .map(|(i, val)| {
            let model = match val {
                Branch::None => &models.log,
                Branch::Left | Branch::Right => &models.branch_log,
            };
            let orientation = match val {
                Branch::Right => Some(UnitQuaternion::from_axis_angle(
                    &Vector3::<f32>::y_axis(),
                    RealField::pi(),
                )),
                Branch::Left | Branch::None => None,
            };
            let transform = Transform {
                translation: Some(Translation3::new(0., i as f32, 0.)),
                scale: None,
                orientation,
            };
            GameObject { model, transform }
        })
        .collect()
}
