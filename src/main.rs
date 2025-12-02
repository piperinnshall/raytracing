#![feature(default_field_values)] 

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
const IMAGE_HEIGHT: i32 = 225;

fn main() {
    assert!(IMAGE_HEIGHT > 1);

    let mut world = HitList::new();

    let material_ground = Rc::new(Metal::new(Color::new(0.2, 0.2, 0.2), 0.0));
    let material_center = Rc::new(Metal::new(Color::new(0.10, 0.36, 0.84), 0.8));
    let material_right = Rc::new(Lambertian::new(Color::new(0.0, 0.85, 0.0)));
    let material_left = Rc::new(Dielectric::new(1.50));
    let material_bubble = Rc::new(Dielectric::new(1.0 / 1.50));

    world.add(Sphere::new(Point3::new(-0.0, -100.5, -1.0), 100.0, material_ground));
    world.add(Sphere::new(Point3::new(0.0, 0.0, -1.2), 0.5, material_center));
    world.add(Sphere::new(Point3::new(-1.0, 0.0, -1.0), 0.5, material_right));
    world.add(Sphere::new(Point3::new(1.0, 0.0, -1.0), 0.5, material_left));
    world.add(Sphere::new(Point3::new(1.0, 0.0, -1.0), 0.4, material_bubble));

    let look_from = Point3::new(-2.0, 2.0, 1.0);
    let look_at = Point3::new(0.0, 0.0, -1.0);
    let up = vec3::Vec3::new(0.0, 1.0, 0.0);

    let camera = Camera::new(
        ASPECT_RATIO,
        IMAGE_HEIGHT,
        look_from,
        look_at,
        up,
    );

    camera.render(&world);
}

