use crate::vec3::Point3;
use crate::ray::Ray;

pub fn hit_sphere(center: &Point3, radius: f64, r: &Ray) -> f64 {
    let oc = r.origin() - *center;
    let a = r.direction().length_squared();
    let half_b = oc.dot(&r.direction());
    let c = oc.length_squared() - radius * radius;
    let discriminant = half_b * half_b - a * c;
    if discriminant < 0.0 {
        return -1.0;
    }
    (-half_b - f64::sqrt(discriminant)) / a
}
