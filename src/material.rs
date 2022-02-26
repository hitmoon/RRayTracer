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

pub struct Dielectric {
    ir: f64
}

impl Material for Dielectric {

    fn scatter(&self, r_in: &Ray, rec: &HitRecord, attenuation: &mut Color, scattered: &mut Ray) -> bool {
        *attenuation = Color::from(1.0, 1.0, 1.0);
        let refraction_ratio = if rec.front_face { 1.0 / self.ir } else {self.ir};
        let unit_direction = r_in.direction().unit_vector();
        let refracted = Vec3::refract(&unit_direction, &rec.normal, refraction_ratio);
        *scattered = Ray::cons(&rec.p, &refracted);
        return true;
    }
}

impl Dielectric {
    pub fn new(index_of_refraction: f64) -> Dielectric {
        Self { ir: index_of_refraction }
    }
}
