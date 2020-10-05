//! 3D Vector implementation, either for points or colors.
//! Class Vec3 contains:
//! * `Constructors/Initializators`: new, const_new, zeros, ones, random.
//! * `Basic math`: +, -, *, /, +=, -=, *=, /=.
//! * `Getters`: x, y, z (for points), r, g, b (for colors).
//! * `Vector ops`: length, square_length, dot, cross, unit_vector, make_unit_vector.
//! * `Color ops`: gamma_2, gamma_3.
//! * `Others`: powi, max, is_nan.
//!

use std::ops::Add;
use std::ops::AddAssign;
use std::ops::Sub;
use std::ops::SubAssign;
use std::ops::Neg;
use std::ops::Index;
use std::ops::Mul;
use std::ops::MulAssign;
use std::ops::Div;
use std::ops::DivAssign;
use std::fmt;
use std::convert::{Into, From};
extern crate rand;
use rand::Rng;
extern crate nalgebra;
use self::nalgebra::Vector3;

type V3 = Vector3<f32>;

#[derive(Copy,Clone)]
pub struct Vec3{
    pub e: [f32; 3]
}

impl Vec3{
    pub fn new(e0: f32, e1: f32, e2:f32) -> Vec3 {
        Vec3 { e: [e0, e1, e2] }
    }
    pub const fn new_const(e0: f32, e1: f32, e2:f32) -> Vec3 {
        Vec3 { e: [e0, e1, e2] }
    }
    pub fn zeros() -> Vec3 { Vec3 { e: [0e0, 0e0, 0e0] } }
    pub fn ones() -> Vec3 { Vec3 { e: [1e0, 1e0, 1e0] } }
    pub fn random() -> Vec3 {
        let mut rng = rand::thread_rng();
        Vec3::new(rng.gen::<f32>(), rng.gen::<f32>(), rng.gen::<f32>())
    }
}

impl Vec3Methods for Vec3{
    fn x(&self) -> f32 { self.e[0] }
    fn y(&self) -> f32 { self.e[1] }
    fn z(&self) -> f32 { self.e[2] }
    fn r(&self) -> f32 { self.x() }
    fn g(&self) -> f32 { self.y() }
    fn b(&self) -> f32 { self.z() }
    fn length(&self) -> f32 { self.square_length().sqrt() }
    fn square_length(&self) -> f32 { self.dot(self) }
    fn gamma2(&mut self) { for i in 0..3 { self.e[i] = self.e[i].sqrt(); } }
    fn gamma3(&mut self) { for i in 0..3 { self.e[i] = self.e[i].cbrt(); } }
    fn make_unit_vector(mut self) { self /= self.length(); }
    fn unit_vector(&self) -> Vec3 { Vec3 { e: self.e } / self.length() }
    fn dot(&self, v2: &Vec3) -> f32 { self.x() * v2.x() + self.y() * v2.y() + self.z() * v2.z() }
    fn cross(&self, v2: &Vec3) -> Vec3 {
        Vec3 {
            e: [
                self.y() * v2.z() - self.z() * v2.y(),
                self.z() * v2.x() - self.x() * v2.z(),
                self.x() * v2.y() - self.y() * v2.x()
            ]
        }
    }
    fn is_nan(&self) -> bool{ self.e[0].is_nan() || self.e[1].is_nan() || self.e[2].is_nan() }
    fn powi(&self, i: i32) -> Vec3{ Vec3::new(self.x().powi(i), self.y().powi(i), self.z().powi(i))}
    fn max(&self) -> f32{
        let mut max: f32 = self.e[0];
        for i in 1..3 {
            if self.e[i] > max { max = self.e[i]; }
        }
        max
    }
}

impl Add for Vec3{
    type Output = Vec3;

    fn add(self, v2: Vec3) -> Vec3 {
        Vec3 { e: [self.x() + v2.x(), self.y() + v2.y(), self.z() + v2.z()] }
    }
}

impl Neg for Vec3{
    type Output = Vec3;

    fn neg(self) -> Vec3 {
        Vec3 { e: [-self.x(), -self.y(), -self.z()] }
    }
}

impl Sub for Vec3{
    type Output = Vec3;

    fn sub(self, v2: Vec3) -> Vec3 {
        self.add(v2.neg())
    }
}

impl Mul for Vec3{
    type Output = Vec3;

    fn mul(self, v: Vec3) -> Vec3 {
        Vec3 { e: [self.x() * v.x(), self.y() * v.y(), self.z() * v.z()] }
    }
}

impl Mul<f32> for Vec3{
    type Output = Vec3;

    fn mul(self, m: f32) -> Vec3 {
        Vec3 { e: [self.x() * m, self.y() * m, self.z() * m] }
    }
}

impl Mul<Vec3> for f32{
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3 { e: [rhs.x() * self, rhs.y() * self, rhs.z() * self] }
    }
}

impl Div<f32> for Vec3{
    type Output = Vec3;

    fn div(self, d: f32) -> Vec3 {
        self.mul(1e0/d)
    }
}

impl Index<usize> for Vec3{
    type Output = f32;

    fn index(&self, key: usize) -> &f32 {
        &self.e[key]
    }
}

impl AddAssign for Vec3{
    fn add_assign(&mut self, v2: Self) { for i in 0..3 {self.e[i] += v2.e[i]; } }
}

impl SubAssign for Vec3{
    fn sub_assign(&mut self, v2: Self) { for i in 0..3 {self.e[i] -= v2.e[i]; } }
}

impl MulAssign for Vec3{
    fn mul_assign(&mut self, v2: Self) { for i in 0..3 {self.e[i] *= v2.e[i]; } }
}

impl DivAssign for Vec3{
    fn div_assign(&mut self, v2: Self) { for i in 0..3 {self.e[i] /= v2.e[i]; } }
}

impl MulAssign<f32> for Vec3{
    fn mul_assign(&mut self, t: f32) { for i in 0..3 {self.e[i] *= t; } }
}

impl DivAssign<f32> for Vec3{
    fn div_assign(&mut self, t: f32) { for i in 0..3 {self.e[i] /= t; } }
}

impl fmt::Display for Vec3{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {} {}", self.x(), self.y(), self.z())
    }
}

impl Into<V3> for Vec3{
    fn into(self) -> V3 {
        V3::new(self.x(), self.y(), self.z())
    }
}

impl From<V3> for Vec3{
    fn from(v: V3) -> Self {
        Vec3::new(v[0], v[1], v[2])
    }
}

pub trait Vec3Methods {
    fn x(&self) -> f32;
    fn y(&self) -> f32;
    fn z(&self) -> f32;
    fn r(&self) -> f32;
    fn g(&self) -> f32;
    fn b(&self) -> f32;
    fn length(&self) -> f32;
    fn square_length(&self) -> f32;
    fn gamma2(&mut self);
    fn gamma3(&mut self);
    fn make_unit_vector(self);
    fn unit_vector(&self) -> Vec3;
    fn dot(&self, v2: &Vec3) -> f32;
    fn cross(&self, v2: &Vec3) -> Vec3;
    fn is_nan(&self) -> bool;
    fn powi(&self, i: i32) -> Vec3;
    fn max(&self) -> f32;
}