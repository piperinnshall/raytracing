pub mod color;
pub mod vec3;
pub mod ray;

use std::io;
use color::Color;
use vec3::Point3;

const ASPECT_RATIO: f64 = 16.0 / 9.0;
const IMAGE_WIDTH: i32 = 400;
const IMAGE_HEIGHT: i32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as i32;

fn main() {

    assert!(IMAGE_HEIGHT > 1);

    let camera = Camera::new(2.0, IMAGE_WIDTH, IMAGE_HEIGHT);

    color::write_header(&mut io::stdout(), IMAGE_WIDTH, IMAGE_HEIGHT);

    for i in 0..IMAGE_HEIGHT {

        eprintln!("Scanlines Remaining: {}", (IMAGE_HEIGHT - i));

        for j in 0..IMAGE_WIDTH {

            let r = j as f64 / (IMAGE_WIDTH - 1) as f64;
            let g = i as f64 / (IMAGE_HEIGHT - 1) as f64;
            let b = 0.25;

            color::write_color(&mut io::stdout(), Color::new(r,g,b) );
        }
    }

    eprintln!("Done!");
}

pub struct Camera {
    center: Point3,
    viewport_width: f64,
    viewport_height: f64,
}

impl Camera { 
    pub fn new(viewport_height: f64, image_width: i32, image_height: i32) -> Self { 
        Self { 
            center: Point3::new(0.0, 0.0, 0.0),
            viewport_width: viewport_height * (image_width / image_height) as f64,
            viewport_height,
        } 
    }

    pub fn center(&self) -> Point3 { self.center }
    pub fn width(&self) -> f64 { self.viewport_width }
    pub fn height(&self) -> f64 { self.viewport_height }
}
