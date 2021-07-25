use image::{imageops, DynamicImage, GenericImage, Rgb, Rgba, RgbaImage};
use rusttype::{point, Font, Scale};

const FONT_DATA: &[u8] = include_bytes!("../fonts/Courier New.ttf");

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
    let font = rusttype::Font::try_from_bytes(FONT_DATA as &[u8]).expect("Constructing font");
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
    let mut image = DynamicImage::new_rgba8(glyphs_width + 40, glyphs_height + 40);

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
    imageops::flip_vertical(&image)
}

pub fn make_text(text: &str) -> RgbaImage {
    let scale = rusttype::Scale::uniform(256.0);
    let font = rusttype::Font::try_from_bytes(FONT_DATA as &[u8]).expect("Constructing font");
    make_text_image(text, font, scale, Rgb([150, 0, 0]))
}
