use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};

struct HitList {
    shapes: Vec<Box<dyn Hittable>>,
}

impl HitList {
    pub fn new() -> Self { Self { shapes: Vec::new() } }

    pub fn clear(&mut self) { self.shapes.clear(); }

    pub fn add<T: Hittable + 'static>(&mut self, object: T) {
        self.shapes.push(Box::new(object));
    }
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, ray_tmin: f64, ray_tmax: f64, rec: &mut HitRecord) -> bool;
}

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

pub struct Sphere {
    center: Point3,
    radius: f64,
}

impl Sphere {
    pub fn new(center: Point3, radius: f64) -> Self {
        Self { center, radius: radius.max(0.0) }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, ray_tmin: f64, ray_tmax: f64, rec: &mut HitRecord) -> bool {
        let oc = self.center - ray.origin();
        let a = ray.direction().length_squared();
        let h = Vec3::dot(ray.direction(), oc);
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = h * h - a * c;
        if discriminant < 0.0 {
            return false;
        }

        let sqrtd = f64::sqrt(discriminant);

        // Find the nearest root that lies in the acceptable range.
        let mut root = (h - sqrtd) / a;
        if root < ray_tmin || ray_tmax <= root {
            root = (h + sqrtd) / a;
            if root <= ray_tmin || ray_tmax <= root {
                return false;
            }
        }

        rec.t = root;
        rec.point = ray.at(rec.t);
        let outward_normal = (rec.point - self.center) / self.radius;
        rec.set_face_normal(ray, outward_normal);

        true
    }
}
