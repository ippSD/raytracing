use crate::objects::{HittableList, Form};
use crate::materials::{Material, LambertianKind};
use crate::{Vec3, Square, Sphere};
use crate::radiation::ViewFactors;

const N: usize = 10;
const N_MC: usize = 224000;
const EPS: f32 = 1e-8;
const ERR_MAX: f32 = 7.5e-2;

fn materials() -> Vec<Material> {
    let mat_1: Material = Material::Lambertian(LambertianKind::new(Vec3::new_const(0.81, 0.3, 0.3)));
    let mat_2: Material = Material::Lambertian(LambertianKind::new(Vec3::new_const(0.3, 0.3, 0.81)));

    let mut v = Vec::new();
    v.push(mat_1);
    v.push(mat_2);
    v
}

fn world_patch_frontal_sphere(r: f32, h: f32) -> HittableList {
    let mut world: HittableList = HittableList::new();
    let mats: Vec<Material> = materials();

    let patch: Square = Square::new(
        Vec3::zeros(),
        r*EPS*EPS,
        *mats.get(0).unwrap(),
        Vec3::new(0e0, 1e0, 0e0),
        Vec3::new(0e0, 0e0, 1e0),
        Vec3::new(1e0, 0e0, 0e0)
    );

    let sph: Sphere = Sphere::new(
        Vec3::new(h, 0e0, 0e0),
        r,
        *mats.get(1).unwrap()
    );

    world.forms.push(Form::Square(patch));
    world.forms.push(Form::Sphere(sph));
    world
}

fn world_patch_leveled_sphere(r: f32, h: f32) -> HittableList {
    let mut world: HittableList = HittableList::new();
    let mats: Vec<Material> = materials();

    let patch: Square = Square::new(
        Vec3::zeros(),
        r*EPS*EPS,
        *mats.get(0).unwrap(),
        Vec3::new(0e0, 0e0, 1e0),
        Vec3::new(1e0, 0e0, 0e0),
        Vec3::new(0e0, 1e0, 0e0)
    );

    let sph: Sphere = Sphere::new(
        Vec3::new(h, 0e0, 0e0),
        r,
        *mats.get(1).unwrap()
    );

    world.forms.push(Form::Square(patch));
    world.forms.push(Form::Sphere(sph));
    world
}

fn view_patch_frontal_sphere(r: f32, h: f32) -> f32 {
    return (r/h).powi(2)
}

fn view_patch_leveled_sphere(r: f32, h: f32) -> f32 {
    let h_min: f32 = h / r;
    let x: f32 = (h_min.powi(2) - 1e0).sqrt();
    return (x.recip().atan() - x / h_min.powi(2)) / std::f32::consts::PI;
}

#[test]
pub fn test_patch_frontal_sphere() {
    let mut h: f32;
    let mut vf: f32;
    let mut vf_calc: f32;
    let mut world: HittableList;

    for i in 0..N {
        h = 2e0 - (i as f32) / (N as f32);
        world = world_patch_frontal_sphere(1e0, h);
        vf = *world.view_factors(N_MC).vfs.get(0).unwrap().get(0).unwrap();
        vf_calc = view_patch_frontal_sphere(1e0, h);
        println!("h={}, VF (raytracing) = {}, VF (aprox) = {}", h, vf, vf_calc);
        assert!((vf - vf_calc).abs() < ERR_MAX);
    }
}


#[test]
pub fn test_patch_leveled_sphere() {
    let mut h: f32;
    let mut vf: f32;
    let mut vf_calc: f32;
    let mut world: HittableList;

    for i in 0..N {
        h = 2e0 - (i as f32) / (N as f32);
        world = world_patch_leveled_sphere(1e0, h);
        vf = *world.view_factors(N_MC).vfs.get(0).unwrap().get(0).unwrap();
        vf_calc = view_patch_leveled_sphere(1e0, h);
        println!("h={}, VF (raytracing) = {}, VF (aprox) = {}", h, vf, vf_calc);
        assert!((vf - vf_calc).abs() < ERR_MAX);
    }
}