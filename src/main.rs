use std::io::{self, Write};
use vec3::Point3;
use sphere::Sphere;
use world::World;
use camera::Camera;
use vec3::Color;
mod vec3;
mod color;
mod ray;
mod sphere;
mod hittable;
mod world;
mod util;
mod camera;

fn main() {
    // Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = (image_width as f64 / aspect_ratio) as i32;
    let samples_per_pixel: i32 = 100;
    let max_depth = 50;

    // World
    let mut world = World::new();
    world.add(Box::new(Sphere::from(&Point3 {e: [0.0, 0.0, -1.0] }, 0.5)));
    world.add(Box::new(Sphere::from(&Point3 {e: [0.0, -100.5, -1.0] }, 100.0)));

    // Camera
    let cam = Camera::new();

    // Render

    print!("P3\n{} {}\n255\n", image_width, image_height);

    for j in (0..image_height).rev() {
        eprint!("\rScanlines remaining: {}", j);
        io::stderr().flush().unwrap();
        for i in 0..image_width {

            let mut pixel_color = Color::new();

            for _s in 0..samples_per_pixel {
                let u = (i as f64 + util::random_double()) / (image_width - 1) as f64;
                let v = (j as f64 + util::random_double()) / (image_height - 1) as f64;
                let r = cam.get_ray(u, v);
                pixel_color = pixel_color + r.ray_color(&world, max_depth);
            }
            color::write_color(&mut std::io::stdout(), pixel_color, samples_per_pixel).unwrap();
        }
    }

    eprint!("\nDone.\n");
}
