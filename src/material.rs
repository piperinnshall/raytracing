use crate::color::Color;
use crate::hit::HitRecord;
use crate::ray::Ray;
use crate::vec3::Vec3;

pub trait Material {
    fn scatter(&self, r_in: Ray, rec: HitRecord) -> Option<(Color, Ray)>;
}

pub struct Lambertian {
    albedo: Color,
}

pub struct Metal {
    albedo: Color,
    fuzz: f64,
}

pub struct Dielectric { 
    refraction_index: f64,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Self {
        Self { albedo }
    }
}

impl Metal {
    pub fn new(albedo: Color, fuzz: f64) -> Self {
        let fuzz = if fuzz < 1.0 { fuzz } else { 1.0 };
        Self { albedo, fuzz }
    }
}

impl Dielectric {
    pub fn new(refraction_index: f64) -> Self {
        Self { refraction_index }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _r_in: Ray, rec: HitRecord) -> Option<(Color, Ray)> {
        let scatter_direction = {
            let dir = rec.normal + Vec3::random_normalized();
            if dir.near_zero() {
                rec.normal
            } else {
                dir
            }
        };
        Some((self.albedo, Ray::new(rec.point, scatter_direction)))
    }
}

impl Material for Metal {
    fn scatter(&self, r_in: Ray, rec: HitRecord) -> Option<(Color, Ray)> {
        let reflected = r_in.direction().reflect(rec.normal).normalized()
            + (Vec3::random_normalized() * self.fuzz);

        let scattered = Ray::new(rec.point, reflected);
        if scattered.direction().dot(rec.normal) > 0.0 {
            Some((self.albedo, scattered))
        } else {
            None
        }
    }
}

impl Material for Dielectric {
    fn scatter(&self, r_in: Ray, rec: HitRecord) -> Option<(Color, Ray)> {
        let ri  = if rec.front_facing { 
            1.0 / self.refraction_index 
        } else { 
            self.refraction_index 
        };

        let unit_direction = r_in.direction().normalized();
        let refracted = unit_direction.refract(rec.normal, ri);

        Some((Color::fill(1.0), Ray::new(rec.point, refracted)))
    }
}
