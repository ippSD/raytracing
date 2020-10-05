//! Raytracing module. Includes functions and objects for:
//! * Generating the view of a world by means of raytracing.
//! * Calculating the view factors of all objects on the world.

use std::fs::File;
use std::io::Write;
use std::vec::Vec;
use std::io::Result;

pub mod vectors;
pub mod rays;
pub mod hittable;
pub mod objects;
pub mod cameras;
pub mod materials;
pub mod radiation;
#[cfg(test)]
pub mod tests;

use vectors::{Vec3,Vec3Methods};
use rays::{Ray};
use objects::{Sphere, Cube, Square, Form, HittableList};
use materials::{Material, LambertianKind, MetalKind, DielectricKind};
use cameras::{Camera, CameraRay};

extern crate rand;
use rand::random;

struct MatChoice {
    prob: f32,
    kind: u8,
}

struct FormChoice {
    prob: f32,
    kind: u8,
}

pub fn random_world(
    n: usize,
    x_lim: [f32; 2],
    y_lim: [f32; 2],
    z_lim: [f32; 2],
    obj_len_lim: [f32; 2],
    sphere_threshold: f32,
    cube_theshold: f32,
    square_threshold: f32,
    lambertian_threshold: f32,
    metal_threshold: f32,
    dielectric_threshold: f32,) -> HittableList
{
    let mut world: HittableList = HittableList::new();

    let mut choose_mat: f32;
    let mut choose_form: f32;
    let mut mats: Vec<MatChoice> = Vec::new();
    let mut forms: Vec<FormChoice> = Vec::new();

    mats.push( MatChoice{prob: lambertian_threshold, kind: 0 } );
    mats.push( MatChoice{prob: metal_threshold, kind: 1 } );
    mats.push( MatChoice{prob: dielectric_threshold, kind: 2 } );
    mats.sort_by(|a, b| a.prob.partial_cmp(&b.prob).unwrap());

    forms.push( FormChoice{prob: sphere_threshold, kind: 0 } );
    forms.push( FormChoice{prob: cube_theshold, kind: 1 } );
    forms.push( FormChoice{prob: square_threshold, kind: 2 } );
    forms.sort_by(|a, b| a.prob.partial_cmp(&b.prob).unwrap());

    let mut center_random: Vec3;
    let mut center: Vec3;
    let center_min: Vec3 = Vec3::new(x_lim[0], y_lim[0], z_lim[0]);
    let center_max: Vec3 = Vec3::new(x_lim[1], y_lim[1], z_lim[1]);

    let mut length_random: f32;
    let mut length: f32;

    for _ in 0..n {
        choose_mat = random::<f32>();
        choose_form = random::<f32>();
        center_random = Vec3::random();

        center = center_random * center_min + (Vec3::ones() - center_random) * center_max;

        length_random = random::<f32>();
        length = obj_len_lim[0] * length_random + obj_len_lim[1] * (1e0 - length_random);

        let mut mat: Material = Material::Dielectric(DielectricKind::new(1.5));
        for mat_type in &mats {
            if choose_mat < mat_type.prob {
                mat = match mat_type.kind {
                    0 => Material::Lambertian(LambertianKind::new(Vec3::random() * Vec3::random())),
                    1 => Material::Metal(MetalKind::new(Vec3::ones(), 0.01 * random::<f32>())),
                    _ => Material::Dielectric(DielectricKind::new(1.5))
                }
            };
        }

        let mut form: Form = Form::Square(Square::horizontal_surface(center, length, mat));
        for form_type in &forms {
            if choose_form < form_type.prob {
                form = match form_type.kind {
                    0 => Form::Sphere(Sphere::new(center, length / 2e0, mat)),
                    1 => Form::Cube(Cube::new(center, length, mat)),
                    _ => Form::Square(Square::horizontal_surface(center, length, mat)),
                }
            };
        }

        world.forms.push(form);

    }

    world
}

pub fn print_world(
    world: &HittableList,
    cam: &Camera,
    width_px: u16,
    height_px: u16,
    dev: f32,
    n_smooth: u16,
    max_depth: usize) -> Result<()>
{
    let mut buffer = File::create("ray_tracing.ppm")?;
    buffer.write(b"P3\n")?;
    buffer.write_fmt(format_args!("{} {}\n", width_px, height_px))?;
    buffer.write(b"255\n")?;
    for j in (0..height_px).rev() {
        for i in 0..width_px {
            let mut col: Vec3 = Vec3::zeros();
            for _s in 0..n_smooth {
                let u: f32 = (i as f32 + dev * random::<f32>()) / (width_px as f32);
                let v: f32 = (j as f32 + dev * random::<f32>()) / (height_px as f32);
                let ray: Ray = cam.get_ray(u, v);
                col += ray.color(&world, 0, max_depth);
            }
            col /= n_smooth as f32;
            // Correction 'Gamma 2': (0 <= r|g|b < 1)^(1/(gamma == 2))
            col.gamma2();

            let ir: u8 = (255.99 * col.r()) as u8;
            let ig: u8 = (255.99 * col.g()) as u8;
            let ib: u8 = (255.99 * col.b()) as u8;

            buffer.write_fmt(format_args!("{} {} {}\n", ir, ig, ib))?;
        }
    }
    drop(buffer);
    Ok(())
}

