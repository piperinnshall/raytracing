mod camera;
mod color;
mod ray;
mod vec3;
mod shape;
mod hit;
mod utils;
mod interval;

use camera::Camera;
use hit::HitList;
use shape::Sphere;
use vec3::Point3;

const ASPECT_RATIO: f64 = 16.0 / 9.0;
const VIEWPORT_HEIGHT: f64 = 2.0;
const IMAGE_HEIGHT: i32 = 1080;

fn main() {
    assert!(IMAGE_HEIGHT > 1);
   
    let mut world = HitList::new();
    world.add(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5));
    world.add(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0));
    world.add(Sphere::new(Point3::new(-1.0, -0.5, -1.0), 0.4));

    let camera = Camera::new(ASPECT_RATIO, VIEWPORT_HEIGHT, IMAGE_HEIGHT);
    camera.render(&world);
}
