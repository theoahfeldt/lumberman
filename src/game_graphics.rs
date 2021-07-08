use crate::{
    game::{Branch, Game},
    geometry,
    object::{Model2Resource, ModelResource, Object, Object2, ResourceManager, TextureResource},
    transform::{Transform, Transform2},
};
use image::{imageops, ImageBuffer, Rgb, RgbImage};
use luminance::context::GraphicsContext;
use luminance_front::Backend;
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

pub struct GameResources {
    pub log: ModelResource,
    pub branch_log: ModelResource,
}

pub struct UIResources {
    pub score_texture: TextureResource,
    pub score_model: Model2Resource,
}

impl UIResources {
    pub fn new(
        rm: &mut ResourceManager,
        ctxt: &mut impl GraphicsContext<Backend = Backend>,
    ) -> Self {
        let score_img = make_text("0");
        let quad = geometry::quad(0.5, 0.5);

        let tess = rm.make_tess(ctxt, quad);
        let score_texture = rm.make_texture(ctxt, &score_img);
        let transform = Transform2::new();
        let score_object = Object2 {
            tess,
            texture: score_texture.clone(),
            transform,
        };
        let score_model = rm.make_model2(vec![score_object]);
        Self {
            score_texture,
            score_model,
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
            .into_rgb8();
        let bark = rm.make_texture(ctxt, &bark_img);

        let angle: f32 = RealField::frac_pi_2();
        let log = Object {
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
        let branch = Object {
            tess: cylinder,
            texture: bark,
            transform: Transform {
                translation: Some(Translation3::new(0.9, 0., 0.)),
                scale: Some([0.2, 0.2, 1.]),
                rotation: Some(UnitQuaternion::from_axis_angle(
                    &Vector3::<f32>::y_axis(),
                    RealField::frac_pi_2(),
                )),
            },
        };
        let log2 = log.clone();
        let log = rm.make_model(vec![log]);
        let branch_log = rm.make_model(vec![log2, branch]);
        Self { log, branch_log }
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
                    x + bounding_box.min.x as u32 - 20,
                    y + bounding_box.min.y as u32,
                    // Turn the coverage into an alpha value
                    Rgb([color_vec.x as u8, color_vec.y as u8, color_vec.z as u8]),
                )
            });
        }
    }

    imageops::flip_vertical(&image)
}

pub fn make_text(text: &str) -> RgbImage {
    let scale = rusttype::Scale::uniform(256.0);
    let font_data = include_bytes!("../fonts/Courier New.ttf");
    let font = rusttype::Font::try_from_bytes(font_data as &[u8]).expect("Constructing font");
    make_text_image(text, font, scale)
}

pub fn make_scene(game: &Game, resources: &GameResources) -> Vec<GameObject> {
    game.tree
        .clone()
        .iter()
        .enumerate()
        .map(|(i, val)| {
            let model = match val {
                Branch::None => resources.log.clone(),
                Branch::Left | Branch::Right => resources.branch_log.clone(),
            };
            let rotation = match val {
                Branch::Left => Some(UnitQuaternion::from_axis_angle(
                    &Vector3::<f32>::y_axis(),
                    RealField::pi(),
                )),
                Branch::Right | Branch::None => None,
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

pub fn update_ui_resources(
    game: &Game,
    resources: &UIResources,
    rm: &mut ResourceManager,
    ctxt: &mut impl GraphicsContext<Backend = Backend>,
) {
    let score = game.get_score();
    let score_img = make_text(score.to_string().as_str());

    rm.update_texture(resources.score_texture.clone(), ctxt, &score_img);
}

pub fn make_ui(resources: &UIResources) -> Vec<UIObject> {
    let model = resources.score_model.clone();
    let transform = Transform2 {
        scale: None,
        rotation: None,
        translation: Some(Translation3::new(0.75, 0.75, 0.)),
    };
    vec![UIObject { model, transform }]
}
