use crate::{
    game::{Branch, Game},
    object::{Model2Resource, ModelResource, ResourceManager},
    transform::{Transform, Transform2},
};
use image::{imageops, ImageBuffer, Rgb, RgbImage};
use nalgebra::{RealField, Translation3, UnitQuaternion, Vector3};
use rusttype::{point, Font, Scale};

pub struct GameObject {
    pub model: ModelResource,
    pub transform: Transform,
}

pub struct UIObject {
    pub model: Model2Resource,
    pub transform: Transform2,
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

pub fn make_scene(game: &Game) -> Vec<GameObject> {
    game.tree
        .clone()
        .iter()
        .enumerate()
        .map(|(i, val)| {
            let model = match val {
                Branch::None => ResourceManager::log(),
                Branch::Left | Branch::Right => ResourceManager::branch_log(),
            };
            let rotation = match val {
                Branch::Right => Some(UnitQuaternion::from_axis_angle(
                    &Vector3::<f32>::y_axis(),
                    RealField::pi(),
                )),
                Branch::Left | Branch::None => None,
            };
            let transform = Transform {
                translation: Some(Translation3::new(0., i as f32, 0.)),
                scale: None,
                rotation,
            };
            GameObject { model, transform }
        })
        .collect()
}

// pub fn make_ui(
//     game: &Game,
//     textures: &mut HashMap<String, DefaultTess>,
//     ctxt: &mut impl GraphicsContext<Backend = Backend>,
// ) -> Vec<Object2> {
//     let scale = rusttype::Scale::uniform(256.0);
//     let font_data = include_bytes!("../fonts/Courier New.ttf");
//     let font = rusttype::Font::try_from_bytes(font_data as &[u8]).expect("Constructing font");
//     let text_img = make_text_image(&"LUMBERMAN", font, scale);
//     let mut temp_txt = object::make_texture(ctxt, &text_img);
//     let quad = object::quad(0.5, 0.5).make_tess(ctxt);
// }
