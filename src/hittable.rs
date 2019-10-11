use crate::vec3::Vec3;
use crate::ray::Ray;

#[derive(Copy, Clone)]
pub struct HitRecord {
    pub t: f64,
    pub p: Vec3,
    pub normal: Vec3
}

pub trait Hittable {

    /// Calculates whether the given ray hits the object within the given range of t.
    ///
    /// If there is a hit, the corresponding `HitRecord` is returned.
    ///
    /// If there is no hit, the result is `None`.
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;

}

impl <T> Hittable for Vec<T> where T: Hittable {

    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut closest_hit_record: Option<HitRecord> = None;
        let mut closest_t = t_max;
        for element in self.iter() {
            let hit_record = element.hit(r, t_min, closest_t);
            if hit_record.is_some() {
                closest_hit_record = hit_record;
                closest_t = hit_record.unwrap().t;
            }
        }
        return closest_hit_record;
    }

}
