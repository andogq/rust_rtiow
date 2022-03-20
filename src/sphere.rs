use crate::hittable::{
    Hittable,
    HitRecord
};
use crate::material::Material;
use glam::Vec3;

pub struct Sphere<M: Material> {
    pub center: Vec3,
    pub radius: f32,
    pub material: M
}

impl<M: Material> Hittable for Sphere<M> {
    fn hit(&self, ray: &crate::ray::Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let oc = ray.origin - self.center;

        let a = ray.direction.length_squared();
        let b = oc.dot(ray.direction);
        let c = oc.length_squared() - (self.radius * self.radius);

        let discriminant: f32 = (b * b) - (a * c);

        if discriminant < 0.0 {
            None
        } else {
            let mut root = (-b - discriminant.sqrt()) / a;

            if root < t_min || root > t_max {
                root = (-b + discriminant.sqrt()) / a;

                if root < t_min || root > t_max {
                    return None;
                }
            }

            let mut hit = HitRecord::new();
            hit.t = root;
            hit.p = ray.at(hit.t);
            hit.set_face_normal(ray, &((hit.p - self.center) / self.radius));
            hit.material = Some(&self.material);

            Some(hit)
        }
    }
}
