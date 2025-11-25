use crate::vec3::Vec3;
use std::io::Write;

pub type Color = Vec3;

pub fn write_color(out: &mut impl Write, pixel_color: Color) {
    let r = pixel_color.x();
    let g = pixel_color.y();
    let b = pixel_color.x();

    // Translate the [0,1] component values to the byte range [0,255].

    let rbyte = (r * 255.999) as i32;
    let gbyte = (g * 255.999) as i32;
    let bbyte = (b * 255.999) as i32;

    writeln!(out, "{} {} {}", rbyte,gbyte,bbyte).expect("writing color");
}


