fn main() {
    // Image specs
    let image_width = 256;
    let image_height = 256;

    // Render image
    println!("P3\n{image_width} {image_height} \n255");

    for pixel_h in 0..image_height {
        for pixel_w in 0..image_width {
            let r: f64 = pixel_w as f64 / (image_width - 1) as f64;
            let g: f64 = pixel_h as f64 / (image_height - 1) as f64;
            let b: f64 = 0.0;

            let ir = (255.99 * r) as i32;
            let ig = (255.99 * g) as i32;
            let ib = (255.99 * b) as i32;

            println!("{ir} {ig} {ib}");
        }
    }
}
