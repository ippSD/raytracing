//! Cube form implementation.

use crate::vectors::{Vec3,Vec3Methods};
use crate::rays::Ray;
use crate::hittable::{HitRecord, Hittable};
use crate::materials::Material;
use crate::objects::square::Square;
use crate::objects::sphere::Sphere;
use crate::objects::traits::{SurfaceFunctions, ObjectGetters};

extern crate nalgebra;
use nalgebra::{Matrix3, Vector3};

extern crate rand;
use self::rand::random;

type M3 = Matrix3<f32>;
type V3 = Vector3<f32>;

/// Cube structure.
pub struct Cube{
    /// Cube center.
    center: Vec3,
    /// Cube edge size.
    length: f32,
    /// Cube material.
    material: Material,
    /// Cube's X-axis direction on world axes.
    u: Vec3,
    /// Cube's Y-axis direction on world axes.
    v: Vec3,
    /// Cube's Z-axis direction on world axes (note that $\vec w=\vec u\times\vec v$).
    w: Vec3,
}

/// Cube surface identifier.
pub enum CubeSurface {
    XN = 0,
    XP = 1,
    YN = 2,
    YP = 3,
    ZN = 4,
    ZP = 5
}

/// Cube function members.
impl Cube{
    /// Cube constructor with Y_axis pointing upwards and random XZ face orientation.
    pub fn new(center: Vec3, length: f32, material: Material) -> Cube {
        let w: Vec3 = Vec3::new(0e0, 1e0, 0e0);
        let ang: f32 = random::<f32>() * 8e0 * 1_f32.atan();
        let u: Vec3 = Vec3::new(ang.cos(), 0e0, ang.sin());
        let v: Vec3 = w.cross(&u);
        Cube {center, length, material, u, v, w}
    }

    pub fn normal(surface_identifier: &CubeSurface) -> Vec3 {
        return match surface_identifier {
            CubeSurface::XN => Vec3::new(-1e0,  0e0,  0e0),
            CubeSurface::XP => Vec3::new( 1e0,  0e0,  0e0),
            CubeSurface::YN => Vec3::new( 0e0, -1e0,  0e0),
            CubeSurface::YP => Vec3::new( 0e0,  1e0,  0e0),
            CubeSurface::ZN => Vec3::new( 0e0,  0e0, -1e0),
            CubeSurface::ZP => Vec3::new( 0e0,  0e0,  1e0),
        }
    }

    pub fn get_square(&self, surface_identifier: &CubeSurface) -> Square {
        let w: Vec3 = Cube::normal(surface_identifier);
        let center: Vec3 = self.center + self.length / 2e0 * w;
        let length: f32 = self.length;
        let material: Material = self.material;
        let u: Vec3;
        let v: Vec3;
        match surface_identifier {
            CubeSurface::XN => {
                u = Cube::normal(&CubeSurface::YN);
                v = Cube::normal(&CubeSurface::ZN);
            },
            CubeSurface::XP => {
                u = Cube::normal(&CubeSurface::YP);
                v = Cube::normal(&CubeSurface::ZP);
            },
            CubeSurface::YN => {
                u = Cube::normal(&CubeSurface::ZN);
                v = Cube::normal(&CubeSurface::XN);
            },
            CubeSurface::YP => {
                u = Cube::normal(&CubeSurface::ZP);
                v = Cube::normal(&CubeSurface::XP);
            },
            CubeSurface::ZN => {
                u = Cube::normal(&CubeSurface::XN);
                v = Cube::normal(&CubeSurface::YN);
            },
            CubeSurface::ZP => {
                u = Cube::normal(&CubeSurface::XP);
                v = Cube::normal(&CubeSurface::YP);
            },
        }

        return Square::new(
            center,
            length,
            material,
            u,
            v,
            w
        );
    }

    /// Cube's surface point from input adimensional parameters
    /// $s$ and $t$ and cube's surface identifier.
    /// # Parameters:
    /// * `s` - First adimensional parameter from 0 to 1.
    /// * `t` - Second adimensional parameter from 0 to 1.
    /// * `sid` - Cube's surface identifier (X, Y or Z, plus or minus surface).
    pub fn point(&self, s: f32, t: f32, sid: &CubeSurface) -> Vec3 {
        return self.get_square(sid).point(s, t);
        // let surf_center: Vec3 = self.center + self.length / 2e0 * self::normal(sid);
        // let point_on_surf_s: Vec3 = self.length * self::normal((sid + 2) % 6) * (s - 5e-1);
        // let point_on_surf_t: Vec3 = self.length * self::normal((sid + 4) % 6) * (t - 5e-1);
        // return surf_center + point_on_surf_s + point_on_surf_t;
    }

}

impl ObjectGetters for Cube {
    fn get_material(&self) -> Material { self.material }

    fn get_center(&self) -> Vec3 { self.center }
}

