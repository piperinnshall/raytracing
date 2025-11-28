use std::fmt::{Display, Formatter, Result};
use std::ops::{Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Sub};

use crate::utils;

#[derive(Clone, Copy)]
pub struct Vec3 {
    e: [f64; 3],
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { e: [x, y, z] }
    }

    pub fn fill(n: f64) -> Self {
        Self::new(n, n, n)
    }

    pub fn random() -> Self {
        Self::new(
            utils::random_f64(),
            utils::random_f64(),
            utils::random_f64(),
        )
    }

    pub fn random_range(min: f64, max: f64) -> Self {
        Self::new(
            utils::random_range_f64(min, max),
            utils::random_range_f64(min, max),
            utils::random_range_f64(min, max),
        )
    }

    pub fn random_normalized() -> Self {
        loop {
            let p = Self::random_range(-1.0, 1.0);
            let lensq = p.length_squared();
            if 1e-160 < lensq && lensq <= 1.0 {
                break p / lensq.sqrt();
            }
        }
    }

    pub fn random_on_hemisphere(normal: Self) -> Self {
        let on_unit_sphere = Self::random_normalized();
        // In the same hemisphere as normal
        if on_unit_sphere.dot(normal) > 0.0 {
            on_unit_sphere
        } else {
            -on_unit_sphere
        }
    }

    pub fn dot(self, rhs: Self) -> f64 {
        self[0] * rhs[0] + self[1] * rhs[1] + self[2] * rhs[2]
    }

    pub fn cross(self, rhs: Self) -> Self {
        Self::new(
            self[1] * rhs[2] - self[2] * rhs[1],
            self[2] * rhs[0] - self[0] * rhs[2],
            self[0] * rhs[1] - self[1] * rhs[0],
        )
    }

    pub fn normalized(self) -> Self {
        self / self.length()
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn length_squared(&self) -> f64 {
        self[0] * self[0] + self[1] * self[1] + self[2] * self[2]
    }

    pub fn x(&self) -> f64 {
        self[0]
    }
    pub fn y(&self) -> f64 {
        self[1]
    }
    pub fn z(&self) -> f64 {
        self[2]
    }
}

pub type Point3 = Vec3;

impl Default for Vec3 {
    fn default() -> Self {
        Self::new(0.0, 0.0, 0.0)
    }
}

impl Display for Vec3 {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{} {} {}", self[0], self[1], self[2])
    }
}

macro_rules! impl_op {
    ($trait:ident, $func:ident, $op:tt) => {
        impl $trait for Vec3 {
            type Output = Self;

            fn $func(self, rhs: Self) -> Self::Output {
                Self::new(self[0] $op rhs[0], self[1] $op rhs[1], self[2] $op rhs[2])
            }
        }
    };
    ($trait:ident, $func:ident, $scalar:ty, $body:expr) => {
        impl $trait<$scalar> for Vec3 {
            type Output = Self;

            fn $func(self, rhs: $scalar) -> Self::Output {
                $body(self,rhs)
            }
        }
    };
}

// Vec3 op Vec3
impl_op!(Sub, sub, -);
impl_op!(Add, add, +);
impl_op!(Mul, mul, *);

// Vec3 op $scalar
impl_op!(Mul, mul, f64, |s: Self, r| Self::new(
    s[0] * r,
    s[1] * r,
    s[2] * r
));
impl_op!(Div, div, f64, |s: Self, r| s * (1.0 / r));

impl AddAssign for Vec3 {
    // Vec3 += Vec3
    fn add_assign(&mut self, rhs: Self) {
        self[0] += rhs[0];
        self[1] += rhs[1];
        self[2] += rhs[2];
    }
}

impl MulAssign<f64> for Vec3 {
    // Vec3 *= f64
    fn mul_assign(&mut self, rhs: f64) {
        self[0] *= rhs;
        self[1] *= rhs;
        self[2] *= rhs;
    }
}

impl DivAssign<f64> for Vec3 {
    // Vec3 /= f64
    fn div_assign(&mut self, rhs: f64) {
        *self *= 1.0 / rhs
    }
}

impl Neg for Vec3 {
    // -Vec3
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self::new(-self[0], -self[1], -self[2])
    }
}

impl Index<usize> for Vec3 {
    // Vec3[i]
    type Output = f64;

    fn index(&self, index: usize) -> &Self::Output {
        &self.e[index]
    }
}

impl IndexMut<usize> for Vec3 {
    // Vec3[i] = 0
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.e[index]
    }
}
