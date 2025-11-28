use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};

#[derive(Clone, Default)]
pub struct HitRecord {
    pub normal: Vec3,
    pub point: Point3,
    pub t: f64,
    pub front_facing: bool,
}

impl HitRecord {
    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: Vec3) {
        // Sets the hit record normal vector.
        // NOTE: the parameter `outward_normal` is assumed to have unit length.

        self.front_facing = Vec3::dot(ray.direction(), outward_normal) < 0.0;
        self.normal = if self.front_facing { outward_normal } else { -outward_normal };
    }
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, ray_tmin: f64, ray_tmax: f64, rec: &mut HitRecord) -> bool;
}

pub struct HitList {
    shapes: Vec<Box<dyn Hittable>>,
}

impl HitList {
    pub fn new() -> Self { Self { shapes: Vec::new() } }
    pub fn clear(&mut self) { self.shapes.clear(); }
    pub fn add<T: Hittable + 'static>(&mut self, object: T) {
        self.shapes.push(Box::new(object));
    }
}

impl Hittable for HitList {
    fn hit(&self, ray: &Ray, ray_tmin: f64, ray_tmax: f64, rec: &mut HitRecord) -> bool {
        let mut temp_rec = HitRecord {
            point: Point3::default(),
            normal: Vec3::default(),
            t: 0.0,
            front_facing: false,
        };

        let mut hit_anything = false;
        let mut closest_so_far = ray_tmax;

        for shape in &self.shapes {
            if shape.hit(ray, ray_tmin, closest_so_far, &mut temp_rec) {
                hit_anything = true;
                closest_so_far = temp_rec.t;
                *rec = temp_rec.clone();
            }
        }

        hit_anything
    }
}

