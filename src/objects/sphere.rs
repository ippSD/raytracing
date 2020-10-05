//! Sphere form implementation.

use std::f32::consts::PI;

use crate::vectors::{Vec3,Vec3Methods};
use crate::rays::Ray;
use crate::hittable::{HitRecord, Hittable};
use crate::materials::Material;
use crate::objects::traits::{SurfaceFunctions, ObjectGetters};


/// Sphere structure.
pub struct Sphere{
    /// Sphere center.
    center: Vec3,
    /// Sphere radius.
    radius: f32,
    /// Sphere material.
    material: Material,
}

/// Sphere function members.
impl Sphere{
    /// Sphere constructor.
    pub fn new(center: Vec3, radius: f32, material: Material) -> Sphere { Sphere {center, radius, material} }
}

impl SurfaceFunctions for Sphere {
    /// Point on sphere at longitude $\lambda$ and latitude $\phi$:
    /// $$\vec P(\vec C, r, \lambda,\phi)=\vec C + R_{Z}(\lambda)\cdot R_{Y}(-\phi)\cdot\left\\{r,0,0\right\\}$$
    fn point(&self, s: f32, t: f32) -> Vec3{
        self.center + self.radius * self.normal(s, t)
    }

    /// Surface normal at surface point defined by the adimensional parameters
    /// $s$ and $t$, so that:
    /// $$\lambda=2\pi s$$
    /// $$\phi=\pi\left(t-\frac{1}{2}\right)$$
    ///
    /// # Parameters:
    /// * `s` - Longitude parameter from 0 to 1.
    /// * `t` - Latitude parameter from 0 to 1.
    ///
    /// # Returns:
    /// * `Vec3` - Surface normal.
    fn normal(&self, s: f32, t:f32) -> Vec3 {
        let lambda: f32 = 2e0 * PI * s;
        let phi: f32 = PI * (t - 5e-1);
        return Vec3::new(lambda.cos()*phi.cos(), lambda.sin()*phi.cos(), phi.sin());
    }

    fn area(&self) -> f32 {
        return 4e0 * PI * self.radius.powi(2);
    }

    fn diff_a(&self, _s: f32, t: f32) -> f32 {
        let phi: f32 = PI * (t - 5e-1);
        return self.radius.powi(2) * phi.cos() * 2e0 * PI.powi(2);
    }
}

impl ObjectGetters for Sphere {
    fn get_material(&self) -> Material { self.material }

    fn get_center(&self) -> Vec3 { self.center }
}

/// Hittable trait on sphere.
impl Hittable for Sphere{
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32, rec: &mut Option<HitRecord>) -> bool{
        // Ray relative to sphere center.
        let r: Vec3 = ray.origin() - self.center;
        let a: f32 = ray.direction().square_length();
        let b: f32 = 2e0 * r.dot(&ray.direction());
        let c: f32 = r.square_length() - self.radius.powi(2);
        let discriminant: f32 = b.powi(2) - 4e0 * a * c;

        // One or two possible generic solutions.
        if discriminant > 0e0 {
            let t1: f32 = (-b - discriminant.sqrt()) / (2e0 * a);
            let t2: f32 = (-b + discriminant.sqrt()) / (2e0 * a);

            let mut t_op: Option<f32> = None;
            // t2 is solution.
            if (t_min < t2) && (t2 < t_max) {
                t_op = Some(t2);
            }
            // t1 is solution.
            if (t_min < t1) && (t1 < t_max) {
                t_op = Some(t1);
            }
            let ans = match t_op {
                // No solutions between t_min and t_max.
                None => false,
                // Solution (t1 or t2).
                Some(_t) => true,
            };

            // Return hit record.
            if ans {
                let rec2 = HitRecord{
                    t: t_op.unwrap(),
                    p: ray.point_at_parameter(t_op.unwrap()),
                    normal: (ray.point_at_parameter(t_op.unwrap()) - self.center) / self.radius,
                    material: self.material,
                    hit_elem: 0
                };
                *rec = Some(rec2);
            }
            return ans;
        }
        return false;
    }
}

/*
impl Fov for Sphere{
    fn fov(&self, obj: &Self) -> f32{
        let mut lambda: f32;
        let mut phi: f32;
        let mut lambda2: f32;
        let mut phi2: f32;
        let mut cos_beta_1: f32;
        let mut cos_beta_2: f32;
        let mut r_12: f32;
        let mut ray: Ray;
        let mut sigma: f32 = 0e0;
        const NL: usize = 50;
        const NP: usize = 50;
        //const NS: usize = 100;
        for i in 0..NL {
            lambda = (i as f32) * 8e0 * 1f32.atan() / (NL as f32);
            for j in 0..NP {
                phi = ((j as f32) - (NP as f32)/2e0) * 2e0 * 1f32.atan() / ((NP/2) as f32);
                for i2 in 0..NL {
                    lambda2 = (i2 as f32) * 8e0 * 1f32.atan() / (NL as f32);
                    for j2 in 0..NP {
                        phi2 = ((j2 as f32) - (NP as f32)/2e0) * 2e0 * 1f32.atan() / ((NP/2) as f32);
                        ray = Ray::new(
                            self.point(lambda, phi),
                            obj.point(lambda2, phi2) - self.point(lambda, phi)
                        );
                        let mut rec: Option<HitRecord> = None;
                        cos_beta_1 = ray.direction().unit_vector().dot(&((ray.origin() - self.center).unit_vector()));
                        if cos_beta_1 > 0e0 && !self.hit(&ray, 1e-6, std::f32::MAX, &mut rec) && obj.hit(&ray, 0f32, std::f32::MAX, &mut rec) {
                            let rrec: HitRecord = rec.unwrap();
                            //println!("t: {}", rrec.t);
                            cos_beta_2 = - ray.direction().unit_vector().dot(rrec.normal.borrow());
                            r_12 = (ray.point_at_parameter(rrec.t) - ray.origin()).length();
                            println!("C1: {}, C2: {}, R12: {}", cos_beta_1, cos_beta_2, r_12);
                            sigma += cos_beta_1 * cos_beta_2 / (4f32 * 1f32.atan() * r_12.powi(2));
                        }
                    }
                }
            }
        }
        sigma *= 4e0 * (4e0 * 1f32.atan()) * obj.radius.powi(2);
        //match obj {
        //    Form::Sphere(sphere) => sigma *= 4e0 * (4e0 * 1f32.atan()) * sphere.radius.powi(2),
        //    Form::Cube(cube) => sigma *= cube.length.powi(2) * 6e0,
        //}
        sigma / ((NL * NP) as f32).powi(2)
    }
}
*/