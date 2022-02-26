use crate::hittable::HitRecord;
use crate::ray::Ray;
use crate::vec3::Color;
use crate::vec3::Vec3;
use crate::util;

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

        let cos_theta = -unit_direction.dot(&rec.normal).min(1.0);
        let sin_theta = f64::sqrt(1.0 - cos_theta * cos_theta);
        let cannot_refract = refraction_ratio * sin_theta > 1.0;
        let mut direction = Vec3::new();
        if cannot_refract || self.reflectance(cos_theta, refraction_ratio) > util::random_double() {
            direction = Vec3::reflect(&unit_direction, &rec.normal);
        } else {
            direction = Vec3::refract(&unit_direction, &rec.normal, refraction_ratio);
        }

        *scattered = Ray::cons(&rec.p, &direction);
        return true;
    }
}

impl Dielectric {
    pub fn new(index_of_refraction: f64) -> Dielectric {
        Self { ir: index_of_refraction }
    }

    fn reflectance(&self, cosine: f64, ref_idx: f64) -> f64 {
        // Use Schlick's approximation for reflectance
        let mut r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
        r0 = r0 * r0;
        r0 + (1.0 - r0) * (1.0 - cosine).powf(5.0)
    }
}
