use crate::{random_unit_vector, near_zero, reflect, refract, random_in_unit_sphere};
use crate::ray::Ray;
use crate::hittable::HitRecord;
use glam::Vec3;
use rand::random;

pub trait Material {
    fn scatter(&self, ray: &Ray, record: &HitRecord) -> Option<(Ray, Vec3)>;
}

#[derive(Clone)]
pub struct Lambertian {
    pub albedo: Vec3
}

impl Material for Lambertian {
    fn scatter(&self, _ray: &Ray, record: &HitRecord) -> Option<(Ray, Vec3)> {
        let mut direction = record.normal + random_unit_vector();

        if near_zero(&direction) {
            direction = record.normal;
        }

        Some((
            Ray {
                origin: record.p,
                direction
            },
            self.albedo
        ))
    }
}

#[derive(Clone)]
pub struct Metal {
    pub albedo: Vec3,
    pub fuzz: f32
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, record: &HitRecord) -> Option<(Ray, Vec3)> {
        let reflected = reflect(&ray.direction.normalize(), &record.normal);

        Some((
            Ray {
                origin: record.p,
                direction: reflected + (self.fuzz * random_in_unit_sphere())
            },
            self.albedo
        ))
    }
}

#[derive(Clone)]
pub struct Dielectric {
    pub refraction_index: f32
}

impl Dielectric {
    fn reflectance(cosine: f32, refractive_index: f32) -> f32 {
        let mut r0 = (1.0 - refractive_index) / (1.0 + refractive_index);
        r0 = r0 * r0;
        r0 + ((1.0 - r0) * (1.0 - cosine).powi(5))
    }
}

impl Material for Dielectric {
    fn scatter(&self, ray: &Ray, record: &HitRecord) -> Option<(Ray, Vec3)> {
        let cos_theta = (-ray.direction).dot(record.normal).min(1.0);
        let sin_theta = (1.0 - (cos_theta * cos_theta)).sqrt();

        let refraction_ratio = if record.front_face { 1.0 / self.refraction_index } else { self.refraction_index };

        let direction = if (refraction_ratio * sin_theta > 1.0) || (Dielectric::reflectance(cos_theta, refraction_ratio) > random::<f32>()) {
            reflect(&ray.direction, &record.normal)
        } else {
            refract(&ray.direction, &record.normal, refraction_ratio)
        };

        Some((
            Ray {
                origin: record.p,
                direction
            },
            Vec3::new(1.0, 1.0, 1.0)
        ))
    }
}
