use crate::color::Color;
use crate::hit::HitRecord;
use crate::ray::Ray;
use crate::vec3::Vec3;

pub trait Material {
    fn scatter(&self, r_in: Ray, rec: HitRecord) -> Option<(Color, Ray)>;
}

struct Lambertian {
    albedo: Color,
}

struct Metal {
    albedo: Color,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Self {
        Self { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _r_in: Ray, rec: HitRecord) -> Option<(Color, Ray)> {
        let scatter_direction = {
            let dir = rec.normal + Vec3::random_normalized();
            if dir.near_zero() { rec.normal } else { dir }
        };
        Some((self.albedo, Ray::new(rec.point, scatter_direction)))
    }
}

impl Metal {
    pub fn new(albedo: Color) -> Self {
        Self { albedo }
    }
}

impl Material for Metal {
    fn scatter(&self, r_in: Ray, rec: HitRecord) -> Option<(Color, Ray)> {
        let reflected = r_in.direction().reflect(rec.normal);
        Some((self.albedo, Ray::new(rec.point, reflected)))
    }
}

