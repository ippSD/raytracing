//! Radiation module. Includes structures, traits and implementations
//! for computing the View Factors inside the world.

use std::f32::consts::PI;
use std::fmt::{Display, Formatter, Error};

use rand::random;
use crate::vectors::{Vec3, Vec3Methods};
use crate::rays::Ray;
use crate::objects::{Form, SurfaceFunctions, HittableList};
use crate::hittable::{HitRecord, Hittable};


/// View Factors structure.
pub struct Vfs {
    pub vfs: Vec<Vec<f32>>
}

/// View Factors methods.
impl Vfs {
    /// Constructor.
    pub fn new() -> Vfs {
        Vfs { vfs: Vec::new() }
    }

    /// From list of view factors Vec<Vec<f32>>.
    pub fn from(vfs: Vec<Vec<f32>>) -> Vfs {
        Vfs { vfs }
    }
}

/// View Factor trait for world.
pub trait ViewFactors {
    /// Compute the view factor for two objects on the world by means of
    /// the Monte Carlo Method.
    ///
    /// # Parameters:
    ///
    /// * `self`: world of objects.
    /// * `n`: number of iterations on the Monte Carlo method.
    /// * `form_1_idx`: World index pointing at the first (main) object.
    /// * `form_2_idx`: World index pointing at the second object.
    ///
    /// # Returns:
    ///
    /// * `f32`: View factor from object 1 to 2, $F_{12}$.
    fn view_factor(&self, n: usize, form_1_idx: usize, form_2_idx: usize) -> f32;

    /// Compute the view factors on the world by means of
    /// the Monte Carlo Method.
    ///
    /// # Parameters:
    ///
    /// * `self`: world of objects.
    /// * `n`: number of iterations on the Monte Carlo method.
    ///
    /// # Returns:
    ///
    /// * `Vfs`: View factors of the world objects.
    fn view_factors(&self, n: usize) -> Vfs;
}

impl ViewFactors for HittableList {
    fn view_factor(&self, n: usize, form_1_idx: usize, form_2_idx: usize) -> f32 {
        let form_1: &Form = self.forms.get(form_1_idx).unwrap();
        let form_2: &Form = self.forms.get(form_2_idx).unwrap();
        let mut temp: f32 = 0e0;
        let mut l;
        let mut r12: Vec3;
        let mut n1: Vec3;
        let mut n2: Vec3;
        let mut cos_beta1: f32;
        let mut cos_beta2: f32;
        let mut s1: f32;
        let mut t1: f32;
        let mut s2: f32;
        let mut t2: f32;

        let mut da1: f32;
        let mut da2: f32;

        let mut p1: Vec3;
        let mut p2: Vec3;

        let mut hit_rec: Option<HitRecord> = None;
        let mut ray: Ray;

        for _ in 0..n {
            s1 = random();
            t1 = random();
            s2 = random();
            t2 = random();

            p1 = form_1.point(s1, t1);
            n1 = form_1.normal(s1, t1);

            p2 = form_2.point(s2, t2);
            n2 = form_2.normal(s2, t2);

            r12 = p2 - p1;
            l = r12.length();

            cos_beta1 = r12.dot(&n1) / l;
            cos_beta2 = (-r12).dot(&n2) / l;

            ray = Ray::new(p1, r12.unit_vector());
            cos_beta2 = match self.hit(&ray, 1e-8, l, &mut hit_rec) {
                true => match &hit_rec {
                    Some(rec) => match rec.hit_elem == form_2_idx {
                        true => cos_beta2,
                        false => 0e0,
                    }
                    None => 0e0,
                }
                false => {
                    cos_beta2
                }
            };

            if cos_beta1 < 0e0 || cos_beta2 < 0e0 {
                cos_beta1 = 0e0
            }
            da1 = form_1.diff_a(s1, t1);
            da2 = form_2.diff_a(s2, t2);
            // println!("C1: {}, C2: {}, L: {}", cos_beta1, cos_beta2, l);
            temp = temp + cos_beta1 * cos_beta2 / l.powi(2) * da1 * da2;
        }

        return temp / PI / form_1.area() / (n as f32);
    }

    fn view_factors(&self, n: usize) -> Vfs {
        let n_objs: usize = self.forms.len();
        let mut viewfactors: Vec<Vec<f32>> = Vec::new();

        for i in 0..n_objs {
            let mut views_i: Vec<f32> = Vec::new();

            for j in (i+1)..n_objs {
                views_i.push(
                    self.view_factor(
                        n,
                        i,
                        j
                    )
                );
            }

            viewfactors.push(views_i);
        }
        Vfs::from(viewfactors)
    }
}

/// Display trait implementation on Vfs.
impl Display for Vfs {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        let n: usize = self.vfs.len();
        let mut vjs: &Vec<f32>;
        for i in 0..n {
            vjs = self.vfs.get(i).unwrap();
            for j in 0..(n-i-1) {
                writeln!(
                    f,
                    "F({},{}) = {:.4}",
                    i,
                    j + i + 1,
                    vjs.get(j).unwrap())?;
            }
        }
        Ok(())
    }
}