pub mod color;
pub mod vec3;
pub mod ray;
pub mod camera;

use std::io;
use camera::Camera;
use color::Color;
use vec3::Point3;
use ray::Ray;

const ASPECT_RATIO: f64 = 16.0 / 9.0;
const IMAGE_WIDTH: i32 = 400;
const IMAGE_HEIGHT: i32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as i32;

fn main() {
    assert!(IMAGE_HEIGHT > 1);

    let camera = Camera::new(2.0, IMAGE_WIDTH, IMAGE_HEIGHT);

    render(camera);
}

fn render(camera: Camera) {

    let pixel_delta_u = camera.delta_u(IMAGE_WIDTH as f64);
    let pixel_delta_v = camera.delta_v(IMAGE_HEIGHT as f64);

    let pixel00_loc: Point3 = camera.upper_left() + (pixel_delta_u + pixel_delta_v) * 0.5;

    eprintln!("--- Begin Rendering ---");

    color::write_header(&mut io::stdout(), IMAGE_WIDTH, IMAGE_HEIGHT);

    for col in 0..IMAGE_HEIGHT {

        eprintln!("Scan lines Remaining: {}", (IMAGE_HEIGHT - col));

        for row in 0..IMAGE_WIDTH {

            let pixel_center = pixel00_loc 
                + (pixel_delta_u * row as f64) 
                + (pixel_delta_v * col as f64);

            let ray_direction = pixel_center - camera.center();
            let ray = Ray::new(camera.center(), ray_direction);

            let color = color(ray);

            color::write_color(&mut io::stdout(), color);
        }
    }

    eprintln!("Done!");
}

fn color(ray: Ray) -> Color {
    Color::default()
}
