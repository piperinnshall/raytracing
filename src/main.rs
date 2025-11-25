pub mod color;
pub mod vec3;

use std::io;
use color::Color;

const IMAGE_WIDTH: i32 = 256;
const IMAGE_HEIGHT: i32 = 256;

fn main() {

    println!("P3\n{} {}\n255", IMAGE_WIDTH, IMAGE_HEIGHT);

    for i in 0..IMAGE_HEIGHT {

        eprintln!("Scanlines Remaining: {}", (IMAGE_HEIGHT - i));

        for j in 0..IMAGE_WIDTH {

            let r = j as f64 / (IMAGE_WIDTH - 1) as f64;
            let g = i as f64 / (IMAGE_HEIGHT - 1) as f64;
            let b = 0.25;

            color::write_color(&mut io::stdout(), Color::new(r,g,b));
        }
    }

    eprintln!("Done!");
}