/*
fn random_old(n: usize) -> HittableList{
    let mut rng = rand::thread_rng();
    let mut center: Vec3 = Vec3::zeros();
    let mut choose_mat: f32;
    let mut choose_form: f32;
    let mut mat: Material;
    let mut world: HittableList = HittableList::new();
    for a in -11..11 {
        for b in -11..11 {
            if world.forms.len() > n { break; }
            choose_mat = random::<f32>();
            choose_form = random::<f32>();
            center.e = [
                (a as f32) + 0.9 * random::<f32>(),
                0.2,
                (b as f32) + 0.9 * random::<f32>()
            ];
            if choose_mat < 0.8{
                mat = Material::Lambertian(
                    LambertianKind::new(Vec3::random() * Vec3::random())
                );
            }
            else if choose_mat < 0.95 {
                mat = Material::Metal(
                    MetalKind::new(Vec3::ones(), 0.01 * rng.gen::<f32>())
                );
            }
            else {
                mat = Material::Dielectric(
                    DielectricKind::new(1.5)
                );
            }
            if choose_form < 0.95 {
                world.forms.push(Form::Square(Square::horizontal_surface(center, 0.2, mat)));
            }
            else if choose_form < 0.99{
                world.forms.push(Form::Sphere(Sphere::new(center, 0.2, mat)));
            }
            else {
                world.forms.push(Form::Cube(Cube::new(center, 0.4, mat)));
            }

        }
    }

    // Sample dielectric sphere.
    world.forms.push(
        Form::Sphere(
            Sphere::new(
                Vec3::new(0e0, 1e0, 0e0),
                1e0,
                Material::Dielectric(DielectricKind::new(1.5)
                )
            )
        )
    );

    // Sample lambertian sphere.
    world.forms.push(
        Form::Sphere(
            Sphere::new(
                Vec3::new(-4e0, 1e0, 0e0),
                1e0,
                Material::Lambertian(
                    LambertianKind::new(
                        Vec3::new(0.4,0.2,0.1)
                    )
                )
            )
        )
    );
    /*world.forms.push(
        Form::Sphere(
            Sphere::new(
                Vec3::new(4e0, 1e0, 0e0),
                1e0,
                Material::Metal(
                    MetalKind::new(
                        Vec3::new(0.7,0.6,0.5),
                        0.0)
                )
            )
        )
    );*/

    // Sample metal cube.
    world.forms.push(
        Form::Cube(
            Cube::new(
                Vec3::new(4e0, 5e-1, 0e0),
                1e0,
                Material::Metal(
                    MetalKind::new(
                        Vec3::new(0.7,0.6,0.5),
                        0.0)
                )
            )
        )
    );

    // Floor.
    world.forms.push(
        Form::Sphere(
            Sphere::new(
                Vec3::new(0e0,-1e3,0e0),
                1e3,
                Material::Lambertian(
                    LambertianKind::new(
                        Vec3::new(5e-1,5e-1,5e-1)
                    )
                )
            )
        )
    );
    /*world.forms.push(
        Form::Cube(
            Cube::new(
                Vec3::new(0e0,0.5,0e0),
                1e0,
                Material::Lambertian(
                    LambertianKind::new(
                        Vec3::new(5e-1,5e-1,5e-1)
                    )
                )
            )
        )
    );*/

    // Delete some objects.
    while world.forms.len() > n {
        world.forms.remove(0);
    }
    world
}
*/

/*
fn main() -> std::io::Result<()> {
    let mut rng = rand::thread_rng();

    let camera: Camera;
    if FOCUS {
        let dist_to_focus: f32 = 10e0;
        let aperture: f32 = 0.1;
        camera = Camera::Focus(CameraFocus::new(
            LOOK_FROM,
            LOOK_AT,
            VUP,
            VFOV,
            ASPECT,
            aperture,
            dist_to_focus
        ));
    }
    else {
        camera = Camera::Simple(CameraSimple::new(
            LOOK_FROM,
            LOOK_AT,
            VUP,
            VFOV,
            ASPECT
        ));
    }

    let world: HittableList = random(N_OBJ);

    let mut buffer = File::create("ray_tracing.ppm")?;
    buffer.write(b"P3\n")?;
    buffer.write_fmt(format_args!("{} {}\n", NX, NY))?;
    buffer.write(b"255\n")?;
    for j in (0..NY).rev() {
        for i in 0..NX {
            let mut col: Vec3 = Vec3::zeros();
            for _s in 0..NS {
                let u: f32 = (i as f32 + rng.gen::<f32>()) / (NX as f32);
                let v: f32 = (j as f32 + rng.gen::<f32>()) / (NY as f32);
                let ray: Ray = camera.get_ray(u, v);
                col += ray.color(&world, 0, MAX_DEPTH);
            }
            col /= NS as f32;
            // Correction 'Gamma 2': (0 <= r|g|b < 1)^(1/(gamma == 2))
            col.gamma2();

            let ir: u8 = (255.99 * col.r()) as u8;
            let ig: u8 = (255.99 * col.g()) as u8;
            let ib: u8 = (255.99 * col.b()) as u8;

            buffer.write_fmt(format_args!("{} {} {}\n", ir, ig, ib))?;
        }
    }
    drop(buffer);
    println!("Hello, world!");
    Ok(())
}
*/
