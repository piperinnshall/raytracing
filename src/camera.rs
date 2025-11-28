use crate::vec3::{Vec3,Point3};

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
            image_height,
        } 
    }

    pub fn upper_left(&self) -> Point3 {
        self.center 
            - Vec3::new(0.0, 0.0, self.focal_length) 
            - self.u() / 2.0
            - self.v() / 2.0
    }

    // Calculate the vectors across the horizontal and down the vertical viewport edges.

    pub fn u(&self) -> Vec3 { Vec3::new(self.viewport_width, 0.0, 0.0) }
    pub fn v(&self) -> Vec3 { Vec3::new(0.0, -self.viewport_height, 0.0) }
    
    // Calculate the horizontal and vertical delta vectors from pixel to pixel.
    
    pub fn delta_u(&self, direction: f64) -> Vec3 { self.u() / direction }
    pub fn delta_v(&self, direction: f64) -> Vec3 { self.v() / direction }

    pub fn center(&self) -> Point3 { self.center }
    pub fn focal_length(&self) -> f64 { self.focal_length }
    pub fn viewport_width(&self) -> f64 { self.viewport_width }
    pub fn viewport_height(&self) -> f64 { self.viewport_height }
    pub fn image_width(&self) -> i32 { self.image_width }
    pub fn image_height(&self) -> i32 { self.image_height }
}
