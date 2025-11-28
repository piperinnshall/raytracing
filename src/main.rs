mod camera;
mod color;
mod ray;
mod vec3;
mod shape;
mod hit;
mod utils;
mod interval;

use camera::Camera;

const ASPECT_RATIO: f64 = 16.0 / 9.0;
const IMAGE_WIDTH: i32 = 400;
const IMAGE_HEIGHT: i32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as i32;

fn main() {
    assert!(IMAGE_HEIGHT > 1);

    let camera = Camera::new(2.0, IMAGE_WIDTH, IMAGE_HEIGHT);
    camera.render_image();
}
