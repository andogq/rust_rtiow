use glam::Vec3;
use rand::random;

mod camera;
mod hittable;
mod material;
mod ray;
mod sphere;

use camera::Camera;
use hittable::{Hittable, HittableList};
use material::{Dielectric, Lambertian, Metal};
use ray::Ray;
use sphere::Sphere;

fn near_zero(v: &Vec3) -> bool {
    let s: f32 = 1e-8;

    (v.x.abs() < s) && (v.y.abs() < s) && (v.z.abs() < s)
}

fn reflect(v: &Vec3, n: &Vec3) -> Vec3 {
    *v - (2.0 * v.dot(*n) * (*n))
}

fn refract(ray: &Vec3, normal: &Vec3, etai_over_etat: f32) -> Vec3 {
    let cos_theta = (-ray.normalize()).dot(*normal).min(1.0);
    let ray_out_perpendicular = etai_over_etat * (ray.normalize() + (cos_theta * (*normal)));
    let ray_out_parallel =
        -((1.0 - ray_out_perpendicular.length_squared()).abs().sqrt()) * (*normal);

    ray_out_perpendicular + ray_out_parallel
}

fn write_color(color: &Vec3, samples_per_pixel: usize) {
    let scale: f32 = 1.0 / (samples_per_pixel as f32);

    let r = (color.x * scale).sqrt();
    let g = (color.y * scale).sqrt();
    let b = (color.z * scale).sqrt();

    println!(
        "{} {} {}",
        (256.0 * r.clamp(0.0, 0.999)) as usize,
        (256.0 * g.clamp(0.0, 0.999)) as usize,
        (256.0 * b.clamp(0.0, 0.999)) as usize
    );
}

fn ray_color(ray: &Ray, world: &dyn Hittable, depth: usize) -> Vec3 {
    if depth == 0 {
        return Vec3::new(0.0, 0.0, 0.0);
    }

    if let Some(record) = world.hit(ray, 0.001, std::f32::INFINITY) {
        if let Some(material) = record.material {
            if let Some((ray, attenuation)) = material.scatter(ray, &record) {
                return attenuation * ray_color(&ray, world, depth - 1);
            }
        }

        Vec3::new(0.0, 0.0, 0.0)
    } else {
        let unit_direction = ray.direction.normalize();
        let t = 0.5 * (unit_direction.y + 1.0);

        ((1.0 - t) * Vec3::new(1.0, 1.0, 1.0)) + (t * Vec3::new(0.5, 0.7, 1.0))
    }
}

fn random_unit_vector() -> Vec3 {
    random_in_unit_sphere().normalize()
}

#[allow(dead_code)]
fn random_in_hemisphere(normal: &Vec3) -> Vec3 {
    let in_unit_sphere = random_in_unit_sphere();

    if in_unit_sphere.dot(*normal) > 0.0 {
        in_unit_sphere
    } else {
        -in_unit_sphere
    }
}

fn random_in_unit_sphere() -> Vec3 {
    let mut point: Option<Vec3> = None;

    while point.is_none() {
        point = Some(Vec3::new(
            (random::<f32>() * 2.0) - 1.0,
            (random::<f32>() * 2.0) - 1.0,
            (random::<f32>() * 2.0) - 1.0,
        ));

        if point.unwrap().length() > 1.0 {
            point = None;
        }
    }

    point.unwrap()
}

fn random_in_unit_disk() -> Vec3 {
    let mut point = Vec3::new(1.0, 1.0, 1.0);

    while point.length_squared() >= 1.0 {
        point = Vec3::new(
            (random::<f32>() * 2.0) - 1.0,
            (random::<f32>() * 2.0) - 1.0,
            0.0,
        );
    }

    point
}

fn random_vec() -> Vec3 {
    Vec3::new(random::<f32>(), random::<f32>(), random::<f32>())
}

fn random_scene() -> HittableList {
    let mut world = HittableList::new();

    let ground_material = Lambertian {
        albedo: Vec3::new(0.5, 0.5, 0.5),
    };
    world.add(Box::new(Sphere {
        center: Vec3::new(0.0, -1000.0, 0.0),
        radius: 1000.0,
        material: ground_material,
    }));

    for a in -11..11 {
        for b in -11..11 {
            let center = Vec3::new(
                (a as f32) + (0.9 * random::<f32>()),
                0.2,
                (b as f32) + (0.9 * random::<f32>()),
            );

            match random::<f32>() {
                r if r < 0.8 => world.add(Box::new(Sphere {
                    center,
                    radius: 0.2,
                    material: Lambertian {
                        albedo: random_vec() * random_vec(),
                    },
                })),
                r if r <= 0.95 => world.add(Box::new(Sphere {
                    center,
                    radius: 0.2,
                    material: Metal {
                        albedo: random_vec(),
                        fuzz: random::<f32>() * 0.5,
                    },
                })),
                _ => world.add(Box::new(Sphere {
                    center,
                    radius: 0.2,
                    material: Dielectric {
                        refraction_index: 1.5,
                    },
                })),
            };
        }
    }

    let material_1 = Dielectric {
        refraction_index: 1.5
    };
    world.add(Box::new(
        Sphere {
            center: Vec3::new(0.0, 1.0, 0.0),
            radius: 1.0,
            material: material_1
        }
    ));

    let material_2 = Lambertian {
        albedo: Vec3::new(0.4, 0.2, 0.1)
    };
    world.add(Box::new(
        Sphere {
            center: Vec3::new(-4.0, 1.0, 0.0),
            radius: 1.0,
            material: material_2
        }
    ));

    let material_3 = Metal {
        albedo: Vec3::new(0.7, 0.6, 0.5),
        fuzz: 0.0
    };
    world.add(Box::new(
        Sphere {
            center: Vec3::new(4.0, 1.0, 0.0),
            radius: 1.0,
            material: material_3
        }
    ));

    world
}

fn main() {
    // Image
    const ASPECT_RATIO: f32 = 16.0 / 9.0;
    const IMAGE_HEIGHT: usize = 540;
    const IMAGE_WIDTH: usize = (ASPECT_RATIO * (IMAGE_HEIGHT as f32)) as usize;
    const SAMPLES_PER_PIXEL: usize = 20;
    const MAX_DEPTH: usize = 100;

    // World
    let world = random_scene();

    // Camera
    let look_from = Vec3::new(13.0, 2.0, 3.0);
    let look_at = Vec3::new(0.0, 0.0, 0.0);

    let camera = Camera::new(
        look_from,
        look_at,
        Vec3::new(0.0, 1.0, 0.0),
        20.0,
        ASPECT_RATIO,
        0.1,
        10.0
    );

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

                let rc = ray_color(&ray, &world, MAX_DEPTH);

                pixel_color += rc;
            }

            write_color(&pixel_color, SAMPLES_PER_PIXEL);
        }
    }

    eprintln!("\nDone!");
}
