use crate::vec3::{Vec3,Point3};
use crate::ray::Ray;

pub struct Sphere {
    pub center: Point3,
    pub radius: f64,
}

pub fn hit(sphere: &Sphere, ray: &Ray) -> bool {
    let oc = sphere.center - ray.origin();
    let a = Vec3::dot(ray.direction(), ray.direction());
    let b = Vec3::dot(ray.direction(), oc) * -2.0;
    let c = Vec3::dot(oc, oc) - sphere.radius * sphere.radius;
    b*b - 4.0*a*c >= 0.0
}
