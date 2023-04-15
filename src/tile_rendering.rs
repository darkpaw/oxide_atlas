use std::io::{Cursor};
use image::{ImageBuffer, ImageEncoder, Rgba, ColorType};
use image::codecs::png::PngEncoder;
use rusttype::{Font, Scale, point};

use crate::matrixset_loader::TileMatrixDef;

pub(crate) fn test_tile_rendering() {

    let text = "Hello worldy !";
    let font_data = include_bytes!("DejaVuSansCondensed.ttf");
    let font = Font::try_from_bytes(font_data as &[u8]).unwrap();

    let scale = Scale::uniform(36.0);
    let mut offset = point(10.0, 40.0);

    let mut image = ImageBuffer::new(256, 256);

    // Clear the image with a white background
    for pixel in image.pixels_mut() {
        *pixel = Rgba([0, 0, 0, 0]);
    }

    let red = Rgba([255, 0, 0, 255]);

    // Draw a red border
    for x in 0..256 {
        image.put_pixel(x, 0, red);
    }

    for y in 0..256 {
        image.put_pixel(0, y, red);
    }

    // Render the text
    for c in text.chars() {

        let glyph = font.glyph(c).scaled(scale).positioned(offset);

        if let Some(bb) = glyph.pixel_bounding_box() {
            glyph.draw(|x, y, v| {
                let x = x as i32 + bb.min.x;
                let y = y as i32 + bb.min.y;

                if x >= 0 && x < 256 && y >= 0 && y < 256 {
                    let pixel = image.get_pixel_mut(x as u32, y as u32);
                    let alpha = (v * 255.0) as u8;
                    *pixel = Rgba([220, 112, 55, alpha]);
                }
            });
        }

        offset.x += glyph.unpositioned().h_metrics().advance_width;
    }

    // Save the image as a PNG file
    image.save("output.png").unwrap();
}

pub(crate) fn convert_to_png_bytes(img_buffer: &ImageBuffer<Rgba<u8>, Vec<u8>>) -> Vec<u8> {

    let mut png_bytes = Cursor::new(Vec::new());
    let encoder = PngEncoder::new(&mut png_bytes);

    encoder.write_image(
        img_buffer.as_raw(),
        img_buffer.width(),
        img_buffer.height(),
        ColorType::Rgba8,
    ).expect("PNG encode failed.");

    png_bytes.into_inner()

}

pub(crate) fn render_debug_tile(
    tile_matrix: &TileMatrixDef,
    tile_x: u32,
    tile_y: u32,
) -> ImageBuffer<Rgba<u8>, Vec<u8>> {

    let text = format!("{} {} {}", tile_x, tile_y, tile_matrix.identifier);
    let font_data = include_bytes!("DejaVuSansCondensed.ttf");
    let font = Font::try_from_bytes(font_data as &[u8]).unwrap();

    let scale = Scale::uniform(36.0);
    let mut offset = point(10.0, 40.0);

    let mut image = ImageBuffer::new(256, 256);

    // Clear the image with a white background
    for pixel in image.pixels_mut() {
        *pixel = Rgba([0, 0, 0, 0]);
    }

    let red = Rgba([255, 20, 33, 128]);

    // Draw a red border
    for x in 0..256 {
        image.put_pixel(x, 0, red);
    }

    for y in 0..256 {
        image.put_pixel(0, y, red);
    }

    // Render the text
    for c in text.chars() {

        let glyph = font.glyph(c).scaled(scale).positioned(offset);

        if let Some(bb) = glyph.pixel_bounding_box() {
            glyph.draw(|x, y, v| {
                let x = x as i32 + bb.min.x;
                let y = y as i32 + bb.min.y;

                if x >= 0 && x < 256 && y >= 0 && y < 256 {
                    let pixel = image.get_pixel_mut(x as u32, y as u32);
                    let alpha = (v * 255.0) as u8;
                    *pixel = Rgba([220, 112, 55, alpha]);
                }
            });
        }

        offset.x += glyph.unpositioned().h_metrics().advance_width;
    }

    // Return the image buffer.
    image
}

