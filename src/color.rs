use crate::{interval::Interval, vec3::Vec3};
use std::io::Write;

pub type Color = Vec3;

pub fn write_header(out: &mut impl Write, image_width: i32, image_height: i32) {
    writeln!(out, "P3\n{} {}\n255", image_width, image_height).expect("writing header");
}

pub fn write_color(out: &mut impl Write, pixel_color: Color) {
    let r = pixel_color.x();
    let g = pixel_color.y();
    let b = pixel_color.z();

    // Translate the [0,1] component values to the byte range [0,255].

    let intensity: Interval = Interval::new(0.000, 0.999);
    let rbyte = (intensity.clamp(r) * 256.0) as u8;
    let gbyte = (intensity.clamp(g) * 256.0) as u8;
    let bbyte = (intensity.clamp(b) * 256.0) as u8;

    writeln!(out, "{} {} {}", rbyte, gbyte, bbyte).expect("writing color");
}

pub fn lerp(c1: Color, c2: Color, t: f64) -> Color {
    c1 * (1.0 - t) + c2 * t
}
