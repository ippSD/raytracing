use crate::rays::Ray;
use crate::vectors::Vec3;
use crate::materials::Material;

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32, rec: &mut Option<HitRecord>) -> bool;
}

pub struct HitRecord{
    pub t: f32,
    pub p: Vec3,
    pub normal: Vec3,
    pub material: Material
}

