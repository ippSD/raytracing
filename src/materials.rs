use crate::rays::Ray;
use crate::hittable::HitRecord;
use crate::vectors::{Vec3, Vec3Methods};
extern crate rand;
use rand::Rng;

fn random_in_unit_sphere() -> Vec3{
    let mut p: Vec3 = Vec3::new(2e0,2e0,2e0);
    while p.square_length() >= 1e0 {
        p = Vec3::random() * 2e0 - Vec3::ones();
    }
    p
}

#[derive(Copy, Clone)]
pub struct LambertianKind{
    albedo: Vec3
}

impl LambertianKind{
    pub fn new(albedo: Vec3) -> LambertianKind { LambertianKind {albedo} }
}

#[derive(Copy, Clone)]
pub struct MetalKind{
    albedo: Vec3,
    fuzz: f32
}

impl MetalKind{
    pub fn new(albedo: Vec3, fuzz: f32) -> MetalKind { MetalKind {albedo, fuzz} }
    pub fn reflect(v: Vec3, n: Vec3) -> Vec3 { v - n * n.dot(&v) * 2e0 }
}

#[derive(Copy, Clone)]
pub struct DielectricKind{
    n: f32
}

impl DielectricKind{
    pub fn new(n: f32) -> DielectricKind{ DielectricKind{n} }
    pub fn refract(v: Vec3, n: Vec3, ni_over_nt: f32, refracted: &mut Vec3) -> bool{
        let ans: bool;
        let uv: Vec3 = v.unit_vector();
        let dt: f32 = uv.dot(&n);
        let discriminant: f32 = 1e0 - ni_over_nt.powi(2) * (1e0 - dt.powi(2));
        if discriminant > 0e0 {
            refracted.e = ((uv - n * dt) * ni_over_nt - n * discriminant.sqrt()).e;
            ans = true;
        }
        else {
            ans = false;
        }
        ans
    }
    pub fn schlick(n: f32, cosine: f32) -> f32 {
        let r0: f32 = ((1e0 - n) / (1e0 + n)).powi(2);
        r0 + (1e0 - r0) * ((1e0 - cosine).powi(5))
    }
}

#[derive(Copy, Clone)]
pub enum Material{
    Lambertian(LambertianKind),
    Metal(MetalKind),
    Dielectric(DielectricKind),
}

impl MaterialScatter for Material{
    fn scatter(self, ray_in: &Ray, hit_rec: &HitRecord, attenuation: &mut Vec3, scattered: &mut Ray) -> bool{
        return match self {
            Material::Lambertian(lambertian) => {
                let target: Vec3 = hit_rec.p + hit_rec.normal + random_in_unit_sphere();
                scattered.a = hit_rec.p;
                scattered.b = target - hit_rec.p;
                attenuation.e = lambertian.albedo.e;
                true
            }
            Material::Metal(metal) => {
                let reflected = MetalKind::reflect(ray_in.direction().unit_vector(), hit_rec.normal);
                scattered.a = hit_rec.p;
                scattered.b = reflected + random_in_unit_sphere() * metal.fuzz;
                attenuation.e = metal.albedo.e;
                scattered.direction().dot(&hit_rec.normal) > 0e0
            }
            Material::Dielectric(dielectric) => {
                let reflected: Vec3 = MetalKind::reflect(ray_in.direction(), hit_rec.normal);
                attenuation.e = Vec3::new(1e0, 1e0, 1e0).e;
                let outward_normal: Vec3;
                let ni_over_nt: f32;
                let reflect_prob: f32;
                let mut cosine: f32;
                let mut refracted: Vec3 = Vec3::zeros();
                if ray_in.direction().dot(&hit_rec.normal) > 0e0 {
                    outward_normal = -hit_rec.normal;
                    ni_over_nt = dielectric.n;
                    //cosine = dielectric.n * ray_in.direction().dot(&hit_rec.normal) / ray_in.direction().length();
                    cosine = ray_in.direction().dot(&hit_rec.normal) / ray_in.direction().length();
                    cosine = (1e0 - dielectric.n.powi(2)*(1e0 - cosine.powi(2))).sqrt();
                }
                else {
                    outward_normal = hit_rec.normal;
                    ni_over_nt = 1e0 / dielectric.n;
                    cosine = - ray_in.direction().dot(&hit_rec.normal) / ray_in.direction().length();
                }

                if DielectricKind::refract(ray_in.direction(), outward_normal, ni_over_nt, &mut refracted) {
                    reflect_prob = DielectricKind::schlick(dielectric.n, cosine);
                }
                else {
                    scattered.a = hit_rec.p;
                    scattered.b = reflected;
                    reflect_prob = 1e0;
                }
                let mut rng = rand::thread_rng();
                if rng.gen::<f32>() < reflect_prob {
                    scattered.a = hit_rec.p;
                    scattered.b = reflected;
                }
                else {
                    scattered.a = hit_rec.p;
                    scattered.b = refracted;
                }
                true
            }
        }
    }
}

pub trait MaterialScatter{
    fn scatter(self, ray_in: &Ray, hit_rec: &HitRecord, attenuation: &mut Vec3, scattered: &mut Ray) -> bool;
}