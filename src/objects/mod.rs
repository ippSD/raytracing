//! Object module. Includes the Form structure consisting of
//! several basic object types (and materials).

mod cube;
mod square;
mod rectangle;
mod sphere;
mod traits;

use crate::vectors::{Vec3};
use crate::rays::Ray;
use crate::hittable::{HitRecord, Hittable};
use crate::materials::Material;
pub use crate::objects::cube::Cube;
pub use crate::objects::square::Square;
pub use crate::objects::sphere::Sphere;
pub use rectangle::Rectangle;
pub use crate::objects::traits::{ObjectGetters, SurfaceFunctions};
use std::borrow::{Borrow};

extern crate nalgebra;
extern crate rand;


/// Object enumerable.
pub enum Form{
    Sphere(Sphere),
    Cube(Cube),
    Square(Square),
    Rectangle(Rectangle),
}

/// List of available hittable objects on the world.
pub struct HittableList{
    /// Object vector.
    pub forms: Vec<Form>
}

/// Object function members.
impl Form{
    /// Getter for material data.
    pub fn material(&self) -> Material {
        return self.get_material();
    }
}

impl ObjectGetters for Form {
    fn get_material(&self) -> Material {
        return match self {
            Form::Sphere(sphere) => sphere.get_material(),
            Form::Cube(cube) => cube.get_material(),
            Form::Square(square) => square.get_material(),
            Form::Rectangle(rec) => rec.get_material(),
        }
    }

    fn get_center(&self) -> Vec3 {
        return match self {
            Form::Sphere(sphere) => sphere.get_center(),
            Form::Cube(cube) => cube.get_center(),
            Form::Square(square) => square.get_center(),
            Form::Rectangle(rec) => rec.get_center(),
        }
    }
}

impl SurfaceFunctions for Form {
    fn point(&self, s: f32, t: f32) -> Vec3 {
        return match self {
            Form::Rectangle(rec) => rec.point(s, t),
            Form::Square(sq) => sq.point(s, t),
            Form::Sphere(sp) => sp.point(s, t),
            Form::Cube(_c) => Vec3::zeros(),
        }
    }

    fn normal(&self, s: f32, t: f32) -> Vec3 {
        return match self {
            Form::Rectangle(rec) => rec.normal(s, t),
            Form::Square(sq) => sq.normal(s, t),
            Form::Sphere(sp) => sp.normal(s, t),
            Form::Cube(_c) => Vec3::zeros(),
        }
    }

    fn area(&self) -> f32 {
        return match self {
            Form::Rectangle(rec) => rec.area(),
            Form::Square(sq) => sq.area(),
            Form::Sphere(sp) => sp.area(),
            Form::Cube(_c) => 0e0,
        }
    }

    fn diff_a(&self, s: f32, t: f32) -> f32 {
        return match self {
            Form::Rectangle(rec) => rec.diff_a(s, t),
            Form::Square(sq) => sq.diff_a(s, t),
            Form::Sphere(sp) => sp.diff_a(s, t),
            Form::Cube(_c) => 0e0,
        }
    }
}

/// HittableList function members.
impl HittableList{
    /// Constructor (empty vector).
    pub fn new() -> HittableList { HittableList {forms: Vec::new()} }
}

impl Hittable for Form {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32, rec: &mut Option<HitRecord>) -> bool{
        return match self {
            Form::Sphere(sphere) => sphere.hit(ray, t_min, t_max, rec),
            Form::Cube(cube) => cube.hit(ray, t_min, t_max, rec),
            Form::Square(square) => square.hit(ray, t_min, t_max, rec),
            Form::Rectangle(rectangle) => rectangle.hit(ray, t_min, t_max, rec),
        }
    }
}

impl Hittable for HittableList {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32, rec: &mut Option<HitRecord>) -> bool{
        let mut temp_rec: Option<HitRecord> = None;
        let mut hit_any: bool = false;
        let mut t_closest: f32 = t_max;
        for (idx, form) in self.forms.iter().enumerate() {
        // for form in &self.forms {
            if form.hit(ray, t_min, t_closest, &mut temp_rec) {
                let rr: &HitRecord = temp_rec.borrow().as_ref().unwrap();
                hit_any = true;
                t_closest = rr.t;

                *rec = Some(HitRecord {
                    t: t_closest,
                    p: rr.p,
                    normal: rr.normal,
                    material: form.material(),
                    hit_elem: idx
                } );

                //rec.t = temp_rec.t;
                //rec.p = temp_rec.p;
                //rec.normal = temp_rec.normal;
            }
        }
        return hit_any;
    }
}
