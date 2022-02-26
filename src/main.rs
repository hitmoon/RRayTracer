use std::io::{self, Write};
use vec3::Point3;
use vec3::Vec3;
use sphere::Sphere;
use world::World;
use camera::Camera;
use vec3::Color;
use material::Lambertian;
use material::Metal;
use material::Dielectric;
use std::rc::Rc;
use std::f64;
mod vec3;
mod color;
mod ray;
mod sphere;
mod hittable;
mod world;
mod util;
mod camera;
mod material;

fn main() {
    // Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = (image_width as f64 / aspect_ratio) as i32;
    let samples_per_pixel: i32 = 100;
    let max_depth = 50;

    // World
    let R = f64::cos(f64::consts::PI / 4.0);
    let mut world = World::new();
    /*
    let left = Rc::new(Lambertian::new(&Color::from(0.0, 0.0, 1.0)));
    let right = Rc::new(Lambertian::new(&Color::from(1.0, 0.0, 0.0)));

    world.add(Box::new(Sphere::from(&Point3::from(-R, 0.0, -1.0), R, left.clone())));
    world.add(Box::new(Sphere::from(&Point3::from(R, 0.0, -1.0), R, right.clone())));
    */
    let ground = Rc::new(Lambertian::new(&Color::from(0.8, 0.8, 0.0)));
    let center = Rc::new(Lambertian::new(&Color::from(0.1, 0.2, 0.5)));
    let left = Rc::new(Dielectric::new(1.5));
    let right = Rc::new(Metal::new(&Color::from(0.8, 0.6, 0.2), 0.0));

    world.add(Box::new(Sphere::from(&Point3::from(0.0, -100.5, -1.0), 100.0, ground)));
    world.add(Box::new(Sphere::from(&Point3::from(0.0, 0.0, -1.0), 0.5, center)));
    world.add(Box::new(Sphere::from(&Point3::from(-1.0, 0.0, -1.0), 0.5, left.clone())));
    world.add(Box::new(Sphere::from(&Point3::from(-1.0, 0.0, -1.0), -0.45, left.clone())));
    world.add(Box::new(Sphere::from(&Point3::from(1.0, 0.0, -1.0), 0.5, right)));

    // Camera
    let cam = Camera::new(&Point3::from(-2.0, 2.0, 1.0),
                          &Point3::from(0.0, 0.0,-1.0), &Vec3::from(0.0, 1.0, 0.0),
                          90.0, aspect_ratio);

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
