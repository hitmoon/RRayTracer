use crate::hittable::Hittable;
use crate::hittable::HitRecord;
use crate::ray::Ray;

pub struct World {
    objects: Vec<Box<dyn Hittable>>
}

impl World {

    pub fn new() -> World {
        World { objects: Vec::new() }
    }

    pub fn add (&mut self, object: Box<dyn Hittable>) {
        self.objects.push(object);
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }

    pub fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> (bool, Option<HitRecord>) {
        let mut temp_rec = None;
        let mut hit_anything = false;
        let mut closest_so_far = t_max;

        for obj in self.objects.iter() {
            let (hitted, rec) = obj.hit(r, t_min, closest_so_far);
            if hitted {
                hit_anything = true;
                closest_so_far = match &rec {
                    Some(rec) => rec.t,
                    None => panic!("should have a value"),
                };
                temp_rec = rec;
            }
        }

        (hit_anything, temp_rec)
    }
}
