mod camera;
mod color;
mod hit;
mod interval;
mod material;
mod ray;
mod shape;
mod utils;
mod vec3;

use std::rc::Rc;

use camera::Camera;
use hit::HitList;
use shape::Sphere;
use vec3::Point3;
use material::{Lambertian, Metal, Dielectric};
use color::Color;

const ASPECT_RATIO: f64 = 16.0 / 9.0;
const VIEWPORT_HEIGHT: f64 = 2.0;
// const IMAGE_HEIGHT: i32 = 225;
const IMAGE_HEIGHT: i32 = 2160;

fn main() {
    assert!(IMAGE_HEIGHT > 1);

    let mut world = HitList::new();

    // Materials
    let material_ground = Rc::new(Metal::new(Color::new(1.0, 0.4, 0.0), 0.0)); // orange metal
    let material_right = Rc::new(Lambertian::new(Color::new(0.0, 0.85, 0.0))); // green matte
    let material_center = Rc::new(Metal::new(Color::new(0.10, 0.36, 0.84), 0.8)); // fuzzy blue metal
    let material_left = Rc::new(Dielectric::new(1.50)); // glass
    let material_bubble = Rc::new(Dielectric::new(1.0 / 1.50)); // air

    world.add(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0, material_ground.clone()));
    world.add(Sphere::new(Point3::new(0.0, 0.0, -1.2), 0.5, material_center.clone()));
    world.add(Sphere::new(Point3::new(-1.0, 0.0, -1.0), 0.5, material_right.clone()));
    world.add(Sphere::new(Point3::new(1.0, 0.0, -1.0), 0.5, material_left.clone()));
    world.add(Sphere::new(Point3::new(1.0, 0.0, -1.0), 0.4, material_bubble.clone()));

    let camera = Camera::new(ASPECT_RATIO, VIEWPORT_HEIGHT, IMAGE_HEIGHT);
    camera.render(&world);
}

