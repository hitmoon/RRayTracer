use crate::ray::Ray;
use crate::vec3::Point3;
use crate::vec3::Vec3;

#[derive(Copy, Clone)]
pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub t: f64,
    front_face: bool
}

impl HitRecord {
    pub fn new() -> HitRecord {
        HitRecord { p: Point3 { e: [0.0, 0.0, 0.0] }, normal: Vec3 { e: [0.0, 0.0, 0.0 ] }, t: 0.0, front_face: true }
    }

    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: &Vec3) {
        self.front_face = r.direction().dot(outward_normal) < 0.0;
        self.normal = if self.front_face { *outward_normal } else { -*outward_normal };
    }
}

pub trait Hittable {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool;
}
