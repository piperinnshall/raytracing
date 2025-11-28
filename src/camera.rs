use crate::vec3::{Vec3,Point3};
use crate::hit::{HitRecord, Hittable, HitList};
use crate::interval::Interval;
use crate::ray::Ray;
use crate::color::{self, Color};
use crate::shape::Sphere;
use std::io;

pub struct Camera {
    center: Point3,
    focal_length: f64,
    viewport_width: f64,
    viewport_height: f64,
    image_width: i32,
    image_height: i32,
}

impl Camera { 
    pub fn new(viewport_height: f64, image_width: i32, image_height: i32) -> Self { 
        Self { 
            center: Vec3::default(),
            focal_length: 1.0,
            viewport_width: viewport_height * (image_width as f64 / image_height as f64),
            viewport_height,
            image_width,
            image_height
        } 
    }
    
    pub fn render(&self, world: &HitList) {
        let pixel_delta_u = self.delta_u(self.image_width as f64);
        let pixel_delta_v = self.delta_v(self.image_height as f64);

        let pixel00_loc: Point3 = 
            self.upper_left() + (pixel_delta_u + pixel_delta_v) * 0.5;

        eprintln!("--- Begin Rendering ---");

        color::write_header(&mut io::stdout(), self.image_width, self.image_height);

        for col in 0..self.image_height {

            eprintln!("scan lines remaining: {}", (self.image_height - col));

            for row in 0..self.image_width {

                let pixel_center = pixel00_loc 
                    + (pixel_delta_u * row as f64) 
                    + (pixel_delta_v * col as f64);

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


    fn upper_left(&self) -> Point3 {
        self.center 
            - Vec3::new(0.0, 0.0, self.focal_length) 
            - self.viewport_u() / 2.0
            - self.viewport_v() / 2.0
    }

    // Calculate the vectors across the horizontal and down the vertical viewport edges.
    fn viewport_u(&self) -> Vec3 { Vec3::new(self.viewport_width, 0.0, 0.0) }
    fn viewport_v(&self) -> Vec3 { Vec3::new(0.0, -self.viewport_height, 0.0) }

    // Calculate the horizontal and vertical delta vectors from pixel to pixel.
    fn delta_u(&self, direction: f64) -> Vec3 { self.viewport_u() / direction }
    fn delta_v(&self, direction: f64) -> Vec3 { self.viewport_v() / direction }

}
