use glam::Vec3;
use rand::random;

mod ray;
mod sphere;
mod hittable;
mod camera;

use hittable::{
    Hittable,
    HitRecord,
    HittableList
};
use ray::Ray;
use sphere::Sphere;
use camera::Camera;

fn write_color(color: &Vec3, samples_per_pixel: usize) {
    let scale: f32 = 1.0 / (samples_per_pixel as f32);

    let r = color.x * scale;
    let g = color.y * scale;
    let b = color.z * scale;

    println!("{} {} {}", (256.0 * r.clamp(0.0, 0.999)) as usize, (256.0 * g.clamp(0.0, 0.999)) as usize, (256.0 * b.clamp(0.0, 0.999)) as usize);
}

fn ray_color(ray: &Ray, world: &dyn Hittable) -> Vec3 {
    let mut record = HitRecord::new();

    if world.hit(ray, 0.0, std::f32::INFINITY, &mut record) {
        return 0.5 * (record.normal + Vec3::new(1.0, 1.0, 1.0));
    } else {
        let unit_direction = ray.direction.normalize();
        let t = 0.5 * (unit_direction.y + 1.0);
    
        return ((1.0 - t) * Vec3::new(1.0, 1.0, 1.0)) + (t * Vec3::new(0.5, 0.7, 1.0))
    }
}

fn main() {
    // Image
    const ASPECT_RATIO: f32 = 16.0 / 9.0;
    const IMAGE_WIDTH: usize = 400;
    const IMAGE_HEIGHT: usize = ((IMAGE_WIDTH as f32) / ASPECT_RATIO) as usize;
    const SAMPLES_PER_PIXEL: usize = 100;

    // World
    let mut world = HittableList::new();

    let sphere_1 = Sphere {
        center: Vec3::new(0.0, 0.0, -1.0),
        radius: 0.5
    };
    world.add(Box::new(sphere_1));

    let sphere_2 = Sphere {
        center: Vec3::new(0.0, -100.5, -1.0),
        radius: 100.0
    };
    world.add(Box::new(sphere_2));

    // Camera
    let camera = Camera::new();

    // Render
    println!("P3");
    println!("{} {}", IMAGE_WIDTH, IMAGE_HEIGHT);
    println!("255");

    for j in (0..IMAGE_HEIGHT).rev() {
        eprint!("\rScanlines remaining: {}", j);
        for i in 0..IMAGE_WIDTH {
            let mut pixel_color = Vec3::new(0.0, 0.0, 0.0);

            for _ in 0..SAMPLES_PER_PIXEL {
                let u = ((i as f32) + (random::<f32>() as f32)) / ((IMAGE_WIDTH - 1) as f32);
                let v = ((j as f32) + (random::<f32>() as f32)) / ((IMAGE_HEIGHT - 1) as f32);

                let ray = camera.get_ray(u, v);

                let rc = ray_color(&ray, &mut world);

                pixel_color += rc;
            }
            
            write_color(&pixel_color, SAMPLES_PER_PIXEL);
        }
    }

    eprintln!("\nDone!");
}
