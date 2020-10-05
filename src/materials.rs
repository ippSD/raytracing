//! Materials module. Defines the object's material, which consists
//! in a set of available material kinds and the required methods.

extern crate rand;
use self::rand::random;

use crate::rays::Ray;
use crate::hittable::HitRecord;
use crate::vectors::{Vec3, Vec3Methods};


/// Lambertian materials.
/// Lambertian (diffuse) materials reflect the
/// light preferentially around the surface's
/// normal vector at the hit-point with a dye
/// of its own surface color, ``albedo``.
#[derive(Copy, Clone)]
pub struct LambertianKind{
    /// Surface RGB color.
    albedo: Vec3
}

/// Metallic material.
/// Light is reflected on the plane formed by
/// the light direction and the surface normal
/// at the hit-point. A fuzzy
/// effect may be added, so that the reflected
/// ray is deviated on around an sphere of
/// radius ``fuzz``.
#[derive(Copy, Clone)]
pub struct MetalKind{
    /// Metal RGB color.
    albedo: Vec3,
    /// Deviation from perfect reflection.
    fuzz: f32
}

/// Dielectric material.
/// Dielectrics may reflect or refract depending
/// on the hit angle and the material's refractive
/// index ``n`` (see Snell's Law).
#[derive(Copy, Clone)]
pub struct DielectricKind{
    /// Material's refractive index.
    n: f32
}

/// Material Structure.
/// Enumerable type which includes several material
/// kinds: lambertian, metallic or dielectric.
#[derive(Copy, Clone)]
pub enum Material{
    Lambertian(LambertianKind),
    Metal(MetalKind),
    Dielectric(DielectricKind),
}

/// Ray scattering methods upon hitting any material.
pub trait MaterialScatter{

    /// Checks whether a ray hits the material and has enough
    /// power in order not to be absorbed. Feedback is sent
    /// with collision data.
    ///
    /// # Parameters
    ///
    /// * `self` - Material struct.
    /// * `ray_in` - Incoming ray.
    /// * `hit_rec` - Information about the surface-ray hit point.
    /// * `attenuation` - How much the output ray is attenuated.
    /// * `scattered` - Reflected or refracted ray.
    ///
    /// # Returns
    ///
    /// * bool - Whether the ray is successfully reflected or refracted.
    fn scatter(
        self,
        ray_in: &Ray,
        hit_rec: &HitRecord,
        attenuation: &mut Vec3,
        scattered: &mut Ray) -> bool;
}

/// Lambertian surface constructor trait implementation.
impl LambertianKind{
    pub fn new(albedo: Vec3) -> LambertianKind { LambertianKind {albedo} }
}

impl MetalKind{
    /// Metallic surface constructor.
    pub fn new(albedo: Vec3, fuzz: f32) -> MetalKind { MetalKind {albedo, fuzz} }

    /// Reflected ray's direction:
    /// $$\vec v_{out} = \vec v_{in} - 2 v_{norm}\cdot\vec n$$
    /// # Parameters
    /// * `v` - Input ray's direction.
    /// * `n` - Surface's normal direction.
    ///
    /// # Returns
    /// * `Vec3` - Reflected ray's direction.
    pub fn reflect(v: Vec3, n: Vec3) -> Vec3 { v - 2e0 * n.dot(&v) * n }
}

impl DielectricKind{
    /// Dielectric surface constructor implementation.
    pub fn new(n: f32) -> DielectricKind{ DielectricKind{n} }

    /// Dielectric refraction:
    ///
    /// $$\eta'\sin\theta'=\eta\sin\theta$$
    /// $$\cos\theta=-\vec v_{in}\cdot\vec n$$
    /// $$\vec v_{out} = \frac{\eta}{\eta'}\left(\vec n\times\left(-\vec n\times\vec v_{in}\right)\right)-\left[1-\left(\frac{\eta}{\eta'}\right)^2\left|\vec n\times\vec v_{in}\right|^2\right]^{\frac 1 2}\vec n$$
    ///
    /// # Parameters:
    /// * `v` - Input ray's direction.
    /// * `n` - Surface normal.
    /// * `ni_over_nt` - input over output refractive index fraction.
    /// * `refracted` - refracted direction.
    ///
    /// # Returns:
    /// * `bool` - Whether the ray was refracted or not.
    pub fn refract(
        v: Vec3,
        n: Vec3,
        ni_over_nt: f32,
        refracted: &mut Vec3) -> bool{

        let uv: Vec3 = v.unit_vector();
        let v_out_perp: Vec3 = ni_over_nt * n.cross(&(-n.cross(&uv)));
        let v_out_norm: Vec3 = -(1e0 - ni_over_nt.powi(2) * (n.cross(&uv)).square_length()).sqrt() * n;
        *refracted = v_out_perp + v_out_norm;
        return true;
    }

