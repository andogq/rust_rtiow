use glam::Vec3;

pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3
}

impl Ray {
    pub fn at(&self, t: f32) -> Vec3{
        return self.origin + (self.direction * t);
    }
}
