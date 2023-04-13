
use image::{ImageBuffer, Rgba};
use rusttype::{Font, Scale, point};

pub(crate) fn test_tile_rendering() {
    let text = "Hello worldy !";
    let font_data = include_bytes!("DejaVuSansCondensed.ttf");
    let font = Font::try_from_bytes(font_data as &[u8]).unwrap();

    let scale = Scale::uniform(36.0);
    let mut offset = point(10.0, 40.0);

    let mut image = ImageBuffer::new(256, 256);

    // Clear the image with a white background
    for pixel in image.pixels_mut() {
        *pixel = Rgba([30, 30, 30, 0]);
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