    /// Polynomial approximation by Christophe Schlick for
    /// ray reflection value instead of full refraction
    /// depending on input angle (cosine).
    ///
    /// # Parameters:
    ///
    /// * `ni_over_nt` - input over output refractive index fraction.
    /// * `cosine` - input ray and surface normal angle's cosine.
    ///
    /// # Returns:
    ///
    /// * `f32` - reflectivity.
    pub fn schlick(ni_over_nt: f32, cosine: f32) -> f32 {
        let r0: f32 = ((1e0 - ni_over_nt) / (1e0 + ni_over_nt)).powi(2);
        r0 + (1e0 - r0) * ((1e0 - cosine).powi(5))
    }
}

/// MaterialScatter implementation for all material kinds.
impl MaterialScatter for Material{
    fn scatter(
        self,
        ray_in: &Ray,
        hit_rec: &HitRecord,
        attenuation: &mut Vec3,
        scattered: &mut Ray) -> bool{
        return match self {
            // Lambertian.
            Material::Lambertian(lambertian) => {

                // New ray origin (hit point).
                scattered.a = hit_rec.p;
                // New ray direction (lambertian).
                scattered.b = hit_rec.normal + random_in_unit_sphere();
                // New attenuation/color.
                attenuation.e = lambertian.albedo.e;
                // New scattered ray always exists.
                true
            }
            // Metallic.
            Material::Metal(metal) => {
                // Reflected ray direction.
                let reflected = MetalKind::reflect(ray_in.direction().unit_vector(), hit_rec.normal);
                // New ray origin (hit point).
                scattered.a = hit_rec.p;
                // New ray direction (reflected + fuzz).
                scattered.b = reflected + random_in_unit_sphere() * metal.fuzz;
                // New attenuation/color.
                attenuation.e = metal.albedo.e;
                // Ray exists if input ray came toward the surface.
                scattered.direction().dot(&hit_rec.normal) > 0e0
            }
            // Dielectric.
            Material::Dielectric(dielectric) => {
                // Reflection case.
                let reflected: Vec3 = MetalKind::reflect(ray_in.direction(), hit_rec.normal);
                // Refraction case. Random initialization, refraction function call sets the real value.
                let mut refracted: Vec3 = Vec3::random();

                attenuation.e = Vec3::new(1e0, 1e0, 1e0).e;
                let outward_normal: Vec3; // = hit_rec.normal;
                let ni_over_nt: f32; // = dielectric.n;
                let reflect_prob: f32;
                let cosine: f32;  //  = -ray_in.direction().unit_vector().dot(&hit_rec.normal);
                let sine: f32; // = (1e0 - cosine.powi(2)).sqrt();

                // Check from and to which ambient the ray is crossing.
                // Ray crossed the material and tries to scape through this surface.
                if ray_in.direction().dot(&hit_rec.normal) > 0e0 {
                    // Surface normal is inwards.
                    outward_normal = - hit_rec.normal;
                    // Refraction index ratio is kept.
                    ni_over_nt = dielectric.n;
                    // cosine = dielectric.n * ray_in.direction().dot(&hit_rec.normal) / ray_in.direction().length();
                    // cosine = ray_in.direction().dot(&hit_rec.normal) / ray_in.direction().length();
                    // cosine = (1e0 - dielectric.n.powi(2)*(1e0 - cosine.powi(2))).sqrt();
                }
                else {
                    // Surface normal is outward (as it is).
                    outward_normal = hit_rec.normal;
                    // Dielectric to air transition, use reciprocal value.
                    ni_over_nt = 1e0 / dielectric.n;
                }

                // Angle cosine an sine.
                cosine = -ray_in.direction().unit_vector().dot(&outward_normal);
                sine = (1e0 - cosine.powi(2)).sqrt();

                // Pure reflection case.
                if ni_over_nt * sine > 1e0 {
                    scattered.a = hit_rec.p;
                    scattered.b = reflected;
                    return true;
                }

                // Reflection probability.
                reflect_prob = DielectricKind::schlick(ni_over_nt, cosine);

                // Refraction occurs.
                if DielectricKind::refract(
                    ray_in.direction().unit_vector(),
                    outward_normal,
                    ni_over_nt,
                    &mut refracted) {
                    // Random reflection.
                    // if rng.gen::<f32>() < reflect_prob {
                    if random::<f32>() < reflect_prob {
                        scattered.a = hit_rec.p;
                        scattered.b = reflected;
                        return true;
                    }
                    // Refraction.
                    else {
                        scattered.a = hit_rec.p;
                        scattered.b = refracted;
                        return true;
                    }
                }
                // No refraction (never occurs).
                else {
                    return false;
                }
            }
        }
    }
}

/// Random 3D point inside unit sphere.
fn random_in_unit_sphere() -> Vec3{
    // Initialization.
    let mut p: Vec3 = Vec3::new(2e0,2e0,2e0);

    // Loop until ``p`` inside sphere.
    while p.square_length() >= 1e0 {
        p = Vec3::random() * 2e0 - Vec3::ones();
    }
    p
}