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
// const IMAGE_HEIGHT: i32 = 100;
const IMAGE_HEIGHT: i32 = 225;
// const IMAGE_HEIGHT: i32 = 2160;

fn main() {
    assert!(IMAGE_HEIGHT > 1);

    let mut world = HitList::new();

    let ground = Rc::new(Metal::new(Color::new(0.7, 0.7, 0.7), 0.0));
    let shiny_metal = Rc::new(Metal::new(Color::new(0.7, 0.7, 0.7), 0.0));
    let matte_green = Rc::new(Lambertian::new(Color::new(0.0, 0.45, 0.0)));
    let glass = Rc::new(Dielectric::new(1.50));
    let bubble = Rc::new(Dielectric::new(1.0 / 1.50));

    // In order, front to back
    world.add(Sphere::new(Point3::new(-0.0, -100.5, -1.0), 100.0, ground));
    world.add(Sphere::new(Point3::new(0.0, 0.0, -1.2), 0.5, glass.clone()));
    world.add(Sphere::new(Point3::new(-1.0, 0.0, -1.0), 0.5, shiny_metal));
    world.add(Sphere::new(Point3::new(1.0, 0.0, -1.0), 0.5, glass.clone()));
    world.add(Sphere::new(Point3::new(1.0, 0.0, -1.0), 0.4, bubble));
    world.add(Sphere::new(Point3::new(2.3, 0.0, -0.9), 0.5, matte_green));

    let vertical_fov = 60.0;
    let look_from = Point3::new(-2.0, 1.2, 1.0);
    let look_at = Point3::new(0.0, 0.0, -1.0);
    let up = vec3::Vec3::new(0.0, 1.0, 0.0);

    let camera = Camera::new(
        vertical_fov,
        ASPECT_RATIO,
        IMAGE_HEIGHT,
        look_from,
        look_at,
        up,
    );

    camera.render(&world);
}

