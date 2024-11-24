mod vec3;
mod color;
use std::io::Write;
use crate::vec3::Vec3;
use crate::color::Color;

fn main() {
    // Image specs
    let image_width = 256;
    let image_height = 256;

    // Render image
    println!("P3\n{image_width} {image_height} \n255");

    for pixel_h in 0..image_height {
        eprint!("\rScanlines remaining: {} ", image_height - pixel_h);
        let _ = std::io::stderr().flush();

        for pixel_w in 0..image_width {
            let pixel_color = Color {e: [pixel_w as f64 / (image_width  - 1) as f64,
                                         pixel_h as f64 / (image_height - 1) as f64, 
                                         0.0]};
            Color::write_color(pixel_color);
        }
    }
    eprintln!("\rDone                    ");
}
