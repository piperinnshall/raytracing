use crate::color::{self, Color};
use crate::hit::{HitList, HitRecord, Hittable};
use crate::interval::Interval;
use crate::ray::Ray;
use crate::utils;
use crate::vec3::{Point3, Vec3};
use std::io;

pub struct Camera {
    aspect_ratio: f64, // Ratio of image width over height
    focal_length: f64,
    image_width: i32, // Rendered image width in pixel count
    image_height: i32, // Rendered image height in pixel count
    viewport_width: f64, // Viewport width in pixel count
    viewport_height: f64, // Viewport height in pixel count
    viewport_u: Vec3,
    viewport_v: Vec3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
    center: Point3, // Camera center
    pixel_00_loc: Point3, // Location of pixel 0, 0
    max_depth: i32, // Maximum number of ray bounces into a scene
    samples_per_pixel: i32, // Count of random samples  for each pixel
    pixel_samples_scale: f64 // Color scale factor for a small sum of pixel samples
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
        let pixel_00_loc: Point3 = Self::upper_left(center, viewport_u, viewport_v, focal_length)
            + (pixel_delta_u + pixel_delta_v) * 0.5;

        let max_depth = 50;
        let samples_per_pixel = 100;
        let pixel_samples_scale = 1.0 / samples_per_pixel as f64;

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
            max_depth,
            samples_per_pixel,
            pixel_samples_scale,
        }
    }

    pub fn render(&self, world: &HitList) {
        eprintln!("--- Begin Rendering ---");

        color::write_header(&mut io::stdout(), self.image_width, self.image_height);

        for col in 0..self.image_height {
            eprintln!("scan lines remaining: {}", (self.image_height - col));

            for row in 0..self.image_width {
                let mut pixel_color = Color::default();

                for _ in 0..self.samples_per_pixel {
                    let ray = self.ray(row, col);
                    pixel_color += Self::color(self.max_depth, ray, &world);
                }

                color::write_color(&mut io::stdout(), pixel_color * self.pixel_samples_scale );
            }
        }

        eprintln!("Done!");
    }

    fn ray(&self, row: i32, col: i32) -> Ray {
        // Construct a camera ray originating from the origin
        // and directed at randomly sampled point around the pixel location i, j.

        let offset = self.sample_square();
        let pixel_sample = self.pixel_00_loc
            + (self.pixel_delta_u * (offset.x() + row as f64))
            + (self.pixel_delta_v * (offset.y() + col as f64));

        let ray_direction = pixel_sample - self.center;

        Ray::new(self.center, ray_direction)
    }

    fn sample_square(&self) -> Vec3 {
        // Returns the vector to a random point in the [-.5,-.5]-[+.5,+.5] unit square.
        Vec3::new(utils::random_f64() - 0.5, utils::random_f64() - 0.5, 0.0)
    }

    fn upper_left(center: Point3, u: Vec3, v: Vec3, focal_length: f64) -> Point3 {
        center - Vec3::new(0.0, 0.0, focal_length) - u / 2.0 - v / 2.0
    }

    fn color(depth: i32, ray: Ray, world: &HitList) -> Color {
        // If we've exceeded the ray bounce limit, no more light is gathered.
        if depth <= 0 {
            return Color::default()
        }

        let mut rec = HitRecord::default();

        if world.hit(&ray, Interval::new(0.001, f64::INFINITY), &mut rec) {
            let direction = rec.normal + Vec3::random_normalized();
            return Self::color(depth - 1, Ray::new(rec.point, direction), &world) * 0.5
        }

        let unit_direction = ray.direction().normalized();
        let t = 0.5 * (unit_direction.y() + 1.0);

        color::lerp(Color::fill(1.0), Color::new(0.5, 0.7, 1.0), t)
    }
}