impl Hittable for Cube{
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32, rec: &mut Option<HitRecord>) -> bool{
        let mut do_hit: bool = false;
        let mut t_closest: f32 = t_max;
        let sphere: Sphere = Sphere::new(self.center, self.length * 3e0f32.sqrt() / 2e0, self.material);
        if sphere.hit(ray, t_min, t_max, rec) {
            let signs = [-1e0f32, 1e0f32];
            for i in 0..3 {
                for si in signs.iter() {
                    let mut m: M3 = M3::zeros();
                    let mut ei: V3 = V3::zeros();
                    let mut ej: V3 = V3::zeros();
                    let mut ek: V3 = V3::zeros();
                    let minus_b: V3 = -V3::new(ray.direction().x(), ray.direction().y(), ray.direction().z());
                    match i {
                        0 => {
                            ei = V3::new(self.u.e[0], self.u.e[1], self.u.e[2]);
                            ej = V3::new(self.v.e[0], self.v.e[1], self.v.e[2]);
                            ek = V3::new(self.w.e[0], self.w.e[1], self.w.e[2]);
                        },
                        1 => {
                            ek = V3::new(self.u.e[0], self.u.e[1], self.u.e[2]);
                            ei = V3::new(self.v.e[0], self.v.e[1], self.v.e[2]);
                            ej = V3::new(self.w.e[0], self.w.e[1], self.w.e[2]);
                        },
                        2 => {
                            ej = V3::new(self.u.e[0], self.u.e[1], self.u.e[2]);
                            ek = V3::new(self.v.e[0], self.v.e[1], self.v.e[2]);
                            ei = V3::new(self.w.e[0], self.w.e[1], self.w.e[2]);
                        },
                        _ => {}
                    }
                    ei.scale_mut(*si);
                    m.set_column(0, &ej);
                    m.set_column(1, &ek);
                    m.set_column(2, &minus_b);

                    let det: f32 = m.determinant();

                    if det.abs() > 1e-4 {
                        let mut k: V3 = V3::new(ray.origin().x(), ray.origin().y(), ray.origin().z());
                        k -= V3::new(self.center.x(), self.center.y(), self.center.z());
                        k -= ei.scale(self.length / 2e0);
                        let mut x: V3 = V3::zeros();
                        m.pseudo_inverse(1e-5).unwrap().mul_to(&k, &mut x);
                        let check_xj: bool = x[0].abs() < self.length / 2e0;
                        let check_xk: bool = x[1].abs() < self.length / 2e0;
                        let check_t: bool = t_min < x[2] && x[2] < t_closest;
                        if check_xj && check_xk && check_t {
                            t_closest = x[2];
                            do_hit = true;
                            let rec2 = HitRecord {
                                t: t_closest,
                                p: ray.point_at_parameter(t_closest),
                                normal: Vec3::new(ei[0], ei[1], ei[2]),
                                material: self.material,
                                hit_elem: 0
                            };
                            *rec = Some(rec2);
                        }
                    }
                }
            }
        }
        return do_hit;
    }
}

/*
impl Hittable for Cube{
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32, rec: &mut Option<HitRecord>) -> bool{
        let mut do_hit: bool = false;
        let mut t_closest: f32 = t_max;
        let mut trans_mat: M3 = M3::zeros();
        let mut a: V3;
        let mut b: V3;
        trans_mat.set_column(0, &(self.u.into()));
        trans_mat.set_column(0, &(self.v.into()));
        trans_mat.set_column(0, &(self.w.into()));
        for i in 0..3{
            match i {
                1 => {
                    trans_mat.swap_columns(0,1);
                    trans_mat.swap_columns(1,2);
                },
                2 => {
                    trans_mat.swap_columns(0,2);
                    trans_mat.swap_columns(1,2);
                },
                _ => (),
            }

            a = (ray.origin() - self.center).into();
            //a.transform_vector(trans_mat.transpose());
            //trans_mat.transpose().rotate(&mut a);
            a = trans_mat.transpose() * a;
            b = ray.direction().into();
            //b.transform_vector(trans_mat.transpose());
            //trans_mat.transpose().rotate(&mut b);
            b = trans_mat.transpose() * b;
            if b[0].abs() > 1e-5 {
                let mut p: V3;
                for si in [-1f32, 1f32].iter() {
                    let t_temp: f32 = ((*si) * self.length / 2e0 - a[0]) / b[0];
                    p = a + b * t_temp;
                    if p[1].abs() < self.length / 2e0 && p[2].abs() < self.length / 2e0{
                        //p.transform_vector(trans_mat);
                        //let t_temp: f32 = (p - a_centered).dot(&(ray.direction().into())) / b_c.norm_squared();
                        if t_min < t_temp && t_temp < t_closest {
                            t_closest = t_temp;
                            do_hit = true;
                            let rec2 = HitRecord {
                                t: t_closest,
                                p: ray.point_at_parameter(t_closest),
                                normal: Vec3::new(trans_mat[(0,0)], trans_mat[(1,0)], trans_mat[(2,0)]) * (*si),
                                material: self.material,
                            };
                            *rec = Some(rec2);
                        }
                    }
                }
            }
        }
        return do_hit;
    }
}
*/
