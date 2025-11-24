use std::ops::{Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Sub};

#[derive(Clone, Copy, Default)]
pub struct Vec3 {
    e: [f64; 3],
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3 { e: [x, y, z] }
    }
    pub fn x(&self) -> f64 {
        self.e[0]
    }
    pub fn y(&self) -> f64 {
        self.e[1]
    }
    pub fn z(&self) -> f64 {
        self.e[2]
    }
}

pub type Point3 = Vec3;

// -Vec3
impl Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Self::Output {
        return Self::new(-self.x(), -self.y(), -self.z());
    }
}
// Vec3[i]
impl Index<usize> for Vec3 {
    type Output = f64;

    fn index(&self, index: usize) -> &Self::Output {
        &self.e[index]
    }
}
// Vec3[i] = 0
impl IndexMut<usize> for Vec3 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.e[index]
    }
}
//
// // Vec3 += Vec3
// impl AddAssign for Vec3 {
//     fn add_assign(&mut self, rhs: Self) {
//         *self = *self + rhs;
//     }
// }
// impl MulAssign for Vec3 {
//     fn mul_assign(&mut self, rhs: Self) {
//         *self = *self * rhs;
//     }
// }

macro_rules! impl_op {
    ($trait:ident, $func:ident, $op:tt, Vec3) => {
        impl $trait for Vec3 {
            type Output = Vec3;

            fn $func(self, rhs: Self) -> Self::Output {
                Self::new(
                    self.e[0] $op rhs.e[0],
                    self.e[1] $op rhs.e[1],
                    self.e[2] $op rhs.e[2],
                )
            }
        }
    };
    ($trait:ident, $func:ident, $op:tt, $rhs:ty) => {
        impl $trait<$rhs> for Vec3 {
            type Output = Vec3;

            fn $func(self, rhs: $rhs) -> Self::Output {
                Self::new(
                    self.e[0] $op rhs,
                    self.e[1] $op rhs,
                    self.e[2] $op rhs,
                )
            }
        }
    };
}

impl_op!(Add, add, +, Vec3); // Vec3 + Vec3
impl_op!(Sub, sub, -, Vec3); // Vec3 - Vec3
impl_op!(Mul, mul, *, Vec3); // Vec3 * Vec3
impl_op!(Mul, mul, *, f64); // Vec3 * f64
impl_op!(Div, div, /, f64); // Vec3 / f64

