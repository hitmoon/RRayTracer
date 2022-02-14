use std::io::{self, Write};
use vec3::Point3;
use ray::Ray;
mod vec3;
mod color;
mod ray;
mod sphere;


fn main() {
    // Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = (image_width as f64 / aspect_ratio) as i32;


    // Camera
    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length: f64 = 1.0;

    let origin = Point3::new();
    let horizontal = vec3::Vec3 { e: [viewport_width as f64, 0.0, 0.0] };
    let vertical = vec3::Vec3 { e: [0.0, viewport_height as f64, 0.0] };
    let lower_left_corner = origin - horizontal / 2f64 - vertical / 2f64 - vec3::Vec3{ e: [0.0, 0.0, focal_length] }; 

    // Render

    print!("P3\n{} {}\n255\n", image_width, image_height);

    for j in (0..image_height).rev() {
        eprint!("\rScanlines remaining: {}", j);
        io::stderr().flush().unwrap();
        for i in 0..image_width {
            let u = i as f64 / (image_width - 1) as f64;
            let v = j as f64 / (image_height - 1) as f64;
            let r = Ray::cons(&origin, &(lower_left_corner + horizontal * u + vertical * v - origin));

            let pixel_color = r.ray_color();
            color::write_color(&mut std::io::stdout(), pixel_color).unwrap();
        }
    }

    eprint!("\nDone.\n");
}
