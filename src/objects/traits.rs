//! Public form traits with getters and common surface
//! methods.

use crate::vectors::Vec3;
use crate::materials::Material;

pub trait ObjectGetters {
    fn get_material(&self) -> Material;
    fn get_center(&self) -> Vec3;
}

/// Common surface methods.
pub trait SurfaceFunctions {
    /// Surface point based on two adimensional parameters ranging from 0 to 1.
    fn point(&self, s: f32, t: f32) -> Vec3;
    /// Surface normal based on two adimensional parameters ranging from 0 to 1.
    fn normal(&self, s: f32, t: f32) -> Vec3;
    /// Surface area.
    fn area(&self) -> f32;
    /// Surface area differential. NOTE!, it must be divided by
    /// the number of discretization points of both s and t parameters:
    ///
    /// $$\text{d}A=\frac{\text{diff\\_a}(s, t)}{N_sN_t}$$
    fn diff_a(&self, s: f32, t: f32) -> f32;
}
