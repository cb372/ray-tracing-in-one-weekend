use crate::hittable::{Hittable, HitRecord};
use crate::vec3::Vec3;
use crate::ray::Ray;

pub struct Sphere {
    pub centre: Vec3,
    pub radius: f64
}

impl Hittable for Sphere {

    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = r.origin - self.centre;
        let a = Vec3::dot(r.direction, r.direction);
        let b = Vec3::dot(oc, r.direction);
        let c = Vec3::dot(oc, oc) - self.radius * self.radius;
        let discriminant = b*b - a*c;
        if discriminant > 0.0 {
            let temp = (-b - discriminant.sqrt()) / a;
            if temp < t_max && temp > t_min {
                let t = temp;
                let p = r.point_at_parameter(t);
                let normal = (p - self.centre) / self.radius;
                Some(HitRecord { t, p, normal })
            } else {
                let temp2 = (-b + discriminant.sqrt()) / a;
                if temp2 < t_max && temp2 > t_min {
                    let t = temp2;
                    let p = r.point_at_parameter(temp2);
                    let normal = (p - self.centre) / self.radius;
                    Some(HitRecord { t, p, normal })
                } else {
                    None
                }
            }
        } else {
            None
        }
    }
}
