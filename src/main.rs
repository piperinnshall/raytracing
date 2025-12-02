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


fn main() {
    let mut world = HitList::new();

    let ground = Rc::new(Lambertian::new(Color::new(0.8, 0.8, 0.0)));
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
    world.add(Sphere::new(Point3::new(2.3, 0.0, -0.85), 0.5, matte_green));

    let aspect_ratio = 16.0 / 9.0;

    // let image_height = 100;
    // let image_height = 225;
    let image_height = 2160;

    let samples_per_pixel = 100;
    let max_depth = 50;

    let vertical_fov = 60.0;
    let look_from = Point3::new(-2.0, 1.2, 1.0);
    let look_at = Point3::new(0.0, 0.0, -1.0);
    let up = vec3::Vec3::new(0.0, 1.0, 0.0);

    let defocus_angle = 10.0;
    let shiny_center = Point3::new(-1.0, 0.0, -1.0);
    let focus_dist = (look_from - shiny_center).length();

    let camera = Camera::new(
        aspect_ratio,
        image_height,
        samples_per_pixel,
        max_depth,
        vertical_fov,
        look_from,
        look_at,
        up,
        defocus_angle,
        focus_dist,
    );

    camera.render(&world);
}

