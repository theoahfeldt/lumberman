use crate::{
    game::{Branch, Game},
    object::{Object, Transform},
};
use image::{imageops, ImageBuffer, Rgb, RgbImage};
use nalgebra::{RealField, Translation3, UnitQuaternion, Vector3};
use rusttype::{point, Font, Scale};

pub type Model<'a> = Vec<Object<'a>>;

pub struct GameObject<'a> {
    pub model: &'a Model<'a>,
    pub transform: Transform,
}

pub struct GameModels<'a> {
    pub log: Model<'a>,
    pub branch_log: Model<'a>,
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
