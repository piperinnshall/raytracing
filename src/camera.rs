use crate::vec3::{Vec3,Point3};
use crate::hit::{HitRecord, Hittable, HitList};
use crate::interval::Interval;
use crate::ray::Ray;
use crate::color::{self, Color};
use std::io;

pub struct Camera {
    aspect_ratio: f64,
    focal_length: f64,
    image_width: i32,
    image_height: i32,
    viewport_width: f64,
    viewport_height: f64,
    viewport_u: Vec3,
    viewport_v: Vec3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
    center: Point3,
    pixel_00_loc: Point3,
    samples_per_pixel: i32,

}

impl Camera { 
    pub fn new(aspect_ratio: f64, viewport_height: f64, image_height: i32) -> Self { 
        let focal_length = 1.0;

        let image_width = (image_height as f64 * aspect_ratio) as i32;
        let viewport_width = viewport_height * (image_width as f64 / image_height as f64);

        // Calculate the vectors across the horizontal and down the vertical viewport edges.
        let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
        let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);
        
        // Calculate the horizontal and vertical delta vectors from pixel to pixel.
        let pixel_delta_u = viewport_u / image_width as f64;
        let pixel_delta_v = viewport_v / image_height as f64;

        let center = Vec3::default();
        let pixel_00_loc: Point3 = 
            Self::upper_left(center, viewport_u, viewport_v, focal_length) 
            + (pixel_delta_u + pixel_delta_v) * 0.5;

        let samples_per_pixel = 10;

        Self { 
            aspect_ratio,
            focal_length,
            image_width,
            image_height,
            viewport_width,
            viewport_height,
            viewport_u,
            viewport_v,
            pixel_delta_u,
            pixel_delta_v,
            center,
            pixel_00_loc,
            samples_per_pixel,
        } 
    }

    pub fn render(&self, world: &HitList) {
        eprintln!("--- Begin Rendering ---");

        color::write_header(&mut io::stdout(), self.image_width, self.image_height);

        for col in 0..self.image_height {

            // eprintln!("scan lines remaining: {}", (self.image_height - col));

            for row in 0..self.image_width {

                let pixel_center = self.pixel_00_loc 
                    + (self.pixel_delta_u * row as f64) 
                    + (self.pixel_delta_v * col as f64);

                let ray_direction = pixel_center - self.center;
                let ray = Ray::new(self.center, ray_direction);

                let color = Self::color(ray, &world);

                color::write_color(&mut io::stdout(), color);
            }
        }

        eprintln!("Done!");
    }

    fn color(ray: Ray, world: &HitList) -> Color {
        let mut rec = HitRecord::default();
        if world.hit(&ray, Interval::new(0.0, f64::INFINITY), &mut rec) {
            return (Color::fill(1.0) + rec.normal) * 0.5
        }

        let unit_direction = ray.direction().normalize();
        let t = 0.5 * (unit_direction.y() + 1.0);

        color::lerp(Color::fill(1.0), Color::new(0.5, 0.7, 1.0), t)
    }

    fn upper_left(center: Point3, u: Vec3, v: Vec3,focal_length: f64) -> Point3 {
        center 
            - Vec3::new(0.0, 0.0, focal_length) 
            - u / 2.0
            - v / 2.0
    }
}
