use crate::hit::{HitRecord, Hittable, HitList};
use crate::vec3::{Point3, Vec3};
use crate::camera::Camera;
use crate::ray::Ray;
use crate::color::{self, Color};
use crate::shape::{self, Sphere};
use crate::utils;

use std::io;

pub fn render_image(camera: Camera, image_width: i32, image_height: i32) {
    
    // World

    let mut world = HitList::new();
    world.add(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5));
    world.add(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0));
    world.add(Sphere::new(Point3::new(-1.0, 0.5, -1.0), 0.3));

    // Camera 
    
    let pixel_delta_u = camera.delta_u(image_width as f64);
    let pixel_delta_v = camera.delta_v(image_height as f64);

    let pixel00_loc: Point3 = 
        camera.upper_left() + (pixel_delta_u + pixel_delta_v) * 0.5;

    // Render

    eprintln!("--- Begin Rendering ---");

    color::write_header(&mut io::stdout(), image_width, image_height);

    for col in 0..image_height {

        eprintln!("scan lines remaining: {}", (image_height - col));

        for row in 0..image_width {

            let pixel_center = pixel00_loc 
                + (pixel_delta_u * row as f64) 
                + (pixel_delta_v * col as f64);

            let ray_direction = pixel_center - camera.center();
            let ray = Ray::new(camera.center(), ray_direction);

            let color = color(ray, &world);

            color::write_color(&mut io::stdout(), color);
        }
    }

    eprintln!("Done!");
}

fn color(ray: Ray, world: &HitList) -> Color {
    let mut rec = HitRecord::default();
    if world.hit(&ray, 0.001, f64::INFINITY, &mut rec) {
        return (Color::fill(1.0) + rec.normal) * 0.5
    }

    let unit_direction = ray.direction().normalize();
    let t = 0.5 * (unit_direction.y() + 1.0);

    color::lerp(Color::fill(1.0), Color::new(0.5, 0.7, 1.0), t)
}

