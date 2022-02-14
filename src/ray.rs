use crate::vec3::Point3;
use crate::vec3;
use crate::vec3::Color;
use crate::sphere;

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

    pub fn ray_color(&self) -> Color {

        let center = Point3 { e: [0.0, 0.0, -1.0] };

        let t = sphere::hit_sphere(&center, 0.5, &self);
        if t > 0.0 {
            let n = (self.at(t) - center).unit_vector();
            let c = Color { e: [n.x() + 1.0, n.y() + 1.0, n.z() + 1.0] };
            return c * 0.5;
        }

        let color = Color { e: [1.0, 1.0, 1.0] };
        let color2 = Color { e: [0.5, 0.7, 1.0]};
        let unit_direction = self.direction().unit_vector();
        let t = 0.5 * (unit_direction.y() + 1.0);
        color * (1.0 - t) +  color2 * t
    }
}