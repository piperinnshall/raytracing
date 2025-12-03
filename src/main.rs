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
use color::Color;
use hit::HitList;
use material::{Dielectric, Lambertian, Metal, Material};
use rand::Rng;
use shape::Sphere;
use vec3::{Point3, Vec3};

fn main() {
    let mut world = HitList::new();

    let ground_material = Rc::new(Lambertian::new(Color::new(0.15, 0.35, 0.15)));
    world.add(Sphere::new( Point3::new(0.0, -1000.0, 0.0), 1000.0, ground_material));

    let mut rng = rand::rng();
    for a in -11..11 {
        for b in -11..11 {
            let choose_mat: f64 = rng.random();

            let center = Point3::new(
                a as f64 + 0.9 * rng.random::<f64>(),
                0.2,
                b as f64 + 0.9 * rng.random::<f64>(),
            );

            if (center - Point3::new(4.0, 0.2, 0.0)).length() > 0.78 {
                let sphere_material: Rc<dyn Material> = if choose_mat < 0.8 {
                    // diffuse
                    let albedo = Color::random() * Color::random();
                    Rc::new(Lambertian::new(albedo))
                } else if choose_mat < 0.90 {
                    // metal
                    let albedo = Color::random_range(0.5, 1.0);
                    let fuzz = rng.random_range(0.0..0.5);
                    Rc::new(Metal::new(albedo, fuzz))
                } else {
                    // glass
                    Rc::new(Dielectric::new(1.5))
                };

                world.add(Sphere::new(center, 0.2, sphere_material));
            }
        }
    }

    let glass = Rc::new(Dielectric::new(1.5));
    world.add(Sphere::new(Point3::new(0.0, 1.0, 0.0), 1.0, glass.clone()));

    let air = Rc::new(Dielectric::new(1.0 / 1.50));
    world.add(Sphere::new( Point3::new(-4.0, 1.0, 0.0), 1.0, glass));
    world.add(Sphere::new( Point3::new(-4.0, 1.0, 0.0), 0.8, air));

    let metal = Rc::new(Metal::new(Color::new(0.7, 0.78, 0.7), 0.0));
    world.add(Sphere::new( Point3::new(4.0, 1.0, 0.0), 1.0, metal));

    let aspect_ratio = 16.0 / 9.0;
    let image_height = 2160;
    let samples_per_pixel = 500;
    let max_depth = 50;

    let vfov = 20.0;
    let lookfrom = Point3::new(12.88, 2.0, -3.46);

    let lookat = Point3::new(0.0, 0.0, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);

    let defocus_angle = 0.75;
    let focus_dist = (Point3::new(4.0, 1.0, 0.0) - lookfrom).length();


    let camera = Camera::new(
        aspect_ratio,
        image_height,
        samples_per_pixel,
        max_depth,
        vfov,
        lookfrom,
        lookat,
        vup,
        defocus_angle,
        focus_dist,
    );

    camera.render(&world);
}

