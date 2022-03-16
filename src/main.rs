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

fn random_scene() -> World {

    let mut world = World::new();
    let ground = Rc::new(Lambertian::new(&Color::from(0.5, 0.5, 0.5)));
    world.add(Box::new(Sphere::from(&Point3::from(0.0, -1000.0, 0.0), 1000.0, ground)));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = util::random_double();
            let center = Point3::from(a as f64 + 0.9 * util::random_double(),
                                      0.2,
                                      b as f64 + 0.9 * util::random_double());
            if (center - Point3::from(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.8 {
                    // diffuse
                    let albedo = Color::random() * Color::random();
                    let material = Rc::new(Lambertian::new(&albedo));
                    world.add(Box::new(Sphere::from(&center, 0.2, material.clone())));
                } else if choose_mat < 0.95 {
                    // metal
                    let albedo = Color::random_range(0.5, 1.0);
                    let fuzz = util::random_double_range(0.0, 0.5);
                    let material = Rc::new(Metal::new(&albedo, fuzz));
                    world.add(Box::new(Sphere::from(&center, 0.2, material.clone())));
                } else {
                    // glass
                    let material = Rc::new(Dielectric::new(1.5));
                    world.add(Box::new(Sphere::from(&center, 0.2, material.clone())));
                }
            }
        }
    }

    let mat1 = Rc::new(Dielectric::new(1.5));
    world.add(Box::new(Sphere::from(&Point3::from(0.0, 1.0, 0.0), 1.0, mat1.clone())));

    let mat2 = Rc::new(Lambertian::new(&Color::from(0.4, 0.2, 0.1)));
    world.add(Box::new(Sphere::from(&Point3::from(-4.0, 1.0, 0.0), 1.0, mat2.clone())));

    let mat3 = Rc::new(Metal::new(&Color::from(0.7, 0.6, 0.5), 0.0));
    world.add(Box::new(Sphere::from(&Point3::from(4.0, 1.0, 0.0), 1.0, mat3.clone())));

    world
}

fn main() {
    // Image
    let aspect_ratio = 3.0 / 2.0;
    let image_width = 1200;
    let image_height = (image_width as f64 / aspect_ratio) as i32;
    let samples_per_pixel: i32 = 500;
    let max_depth = 50;

    // World
    let world = random_scene();

    // Camera
    let lookfrom = Point3::from(13.0, 2.0, 3.0);
    let lookat = Point3::from(0.0, 0.0, 0.0);
    let vup = Vec3::from(0.0, 1.0, 0.0);
    let dist_to_focus = 10.0;
    let aperture = 0.1;
    let cam = Camera::new(&lookfrom, &lookat, &vup, 20.0, aspect_ratio, aperture, dist_to_focus);

    // Render

    print!("P3\n{} {}\n255\n", image_width, image_height);

    for j in (0..image_height).rev() {
        eprint!("Scanlines remaining: {:<8}\r", j);
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
