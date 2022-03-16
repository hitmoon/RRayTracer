use std::io::Write;
use vec3::Point3;
use vec3::Vec3;
use sphere::Sphere;
use world::World;
use camera::Camera;
use vec3::Color;
use material::Lambertian;
use material::Metal;
use material::Dielectric;
use std::sync::{Arc,Mutex};
use std::f64;
use std::env;
use std::fs::File;
use std::path::Path;
use rayon::prelude::*;

mod vec3;
mod color;
mod ray;
mod sphere;
mod hittable;
mod world;
mod util;
mod camera;
mod material;

struct ImageConfig {
    world: Arc<World>,
    cam: Arc<Camera>,
    width: i32,
    height: i32,
    max_depth: i32,
    samples_per_pixel: i32
}

fn random_scene() -> World {

    let mut world = World::new();
    let ground = Arc::new(Lambertian::new(&Color::from(0.5, 0.5, 0.5)));
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
                    let material = Arc::new(Lambertian::new(&albedo));
                    world.add(Box::new(Sphere::from(&center, 0.2, material.clone())));
                } else if choose_mat < 0.95 {
                    // metal
                    let albedo = Color::random_range(0.5, 1.0);
                    let fuzz = util::random_double_range(0.0, 0.5);
                    let material = Arc::new(Metal::new(&albedo, fuzz));
                    world.add(Box::new(Sphere::from(&center, 0.2, material.clone())));
                } else {
                    // glass
                    let material = Arc::new(Dielectric::new(1.5));
                    world.add(Box::new(Sphere::from(&center, 0.2, material.clone())));
                }
            }
        }
    }

    let mat1 = Arc::new(Dielectric::new(1.5));
    world.add(Box::new(Sphere::from(&Point3::from(0.0, 1.0, 0.0), 1.0, mat1.clone())));

    let mat2 = Arc::new(Lambertian::new(&Color::from(0.4, 0.2, 0.1)));
    world.add(Box::new(Sphere::from(&Point3::from(-4.0, 1.0, 0.0), 1.0, mat2.clone())));

    let mat3 = Arc::new(Metal::new(&Color::from(0.7, 0.6, 0.5), 0.0));
    world.add(Box::new(Sphere::from(&Point3::from(4.0, 1.0, 0.0), 1.0, mat3.clone())));

    world
}

fn image_write_row(row: i32, config: &ImageConfig, buf: Arc<Mutex<&mut Vec<Vec<u8>>>>) -> Result<(), std::io::Error> {

    let image_width = config.width;
    let image_height = config.height;
    let cam = &(*config.cam);
    let world = &(*config.world);
    let max_depth = config.max_depth;
    let samples_per_pixel = config.samples_per_pixel;

    let j = image_height - 1 - row;

    let mut tmp = vec![0_u8; (image_width * 3).try_into().unwrap()];

    for i in 0..image_width {
        let mut pixel_color = Color::new();

        for _s in 0..samples_per_pixel {
            let u = (i as f64 + util::random_double()) / (image_width - 1) as f64;
            let v = (j as f64 + util::random_double()) / (image_height - 1) as f64;
            let r = cam.get_ray(u, v);
            pixel_color = pixel_color + r.ray_color(world, max_depth);
        }

        color::write_color(&mut tmp, (i as usize).try_into().unwrap(), pixel_color, samples_per_pixel).unwrap();
    }

    // copy the row data
    let mut b = buf.lock().unwrap();
    b[row as usize] = tmp;

    Ok(())
}

fn main() {

    let v: Vec<String> = env::args().collect();
    if v.len() < 2 {
        eprint!("Usage: {} <ppm image file>\n", v[0]);
        std::process::exit(0);
    }

    print!("Output file: {}\n", v[1]);
    let path = Path::new(&v[1]);
    let mut file = match File::create(&path) {
        Err(why) => panic!("couldn't open {}: {}", path.display(), why),
        Ok(file) => file,
    };

    // Image
    let aspect_ratio = 3.0 / 2.0;
    let image_width = 1200;
    let image_height = (image_width as f64 / aspect_ratio) as i32;
    let samples_per_pixel: i32 = 500;
    let max_depth = 50;

    // World
    let world = Arc::new(random_scene());

    // Camera
    let lookfrom = Point3::from(13.0, 2.0, 3.0);
    let lookat = Point3::from(0.0, 0.0, 0.0);
    let vup = Vec3::from(0.0, 1.0, 0.0);
    let dist_to_focus = 10.0;
    let aperture = 0.1;
    let cam = Arc::new(Camera::new(&lookfrom, &lookat, &vup, 20.0, aspect_ratio, aperture, dist_to_focus));

    let cfg = ImageConfig { world: world.clone(), cam: cam.clone(), width: image_width, height: image_height, max_depth, samples_per_pixel };

    let mut img_buf: Vec<Vec<u8>> = vec![vec![]; image_height as usize];

    // Render
    file.write(format!("P6\n{} {}\n255\n", image_width, image_height).as_bytes()).unwrap();

    let b = Arc::new(Mutex::new(&mut img_buf));
    eprint!("Generating with multi core, please wait ...\n");
    eprint!("Total row: {}\nFinished: ", image_height);
    (0..image_height).into_par_iter()
        .for_each(|idx| {
            match image_write_row(idx, &cfg, b.clone()) {
                Ok(()) => (),
                Err(e) => panic!("write row error {}", e),
            }
            eprint!(".");
         });

    // write each row data
    for row in img_buf {
        for idx in 0..image_width {
            let s = (idx * 3) as usize;
            let e = s + 3;
            file.write(&row[s..e]).unwrap();
        }
    }
    eprint!("\nDone.\n");
}
