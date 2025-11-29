use crate::color::Color;
use crate::hit::HitRecord;
use crate::ray::Ray;
use crate::vec3::Vec3;

pub trait Material {
    fn scatter(&self, r_in: Ray, rec: HitRecord, attentuation: &mut Color, scattered: &mut Ray) -> bool;
}

struct Lambertian {
    albedo: Color,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Self {
        Self { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, r_in: Ray, rec: HitRecord, attentuation: &mut Color, scattered: &mut Ray) -> bool {
        let scatter_direction = {
            let dir = rec.normal + Vec3::random_normalized();
            if dir.near_zero() { rec.normal } else { dir }
        };

        *scattered = Ray::new(rec.point, scatter_direction);
        *attentuation = self.albedo;
        true
    }
}
