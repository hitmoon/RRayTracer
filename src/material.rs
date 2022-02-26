use crate::hittable::HitRecord;
use crate::ray::Ray;
use crate::vec3::Color;
use crate::vec3::Vec3;

pub trait Material {

    fn scatter(&self, r_in: &Ray, rec: &HitRecord, attenuation: &mut Color, scattered: &mut Ray) -> bool;
}

pub struct Lambertian {
    albedo: Color,
}

impl Material for Lambertian {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, attenuation: &mut Color, scattered: &mut Ray) -> bool {
        let mut scatter_direction = rec.normal + Vec3::random_unit_vector();

        // Catch degenerate scatter direction
        if scatter_direction.near_zero() {
            scatter_direction = rec.normal;
        }

        *scattered = Ray::cons(&rec.p, &scatter_direction);
        *attenuation = self.albedo;
        return true;
    }
}

impl Lambertian {
    pub fn new(a: &Color) -> Self {
        Self { albedo: *a }
    }
}

pub struct Metal {
    albedo: Color,
    fuzz: f64
}

impl Material for Metal {

    fn scatter(&self, r_in: &Ray, rec: &HitRecord, attenuation: &mut Color, scattered: &mut Ray) -> bool {
        let reflected = Vec3::reflect(&Vec3::unit_vector(&r_in.direction()), &rec.normal);
        *scattered = Ray::cons(&rec.p, &(reflected + Vec3::random_in_unit_sphere() * self.fuzz));
        *attenuation = self.albedo;
        scattered.direction().dot(&rec.normal) > 0.0
    }
}

impl Metal {
    pub fn new(a: &Color, fuzz: f64) -> Self {
        Self { albedo: *a, fuzz: if fuzz < 1.0 {fuzz} else {1.0} }
    }
}
