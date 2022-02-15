use crate::hittable::Hittable;
use crate::hittable::HitRecord;
use crate::ray::Ray;

pub struct World {
    objects: Vec<Box<dyn Hittable>>
}

impl Hittable for World {

    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let mut temp_rec: HitRecord;
        let hit_anything = false;
        let closest_so_far = t_max;

        for obj in self.objects.iter() {
            if obj.hit(r, t_min, closest_so_far, &mut temp_rec) {
                hit_anything = true;
                closest_so_far = temp_rec.t;
                *rec = temp_rec;
            }
        }

        return hit_anything;
    }
}

impl World {

    pub fn new() -> World {
        World { objects: Vec::new() }
    }

    pub fn add<T: 'static + Hittable> (&self, object: T) {
        self.objects.push(Box::new(object))
    }

    pub fn clear(&self) {
        self.objects.clear();
    }
}
