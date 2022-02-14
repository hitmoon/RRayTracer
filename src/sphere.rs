use crate::vec3::Point3;
use crate::ray::Ray;
use crate::hittable::Hittable;
use crate::hittable::HitRecord;

pub struct Sphere {
    center: Point3,
    radius: f64
}

impl Sphere {

    pub fn new() -> Sphere {
        Sphere { center: Point3 { e: [0.0, 0.0, 0.0]}, radius: 0.0 }
    }

    pub fn from(cen: &Point3, r: f64) -> Sphere {
        Sphere { center: cen.clone(), radius: r }
    }
}

impl Hittable for Sphere {

    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let oc = r.origin() - self.center;
        let a = r.direction().length_squared();
        let half_b = oc.dot(&r.direction());
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;

        if discriminant < 0.0 {
            return false;
        }

        let sqrtd = f64::sqrt(discriminant);

        // Find the nearest root that lies in the acceptable range.
        let root = (-half_b - sqrtd) / a;
        if root < t_min || t_max < root {
            root = (-half_b + sqrtd) / a;
            if root < t_min || t_max < root {
                return false;
            }
        }

        rec.t = root;
        rec.p = r.at(rec.t);
        let outward_normal = (rec.p - self.center) / self.radius;
        rec.set_face_normal(r, &outward_normal);
        return true;
    }
}
