//! 2D square form implementation.

use crate::vectors::{Vec3,Vec3Methods};
use crate::rays::Ray;
use crate::hittable::{HitRecord, Hittable};
use crate::materials::Material;
use crate::objects::traits::{SurfaceFunctions, ObjectGetters};


extern crate nalgebra;
use nalgebra::Matrix3;
use self::nalgebra::{Vector3};
extern crate rand;
use self::rand::random;

type M3 = Matrix3<f32>;
type V3 = Vector3<f32>;

/// Square structure.
pub struct Square {
    /// Square center.
    center: Vec3,
    /// Square edge size.
    length: f32,
    /// Square material.
    material: Material,
    /// Square's X-axis direction on world axes.
    u: Vec3,
    /// Square's Y-axis direction on world axes.
    v: Vec3,
    /// Square's Z-axis (surface normal) direction on world axes (note that $\vec w=\vec u\times\vec v$).
    w: Vec3,
}


/// Square function members.
impl Square {
    /// Square constructor with Z_axis pointing upwards [0, 1, 0] and random XZ face orientation.
    pub fn horizontal_surface(center: Vec3, length: f32, material: Material) -> Square {
        let w: Vec3 = Vec3::new(0e0, 1e0, 0e0);
        let ang: f32 = random::<f32>() * 8e0 * 1_f32.atan();
        let u: Vec3 = Vec3::new(ang.cos(), 0e0, ang.sin());
        let v: Vec3 = w.cross(&u);
        Square {center, length, material, u, v, w}
    }

    pub fn new(center: Vec3, length: f32, material: Material, u: Vec3, v: Vec3, w: Vec3) -> Square {
        Square {center, length, material, u, v, w}
    }
}

impl SurfaceFunctions for Square {
    /// Point on surface point from adimensional input parameters
    /// $s$ and $t$.
    /// # Parameters:
    /// * `s` - First adimensional parameter from 0 to 1.
    /// * `t` - Second adimensional parameter from 0 to 1.
    fn point(&self, s: f32, t: f32) -> Vec3 {
        let surf_center: Vec3 = self.center;
        let point_on_surf_s: Vec3 = self.length * (s - 5e-1) * self.u;
        let point_on_surf_t: Vec3 = self.length * (t - 5e-1) * self.v;
        return surf_center + point_on_surf_s + point_on_surf_t;
    }

    fn normal(&self, _s: f32, _t: f32) -> Vec3 {
        return self.w;
    }

    fn area(&self) -> f32 {
        return self.length.powi(2);
    }

    fn diff_a(&self, _s: f32, _t: f32) -> f32 {
        return self.area();
    }
}

impl ObjectGetters for Square {
    fn get_material(&self) -> Material { self.material }

    fn get_center(&self) -> Vec3 { self.center }
}


/// Hittable trait on square.
impl Hittable for Square {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32, rec: &mut Option<HitRecord>) -> bool{
        let mut n_pp: Vec3 = Vec3::random();
        let n: Vec3 = self.normal(0e0, 0e0);

        // Ray belongs to the surface, no collision.
        if (n.dot(&ray.direction())).abs() < 1e-3 {
            return false;
        }

        // Random non-parallel vector to ray direction.
        while n_pp.dot(&ray.direction()).abs() > 0.8 {
            n_pp = Vec3::random();
        }

        // Random directions perpendicular to ray direction.
        let n_1: Vec3 = (n_pp.cross(&ray.direction())).unit_vector();
        let n_2: Vec3 = (ray.direction().cross(&n_1)).unit_vector();

        let a: M3 = M3::new(
            n.e[0], n.e[1], n.e[2],
            n_1.e[0], n_1.e[1], n_1.e[2],
            n_2.e[0], n_2.e[1], n_2.e[2]
        );
        let b: V3 = V3:: new(
            n.dot(&self.center),
            n_1.dot(&ray.origin()),
            n_2.dot(&ray.origin())
        );
        let p_v3: V3 = a.lu().solve(&b).unwrap();
        let p: Vec3 = Vec3::new(p_v3.x, p_v3.y, p_v3.z);
        let t: f32 = (p - ray.origin()).dot(&ray.direction()) / ray.direction().square_length();

        // Unreachable collision.
        if t < t_min || t > t_max {
            return false;
        }

        // Out of plane bounds (1).
        if (p - self.center).dot(&self.u).abs() > self.length / 2e0 {
            return false;
        }

        // Out of plane bounds (2).
        if (p - self.center).dot(&self.v).abs() > self.length / 2e0 {
            return false;
        }

        // Collision.
        let rec2 = HitRecord {
            t,
            p,
            normal: -n * n.dot(&ray.direction()) / (n.dot(&ray.direction())).abs(),
            material: self.material,
            hit_elem: 0
        };
        *rec = Some(rec2);

        return true;
    }
}
