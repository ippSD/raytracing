use crate::vectors::{Vec3, Vec3Methods};
use crate::objects::{HittableList};
use crate::hittable::{HitRecord, Hittable};
extern crate rand;
use crate::materials::{Material, MaterialScatter};

pub struct Ray{
    pub a: Vec3,
    pub b: Vec3
}

impl Ray{
    pub fn new(a: Vec3, b: Vec3) -> Ray{ Ray {a, b } }
    pub fn origin(&self) -> Vec3{ self.a }
    pub fn direction(&self) -> Vec3{ self.b }
    pub fn point_at_parameter(&self, t: f32) -> Vec3{ self.origin() + self.direction() * t }
    pub fn color(&self, world: &HittableList, depth: usize) -> Vec3 {
        let mut rec: Option<HitRecord> = None;
        if world.hit(self, 1e-3, std::f32::MAX, &mut rec) {
            let mut scattered: Ray = Ray::new(Vec3::new(0e0, 0e0, 0e0), Vec3::new(0e0, 0e0, 0e0));
            let mut attenuation: Vec3 = Vec3::new(0e0, 0e0, 0e0);
            let hit_rec: HitRecord = rec.unwrap();
            let mat: Material = hit_rec.material;
            let do_scatter: bool = mat.scatter(&self, &hit_rec, &mut attenuation, &mut scattered);

            if depth < 50 && do_scatter {
                return scattered.color(world, depth+1) * attenuation;
            }
            else {
                return Vec3::zeros();
            }
        }
        else {
            let unit_direction: Vec3 = self.direction().unit_vector();
            let t: f32 = 5e-1 * (unit_direction.y() + 1e0);
            let col: Vec3 = Vec3::ones() * (1e0 - t) + Vec3::new(5e-1, 7e-1, 1e0) * t;
            //println!("t = {}, y = {}, col = ({})", t, unit_direction.y(), col);
            return col;
        }
    }
    /*
    pub fn hit_sphere(&self, center: Vec3, radius: f32) -> f32{
        let r: Vec3 = self.origin() - center;
        let a: f32 = self.direction().square_length();
        let b: f32 = 2e0 * r.dot(&self.direction());
        let c: f32 = r.square_length() - radius.powi(2);
        let discriminant: f32 = b.powi(2) - 4e0 * a * c;
        if discriminant <= 0e0 {
            return -1e0;
        }
        return (-b - discriminant.sqrt()) / (2e0 * a);
    }
    */
}