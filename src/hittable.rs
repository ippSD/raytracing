//! Hit traits and structures for computing the ray-object
//! intersection on a world of objects.

use crate::rays::Ray;
use crate::vectors::Vec3;
use crate::materials::Material;

/// Hittable trait for computing ray-object hits.
pub trait Hittable {
    /// Returns whether `ray` hits any object on the world as well as
    /// a record of the hit.
    ///
    /// # Parameters:
    ///
    /// * `self`: World of objects.
    /// * `t_min`: Minimum allowed distance to the hit object.
    /// * `t_max`: Maximum allowed distance to the hit object.
    /// * `rec`: Hit record, if any, of the nearest object to the ray origin.
    ///
    /// # Returns:
    ///
    /// * `bool`: whether the ray hits any world object.
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32, rec: &mut Option<HitRecord>) -> bool;
}

/// Structure containing the hit information.
pub struct HitRecord{
    /// Ray's ``t`` parameter to hit-point.
    pub t: f32,
    /// Hit point.
    pub p: Vec3,
    /// Hit-point surface normal.
    pub normal: Vec3,
    /// Surface material.
    pub material: Material,
    /// Hit object's index on world.
    pub hit_elem: usize,
}
