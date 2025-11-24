pub mod image;
pub mod vector;

use image::Color;
use vector::Vec3;

fn main() {
    let image_width = 256;
    let image_height = 256;

    println!("P3\n{} {}\n255", image_width, image_height);

    for i in 0..image_height {
        eprintln!("Scanlines Remaining: {}", (image_height - i));
        for j in 0..image_width {
            let r = j as f64 / (image_width - 1) as f64;
            let g = i as f64 / (image_height - 1) as f64;
            let b = 0.0;
            let color = Color {
                r: (255.999 * r) as u8,
                g: (255.999 * g) as u8,
                b: (255.999 * b) as u8,
                a: 255,
            };

            println!("{} {} {}", color.r, color.g, color.b);
        }
    }

    eprintln!("Done!");
}
