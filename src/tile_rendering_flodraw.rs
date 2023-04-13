// use flo_draw::*;
// use flo_draw::canvas::*;
// use flo_draw::draw::*;
// use image::{ImageBuffer, Rgba};
//
// fn main() {
//     // Create a 256x256 canvas
//     let mut canvas = create_canvas(256, 256);
//
//     // Create a simple drawing
//     let mut drawing = create_drawing();
//
//     // Add some text to the drawing
//     let font_id = drawing.create_font_data(include_bytes!("path/to/your/font.ttf"));
//     let layout = drawing.layout_text(font_id, "Hello, World!", 32.0, TextStyle::Emphasized);
//     drawing.draw_text(FontId(font_id), &layout, (128.0, 128.0));
//
//     // Render the drawing to the canvas
//     render_drawing(&mut canvas, &drawing);
//
//     // Convert the canvas to an image buffer
//     let image_buffer: ImageBuffer<Rgba<u8>, _> = canvas.into_image_buffer();
//
//     // Save the image buffer to a PNG file
//     image_buffer.save("output.png").unwrap();
// }
//
// fn create_canvas(width: usize, height: usize) -> Canvas {
//     let mut canvas = Canvas::new(width, height);
//     canvas.set_color(Color::Rgba(1.0, 1.0, 1.0, 1.0));
//     canvas.fill();
//     canvas.set_color(Color::Rgba(0.0, 0.0, 0.0, 1.0));
//     canvas
// }
//
// fn create_drawing() -> Drawing {
//     let mut drawing = Drawing::new();
//     drawing.set_color(Color::Rgba(0.0, 0.0, 0.0, 1.0));
//     drawing
// }