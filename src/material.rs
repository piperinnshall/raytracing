use crate::color::Color;
use crate::hit::HitRecord;
use crate::ray::Ray;

pub trait Material {
    fn scatter(&self, r_in: Ray, rec: HitRecord, attentuation: Color, scattered: Ray) -> bool;
}
