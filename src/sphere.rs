use crate::hittable::{
    Hittable,
    HitRecord
};
use glam::Vec3;

pub struct Sphere {
    pub center: Vec3,
    pub radius: f32,
}

impl Hittable for Sphere {
    fn hit(&self, ray: &crate::ray::Ray, t_min: f32, t_max: f32, record: &mut HitRecord) -> bool {
        let oc = ray.origin - self.center.clone();

        let a = ray.direction.length_squared();
        let b = oc.dot(ray.direction);
        let c = oc.length_squared() - (self.radius * self.radius);

        let discriminant: f32 = (b * b) - (a * c);

        if discriminant < 0.0 {
            return false;
        } else {
            let mut root = (-b - discriminant.sqrt()) / a;

            if root < t_min || root > t_max {
                root = (-b + discriminant.sqrt()) / a;

                if root < t_min || root > t_max {
                    return false;
                }
            }

            record.t = root;
            record.p = ray.at(record.t);
            record.set_face_normal(ray, &((record.p - self.center) / self.radius));

            return true;
        }
    }
}
