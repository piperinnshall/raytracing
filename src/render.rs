use crate::vec3::{Point3, Vec3};
use crate::camera::Camera;
use crate::ray::Ray;
use crate::color::{self, Color};
use crate::shape::Sphere;

use std::io;

pub fn render_image(camera: Camera, image_height: i32, image_width: i32) {
    let pixel_delta_u = camera.delta_u(image_width as f64);
    let pixel_delta_v = camera.delta_v(image_height as f64);

    let pixel00_loc: Point3 = 
        camera.upper_left() + (pixel_delta_u + pixel_delta_v) * 0.5;

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

            let color = color(ray);

            color::write_color(&mut io::stdout(), color);
        }
    }

    eprintln!("Done!");
}

fn color(ray: Ray) -> Color {
    let sphere = Sphere {
        center: Vec3::new(0.0, 0.0, -1.0),
        radius: 0.5,
    };

    let unit_direction = ray.direction().normalize();
    let t = 0.5 * (unit_direction.y() + 1.0);

    color::lerp(Color::fill(1.0), Color::new(0.5, 0.7, 1.0), t)
}



