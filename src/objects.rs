use crate::vectors::{Vec3,Vec3Methods};
use crate::rays::Ray;
use crate::hittable::{HitRecord, Hittable};
use crate::materials::Material;
use std::borrow::Borrow;
extern crate nalgebra;
use nalgebra::Matrix3;
use self::nalgebra::{Vector3};

type M3 = Matrix3<f32>;
type V3 = Vector3<f32>;

pub enum Form{
    Sphere(Sphere),
    Cube(Cube),
}

pub struct Sphere{
    center: Vec3,
    radius: f32,
    material: Material,
}

pub struct Cube{
    center: Vec3,
    length: f32,
    material: Material,
    u: Vec3,
    v: Vec3,
    w: Vec3,
}

pub struct HittableList{
    pub forms: Vec<Form>
}

impl Form{
    pub fn material(&self) -> Material{
        return match self {
            Form::Sphere(sphere) => sphere.material,
            Form::Cube(cube) => cube.material,
        }
    }
}

impl Sphere{
    pub fn new(center: Vec3, radius: f32, material: Material) -> Sphere { Sphere {center, radius, material} }
}

impl Cube{
    pub fn new(center: Vec3, length: f32, material: Material) -> Cube {
        let w: Vec3 = Vec3::new(0e0, 1e0, 0e0);
        let mut u: Vec3 = Vec3::zeros();
        while u.cross(&w).length() < 1e-3 {
            u = Vec3::random().cross(&w).unit_vector();
        }
        let v: Vec3 = w.cross(&u);
        Cube {center, length, material, u, v, w}
    }
}

impl HittableList{
    pub fn new() -> HittableList { HittableList {forms: Vec::new()} }
}

impl Hittable for Sphere{
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32, rec: &mut Option<HitRecord>) -> bool{
        let r: Vec3 = ray.origin() - self.center;
        let a: f32 = ray.direction().square_length();
        let b: f32 = 2e0 * r.dot(&ray.direction());
        let c: f32 = r.square_length() - self.radius.powi(2);
        let discriminant: f32 = b.powi(2) - 4e0 * a * c;
        if discriminant > 0e0 {
            let t1: f32 = (-b - discriminant.sqrt()) / (2e0 * a);
            let t2: f32 = (-b + discriminant.sqrt()) / (2e0 * a);
            let mut t_op: Option<f32> = None;
            if (t_min < t2) && (t2 < t_max) {
                t_op = Some(t2);
            }
            if (t_min < t1) && (t1 < t_max) {
                t_op = Some(t1);
            }
            let ans = match t_op {
                None => false,
                Some(_t) => true,
            };
            if ans {
                let rec2 = HitRecord{
                    t: t_op.unwrap(),
                    p: ray.point_at_parameter(t_op.unwrap()),
                    normal: (ray.point_at_parameter(t_op.unwrap()) - self.center) / self.radius,
                    material: self.material,
                };
                //*rec = MaybeUninit(rec2);
                *rec = Some(rec2);
                // rec.t = t_op.unwrap();
                // rec.p = ray.point_at_parameter(rec.t);
                // rec.normal = (rec.p - self.center) / self.radius;
            }
            return ans;
        }
        return false;
    }
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

impl Hittable for Form{
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32, rec: &mut Option<HitRecord>) -> bool{
        return match self {
            Form::Sphere(sphere) => sphere.hit(ray, t_min, t_max, rec),
            Form::Cube(cube) => cube.hit(ray, t_min, t_max, rec),
        }
    }
}

impl Hittable for HittableList{
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32, rec: &mut Option<HitRecord>) -> bool{
        let mut temp_rec: Option<HitRecord> = None;
        let mut hit_any: bool = false;
        let mut t_closest: f32 = t_max;
        for form in &self.forms {
            if form.hit(ray, t_min, t_closest, &mut temp_rec) {
                let rr: &HitRecord = temp_rec.borrow().as_ref().unwrap();
                hit_any = true;
                t_closest = rr.t;

                *rec = Some(HitRecord {
                    t: t_closest,
                    p: rr.p,
                    normal: rr.normal,
                    material: form.material()
                } );

                //rec.t = temp_rec.t;
                //rec.p = temp_rec.p;
                //rec.normal = temp_rec.normal;
            }
        }
        return hit_any;
    }
}