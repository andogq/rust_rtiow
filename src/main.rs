use glam::Vec3;

mod ray;
use ray::Ray;

fn write_color(color: &Vec3) {
    println!("{} {} {}", (color.x * 255.999) as usize, (color.y * 255.999) as usize, (color.z * 255.999) as usize);
}

fn hit_sphere(center: &Vec3, radius: f32, ray: &Ray) -> f32 {
    let oc = ray.origin - center.clone();

    let a = ray.direction.length_squared();
    let b = oc.dot(ray.direction);
    let c = oc.length_squared() - (radius * radius);

    let discriminant: f32 = (b * b) - (a * c);

    if discriminant < 0.0 {
        return -1.0;
    } else {
        return (-b - discriminant.sqrt()) / a;
    }
}

fn ray_color(ray: &Ray) -> Vec3 {
    let mut t = hit_sphere(&Vec3::new(0.0, 0.0, -1.0), 0.5, ray);

    if t > 0.0 {
        let normal = (ray.at(t) - Vec3::new(0.0, 0.0, -1.0)).normalize();
        return 0.5 * (normal + Vec3::new(1.0, 1.0, 1.0));
    }
            
    let unit_direction = ray.direction.normalize();
    t = 0.5 * (unit_direction.y + 1.0);

    ((1.0 - t) * Vec3::new(1.0, 1.0, 1.0)) + (t * Vec3::new(0.5, 0.7, 1.0))
}

fn main() {
    // Image
    const ASPECT_RATIO: f32 = 16.0 / 9.0;
    const IMAGE_WIDTH: usize = 400;
    const IMAGE_HEIGHT: usize = ((IMAGE_WIDTH as f32) / ASPECT_RATIO) as usize;

    // Camera
    const VIEWPORT_HEIGHT: f32 = 2.0;
    const VIEWPORT_WIDTH: f32 = ASPECT_RATIO * VIEWPORT_HEIGHT;
    const FOCAL_LENGTH: f32 = 1.0;

    let ORIGIN: Vec3 = Vec3::new(0.0, 0.0, 0.0);
    let HORIZONTAL: Vec3 = Vec3::new(VIEWPORT_WIDTH, 0.0, 0.0);
    let VERTICAL: Vec3 = Vec3::new(0.0, VIEWPORT_HEIGHT, 0.0);
    let LOWER_LEFT_CORNER: Vec3 = ORIGIN - (HORIZONTAL / 2.0) - (VERTICAL / 2.0) - Vec3::new(0.0, 0.0, FOCAL_LENGTH);

    // Render
    println!("P3");
    println!("{} {}", IMAGE_WIDTH, IMAGE_HEIGHT);
    println!("255");

    for j in (0..IMAGE_HEIGHT).rev() {
        eprint!("\rScanlines remaining: {}", j);
        for i in 0..IMAGE_WIDTH {
            let u: f32 = (i as f32) / ((IMAGE_WIDTH - 1) as f32);
            let v: f32 = (j as f32) / ((IMAGE_HEIGHT - 1) as f32);

            let ray = Ray {
                origin: ORIGIN,
                direction: LOWER_LEFT_CORNER + (u * HORIZONTAL) + (v * VERTICAL) - ORIGIN
            };

            let pixel_color = ray_color(&ray);
            write_color(&pixel_color);
        }
    }

    eprintln!("\nDone!");
}
