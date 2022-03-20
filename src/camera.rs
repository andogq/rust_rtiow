use glam::Vec3;

use crate::{ray::Ray, random_in_unit_disk};

pub struct Camera {
    pub origin: Vec3,
    pub lower_left_corner: Vec3,
    pub horizontal: Vec3,
    pub vertical: Vec3,

    pub w: Vec3,
    pub u: Vec3,
    pub v: Vec3,

    pub lens_radius: f32
}

impl Camera {
    pub fn new(look_from: Vec3, look_at: Vec3, v_up: Vec3, v_fov: f32, aspect_ratio: f32, aperture: f32, focus_distance: f32) -> Camera {
        let theta = v_fov.to_radians();
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        let w = (look_from - look_at).normalize();
        let u = v_up.cross(w).normalize();
        let v= w.cross(u);

        let origin = look_from;
        let horizontal = focus_distance * viewport_width * u;
        let vertical = focus_distance * viewport_height * v;
        let lower_left_corner = origin - (horizontal / 2.0) - (vertical / 2.0) - (focus_distance * w);

        let lens_radius = aperture / 2.0;

        Camera {
            origin,
            horizontal,
            vertical,
            lower_left_corner,

            w,
            u,
            v,

            lens_radius
        }
    }

    pub fn get_ray(&self, u: f32, v: f32) -> Ray {
        let rd = self.lens_radius * random_in_unit_disk();
        let offset = (self.u * rd.x) + (self.v * rd.y);

        Ray {
            origin: self.origin + offset,
            direction: self.lower_left_corner + (u * self.horizontal) + (v * self.vertical) - self.origin - offset
        }
    }
}