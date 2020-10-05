/*!
Program that generates a random image of basic shapes and materials using ray-tracing.

# Arguments
* `NX`: u16 = 1200; Number of pixels on X-Axis.
* `NY`: u16 = 800; Number of pixels on Y-Axis.
* `NS`: Number of slightly deviated rays per pixel for color smoothing.
* `N_OBJ`: usize = 500; Number of objects to be created.
* `MAX_DEPTH`: Maximum number of new ray spawns.
* `LOOK_FROM`: Vec3 = Vec3::new_const(13e0, 2e0, 3e0); Camera position.
* `LOOK_AT`: Vec3 = Vec3::new_const(0e0, 0e0, 0e0); Camera focus point.
* `VUP`: Vec3 = Vec3::new_const(0e0, 1e0, 0e0); World's upward direction.
* `VFOV`: f32 = 20e0; Camera's field of view (degrees).
* `ASPECT`: f32 = (NX as f32) / (NY as f32); Camera's aspect.
* `FOCUS`: bool = false; If false, a simple camera is used, else, a focus-featured one.
*/


use raytracing::{random_world, print_world};
use raytracing::objects::HittableList;
use raytracing::vectors::Vec3;
use raytracing::cameras::{Camera, CameraFocus, CameraSimple};
use raytracing::objects::{Form, Sphere, Cube, Square};
use raytracing::materials::{Material, LambertianKind, MetalKind, DielectricKind};

use rand::Rng;
use raytracing::radiation::ViewFactors;

extern crate rand;

const NX: u16 = 1200;
const NY: u16 = 800;
const NS: u16 = 16;
const N_OBJ: usize = 500;
const MAX_DEPTH: usize = 30;
const LOOK_FROM: Vec3 = Vec3::new_const(13e0, 2e0, 3e0);
const LOOK_AT: Vec3 = Vec3::new_const(0e0, 0e0, 0e0);
const VUP: Vec3 = Vec3::new_const(0e0, 1e0, 0e0);
const VFOV: f32 = 20e0;
const ASPECT: f32 = (NX as f32) / (NY as f32);
const FOCUS: bool = false;
const TEST_WORLD: bool = false;
const TEST_VF: bool = false;
const DO_PRINT: bool = true;


fn random_old_world(n: usize) -> HittableList{
    let mut rng = rand::thread_rng();
    let mut center: Vec3 = Vec3::zeros();
    let mut choose_mat: f32;
    let mut choose_form: f32;
    let mut mat: Material;
    let mut world: HittableList = HittableList::new();
    for a in -11..11 {
        for b in -11..11 {
            if world.forms.len() > n { break; }
            choose_mat = rng.gen::<f32>();
            choose_form = rng.gen::<f32>();
            center.e = [
                (a as f32) + 0.9 * rng.gen::<f32>(),
                0.2,
                (b as f32) + 0.9 * rng.gen::<f32>()
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
            if choose_form < 0.1 {
                world.forms.push(Form::Square(Square::horizontal_surface(center, 0.2, mat)));
            }
            else if choose_form < 0.8 {
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

/*
fn main_OLD() -> std::io::Result<()> {
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

fn main() -> std::io::Result<()> {
    let camera: Camera = match TEST_VF {
        true => Camera::Simple(CameraSimple::new(
            Vec3::new(1.8, 1.1, 1.0),
            Vec3::new(0.4, 0.05, -0.2),
            Vec3::new(0e0, 1e0, 0e0),
            VFOV,
            ASPECT
        )),
        false =>{
            match FOCUS {
                true => Camera::Focus(CameraFocus::new(
                    LOOK_FROM,
                    LOOK_AT,
                    VUP,
                    VFOV,
                    ASPECT,
                    0.1,
                    10e0
                )),
                false => Camera::Simple(CameraSimple::new(
                    LOOK_FROM,
                    LOOK_AT,
                    VUP,
                    VFOV,
                    ASPECT
                )),
            }
        }
    };

    let world: HittableList = match TEST_WORLD {
        true => random_world(
            N_OBJ,
            [-4e0, 4e0],
            [0.2, 0.4],
            [5e0, 10e0],
            [0.4, 0.8],
            1.0,
            0.0,
            0e0,
            1e0,
            0e0,
            0e0
        ),
        false => random_old_world(N_OBJ),
    };

    /*
    if ! TEST_VF {
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
    }
    */

    if TEST_VF {
        println!("{}", world.view_factors(10240));
    }

    return match DO_PRINT {
        true => print_world(
            &world,
            &camera,
            NX,
            NY,
            0e0,
            NS,
            MAX_DEPTH
        ),
        false => Ok(()),
    };
}
