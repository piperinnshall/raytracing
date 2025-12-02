use crate::color::{self, Color};
use crate::hit::{HitList, HitRecord, Hittable};
use crate::interval::Interval;
use crate::ray::Ray;
use crate::utils;
use crate::vec3::{Point3, Vec3};
use std::io;

#[derive(Default)]
pub struct Camera {
    vertical_fov: f64 = 90.0,

    look_from: Point3, // Point camera is looking from
    look_at: Point3, // Point camera is looking at
    up: Vec3, // Camera-relative "up" direction
    
    // Camera frame basis vectors
    u: Vec3,
    v: Vec3,
    w: Vec3,

    aspect_ratio: f64 = 16.0 / 9.0, // Ratio of image width over height
    max_depth: i32 = 50, // Maximum number of ray bounces into a scene
    samples_per_pixel: i32 = 100, // Count of random samples for each pixel
    pixel_samples_scale: f64 = 1.0 / 100.0,  // Color scale factor for a small sum of pixel samples

                                             
    image_width: i32, // Rendered image width in pixel count
    image_height: i32, // Rendered image height in pixel count
    viewport_width: f64, // Viewport width in pixel count
    viewport_height: f64, // Viewport height in pixel count

    viewport_u: Vec3, // Vector down viewport vertical edge
    viewport_v: Vec3, // Vector across viewport horizontal edge
    pixel_delta_u: Vec3, // Offset to pixel to the right
    pixel_delta_v: Vec3, // Offset to pixel below

    center: Point3, // Camera center
    focal_length: f64,
    pixel_00_loc: Point3, // Location of pixel 0, 0
}

impl Camera {
    pub fn new(
        aspect_ratio: f64,
        image_height: i32,
        look_from: Point3,
        look_at: Point3,
        up: Vec3,
    ) -> Self {
        let mut cam = Camera {
            aspect_ratio,
            image_height,
            image_width: (image_height as f64 * aspect_ratio).max(1.0) as i32,
            look_from,
            look_at,
            up,
            w: (look_from - look_at).normalized(),
            center: look_from,
            focal_length: (look_from - look_at).length(),
            ..Self::default()
        };

        // Determine viewport dimensions.
        let theta = utils::deg_to_rad(cam.vertical_fov);
        let h = (theta / 2.0).tan();
        cam.viewport_height = 2.0 * h * cam.focal_length;
        cam.viewport_width =
            cam.viewport_height * (cam.image_width as f64 / cam.image_height as f64);

        // Calculate the u,v unit basis vectors for the camera coordinate frame.
        cam.u = cam.up.cross(cam.w).normalized();
        cam.v = cam.w.cross(cam.u);

        // Calculate the vectors across the horizontal and down the vertical viewport edges.
        cam.viewport_u = cam.u * cam.viewport_width;
        cam.viewport_v = -cam.v * cam.viewport_height;

        // Calculate the horizontal and vertical delta vectors from pixel to pixel.
        cam.pixel_delta_u = cam.viewport_u / cam.image_width as f64;
        cam.pixel_delta_v = cam.viewport_v / cam.image_height as f64;

        // Calculate the location of the upper left pixel.
        let viewport_upper_left =
            cam.center - (cam.w * cam.focal_length) - cam.viewport_u/2.0 - cam.viewport_v/2.0;
        cam.pixel_00_loc = 
            viewport_upper_left + (cam.pixel_delta_u + cam.pixel_delta_v) * 0.5;

        cam
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

                color::write_color(&mut io::stdout(), pixel_color * self.pixel_samples_scale);
            }
        }

        eprintln!("Done!");
    }

    fn ray(&self, row: i32, col: i32) -> Ray {
        // Construct a camera ray originating from the origin
        // and directed at randomly sampled point around the pixel location i, j.

        let offset = Self::sample_square();
        let pixel_sample = self.pixel_00_loc
            + (self.pixel_delta_u * (offset.x() + row as f64))
            + (self.pixel_delta_v * (offset.y() + col as f64));

        let ray_direction = pixel_sample - self.center;

        Ray::new(self.center, ray_direction)
    }

    fn sample_square() -> Vec3 {
        // Returns the vector to a random point in the [-.5,-.5]-[+.5,+.5] unit square.
        Vec3::new(utils::random_f64() - 0.5, utils::random_f64() - 0.5, 0.0)
    }

    fn color(depth: i32, ray: Ray, world: &HitList) -> Color {
        // If we've exceeded the ray bounce limit, no more light is gathered.
        if depth <= 0 {
            return Color::default();
        }

        let mut rec = HitRecord::default();

        if world.hit(&ray, Interval::new(0.001, f64::INFINITY), &mut rec) {
            if let Some(mat) = rec.mat.clone() {
                match mat.scatter(ray, rec) {
                    Some((attenuation, scattered)) => {
                        return attenuation * Self::color(depth - 1, scattered, &world)
                    }
                    None => return Color::default(),
                }
            }
        }

        let unit_direction = ray.direction().normalized();
        let t = 0.5 * (unit_direction.y() + 1.0);

        color::lerp(Color::fill(1.0), Color::new(0.5, 0.7, 1.0), t)
    }
}
