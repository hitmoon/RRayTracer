use crate::vec3::Point3;
use crate::vec3;
use crate::vec3::Color;
use crate::hittable::Hittable;
use crate::hittable::HitRecord;

pub struct Ray {

    /// Origin point
    pub orig: Point3,
    /// direction
    pub dir: vec3::Vec3,

}


impl Ray {

    pub fn new() -> Self {
        Ray { orig: Point3::new(), dir: vec3::Vec3::new() }
    }

    pub fn cons(origin: &Point3, direction: &vec3::Vec3) -> Self {

        Ray { orig: *origin, dir: *direction }
    }


    pub fn origin(&self) -> Point3 {
        self.orig
    }

    pub fn direction(&self) -> vec3::Vec3 {
        self.dir
    }

    pub fn at(&self, t: f64) -> Point3 {
        self.orig + self.dir * t
    }

    pub fn ray_color(&self, world: &dyn Hittable, depth: i32) -> Color {
        let mut rec = HitRecord::new();

        // If we've exceeded the ray bounce limit, no more light is gathered.
        if depth <= 0 {
            return Color::new();
        }

        if world.hit(self, 0.001, f64::INFINITY, &mut rec) {
            let target = rec.p + rec.normal + vec3::Vec3::random_unit_vector();
            let ray = Ray::cons(&rec.p, &(target - rec.p));
            return ray.ray_color(world, depth - 1) * 0.5;
        }

        let color = Color { e: [1.0, 1.0, 1.0] };
        let color2 = Color { e: [0.5, 0.7, 1.0]};
        let unit_direction = self.direction().unit_vector();
        let t = 0.5 * (unit_direction.y() + 1.0);
        color * (1.0 - t) +  color2 * t
    }
}
