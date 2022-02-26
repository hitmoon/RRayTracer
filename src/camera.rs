use crate::vec3::Point3;
use crate::ray::Ray;
use crate::vec3::Vec3;
use crate::util;

pub struct Camera {
    origin: Point3,
    lower_left_corner: Point3,
    horizontal: Vec3,
    vertical: Vec3
}

impl Camera {

    pub fn new(vfov: f64, aspect_ratio: f64) -> Camera {
        let theta = util::degrees_to_radians(vfov);
        let h = f64::tan(theta / 2.0);
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;
        let focal_length = 1.0;

        let origin = Point3 { e: [0.0, 0.0, 0.0] };
        let horizontal = Vec3 { e: [viewport_width, 0.0, 0.0] };
        let vertical = Vec3 { e: [0.0, viewport_height, 0.0] };
        let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - Vec3 { e: [0.0, 0.0, focal_length] };
        Camera { origin, lower_left_corner, horizontal, vertical }
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        Ray::cons(&self.origin,
            &(self.lower_left_corner + self.horizontal * u + self.vertical * v - self.origin))
    }
}
