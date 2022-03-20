use crate::ray::Ray;
use glam::Vec3;
use crate::material::Material;

#[derive(Clone)]
pub struct HitRecord<'a> {
    pub p: Vec3,
    pub normal: Vec3,
    pub t: f32,
    pub front_face: bool,
    pub material: Option<&'a dyn Material>
}

impl<'a> HitRecord<'a> {
    pub fn new() -> HitRecord<'a> {
        HitRecord {
            p: Vec3::new(0.0, 0.0, 0.0),
            normal: Vec3::new(0.0, 0.0, 0.0),
            t: 0.0,
            front_face: false,
            material: None
        }
    }

    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: &Vec3) {
        self.front_face = ray.direction.dot(*outward_normal) < 0.0;
        self.normal = if self.front_face { *outward_normal } else { -(*outward_normal) };
    }
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
}

pub struct HittableList {
    objects: Vec<Box<dyn Hittable>>
}

impl HittableList {
    pub fn new() -> HittableList {
        HittableList {
            objects: Vec::new()
        }
    }

    pub fn add(&mut self, object: Box<dyn Hittable>) {
        self.objects.push(object);
    }
}

impl Hittable for HittableList {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let mut hit_anything = None;
        let mut closest_t = t_max;

        for object in self.objects.iter() {
            if let Some(hit) = object.hit(ray, t_min, closest_t) {
                closest_t = hit.t;
                hit_anything = Some(hit);
            }
        }

        hit_anything
    }
}
