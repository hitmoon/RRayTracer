use std::io::{self, Write};
use vec3::Color;
mod vec3;
mod color;


fn main() {
    // Image
    let image_width = 256;
    let image_height = 256;

    // Render

    print!("P3\n{} {}\n255\n", image_width, image_height);

    for j in (0..image_height).rev() {
        eprint!("\rScanlines remaining: {}", j);
        io::stderr().flush().unwrap();
        for i in 0..image_width {
            let r = i as f64 / (image_width - 1) as f64;
            let g = j as f64 / (image_height - 1) as f64;
            let b = 0.25;
            let pixel_color = Color { e: [r, g, b] };
            color::write_color(&mut std::io::stdout(), pixel_color).unwrap();
        }
    }

    eprint!("\nDone.\n");
}
