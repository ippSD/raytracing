mod test_patch_sphere;

use crate::objects::{HittableList, Form, Rectangle};
use crate::materials::{Material, LambertianKind};
use crate::Vec3;
use crate::radiation::{ViewFactors, Vfs};
use std::ops::{AddAssign};

const W: f32 = 0.4;
const H: f32 = 0.1;
const L: f32 = 0.8;

pub fn test_world() -> HittableList {
    let mut world: HittableList = HittableList::new();

    let rec_1: Rectangle = Rectangle::new(
        Vec3::new(0e0, H/2e0, -L/2e0),
        L,
        H,
        Material::Lambertian(LambertianKind::new(Vec3::new(0.81, 0.3, 0.3))),
        Vec3::new(0e0, 0e0, -1e0),
        Vec3::new(0e0, 1e0, 0e0),
        Vec3::new(1e0, 0e0, 0e0)
    );

    let rec_2: Rectangle = Rectangle::new(
        Vec3::new(W/2e0, 0e0, -L/2e0),
        L,
        W,
        Material::Lambertian(LambertianKind::new(Vec3::new(0.3, 0.3, 0.81))),
        Vec3::new(0e0, 0e0, -1e0),
        Vec3::new(1e0, 0e0, 0e0),
        Vec3::new(0e0, -1e0, 0e0)
    );

    world.forms.push(Form::Rectangle(rec_1));
    world.forms.push(Form::Rectangle(rec_2));
    world
}

pub fn test_vf(n: usize) {
    let world: HittableList = test_world();
    let output: Vfs = world.view_factors(n);
    let mut output_string: String = String::new();

    for row in output.vfs.iter() {
        for col in row.iter() {
            output_string.add_assign(&*format!("{}", col));
            output_string.add_assign(";");
        }
        output_string.add_assign("\n");
    }
    println!("{}", output_string);
}

fn parallel_square_world(c1: Vec3, c2: Vec3, w1: f32, h1: f32, w2: f32, h2: f32) -> HittableList {
    let mut world: HittableList = HittableList::new();

    let rec_1: Rectangle = Rectangle::new(
        c1,
        w1,
        h1,
        Material::Lambertian(
            LambertianKind::new(Vec3::new(0.81, 0.3, 0.3))
        ),
        Vec3::new(0e0, 0e0, 1e0),
        Vec3::new(0e0, 1e0, 0e0),
        Vec3::new(-1e0, 0e0, 0e0)
    );

    let rec_2: Rectangle = Rectangle::new(
        c2,
        w2,
        h2,
        Material::Lambertian(
            LambertianKind::new(Vec3::new(0.3, 0.3, 0.81))
        ),
        Vec3::new(0e0, 0e0, 1e0),
        Vec3::new(0e0, 1e0, 0e0),
        Vec3::new(-1e0, 0e0, 0e0)
    );

    world.forms.push(Form::Rectangle(rec_1));
    world.forms.push(Form::Rectangle(rec_2));
    world
}

pub fn test_eq_square(n: usize, w: f32, h: f32) {
    let world: HittableList = parallel_square_world(
        Vec3::new(-h/2e0, w/2e0, w/2e0),
        Vec3::new( h/2e0, w/2e0, w/2e0),
        w,
        w,
        w,
        w,
    );
    let output: Vfs = world.view_factors(n);
    let mut output_string: String = String::new();

    for row in output.vfs.iter() {
        for col in row.iter() {
            output_string.add_assign(&*format!("{}", col));
            output_string.add_assign(";");
        }
        output_string.add_assign("\n");
    }
    println!("{}", output_string);
}
