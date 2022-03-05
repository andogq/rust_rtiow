fn main() {
    const HEIGHT: usize = 256;
    const WIDTH: usize = 256;

    println!("P3");
    println!("{} {}", HEIGHT, WIDTH);
    println!("255");

    for j in (0..HEIGHT) {
        eprintln!("Scanlines remaining: {}", j);
        for i in 0..WIDTH {
            let r: f32 = (i as f32) / ((WIDTH - 1) as f32);
            let g: f32 = (j as f32) / ((HEIGHT - 1) as f32);
            let b: f32 = 0.25;

            let ir: usize = (255.999 * r) as usize;
            let ig: usize = (255.999 * g) as usize;
            let ib: usize = (255.999 * b) as usize;

            println!("{} {} {}", ir, ig, ib);
        }
    }
}
