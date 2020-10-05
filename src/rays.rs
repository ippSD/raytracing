//! Ray class implementation.
//! Includes basic ray functions and ray spawning methods for
//! getting the ray color.
use std::borrow::{BorrowMut};

extern crate rand;

use crate::vectors::{Vec3, Vec3Methods};
use crate::objects::{HittableList};
use crate::hittable::{HitRecord, Hittable};
use crate::materials::{Material, MaterialScatter};

/// Background color.
const BACKGROUND_COLOR: Vec3 = Vec3::new_const(5e-1, 7e-1, 1e0);

/// Ray structure.
pub struct Ray{
    /// Ray origin.
    pub a: Vec3,
    /// Ray direction.
    pub b: Vec3
}

/// Ray member functions.
impl Ray{
    /// Ray constructor.
    ///
    /// # Parameters:
    ///
    /// * `a` - Ray origin.
    /// * `b` - Ray direction.
    pub fn new(a: Vec3, b: Vec3) -> Ray{ Ray {a, b } }

    /// Getter for ray origin.
    pub fn origin(&self) -> Vec3{ self.a }

    /// Getter for ray direction.
    pub fn direction(&self) -> Vec3{ self.b }

    /// Point crossed by the ray at ``t``:
    /// $$\vec P(t)=\vec A+\vec Bt$$
    pub fn point_at_parameter(&self, t: f32) -> Vec3{ self.origin() + self.direction() * t }

    /// Get the ray color.
    ///
    /// If the ray hits a world object, a new ray is spawned (based on the hit material)
    /// at least ``max_depth`` times.
    ///
    /// # Parameters:
    ///
    /// * `self` - current ray.
    /// * `world` - world of objects where the ray may hit.
    /// * `depth` - count of how many objects the original ray has hit up until the current ray.
    /// * `max_depth` - maximum depth value.
    ///
    /// # Returns:
    ///
    /// * `Vec3` - color of the ray after object reflections and refractions.
    pub fn color(&self, world: &HittableList, depth: usize, max_depth: usize) -> Vec3 {
        let mut rec: Option<HitRecord> = None;

        // Hit something on World.
        if world.hit(self, 1e-3, std::f32::MAX, &mut rec.borrow_mut()) {
            let mut scattered: Ray = Ray::new(Vec3::new(0e0, 0e0, 0e0), Vec3::new(0e0, 0e0, 0e0));
            let mut attenuation: Vec3 = Vec3::new(0e0, 0e0, 0e0);
            let hit_rec: HitRecord = rec.unwrap();
            let mat: Material = hit_rec.material;
            let do_scatter: bool = mat.scatter(&self, &hit_rec, &mut attenuation, &mut scattered);

            // New object hit, return color.
            if depth < max_depth && do_scatter {
                return scattered.color(world, depth+1, max_depth) * attenuation;
            }
            // Either max_depth reached or no-hit, return darkness.
            else {
                return Vec3::zeros();
            }
        }
        // Background gradient color.
        else {
            let unit_direction: Vec3 = self.direction().unit_vector();
            let t: f32 = 5e-1 * (unit_direction.y() + 1e0);
            let col: Vec3 = Vec3::ones() * (1e0 - t) + BACKGROUND_COLOR * t;
            return col;
        }
    }
}