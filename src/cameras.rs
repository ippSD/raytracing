use crate::vectors::{Vec3, Vec3Methods};
use crate::rays::Ray;
use rand::Rng;

pub enum Camera {
    Simple(CameraSimple),
    Focus(CameraFocus),
}

impl CameraRay for Camera {
    fn get_ray(&self, s: f32, t: f32) -> Ray {
        match self {
            Camera::Simple(simple) => simple.get_ray(s, t),
            Camera::Focus(focus) => focus.get_ray(s, t),
        }
    }
}

pub struct CameraSimple {
    origin: Vec3,
    lower_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3
}

impl CameraSimple {
    /**
    # Arguments
    * `look_from` - camera origin.
    * `look_at` - point at which the camera points.
    * `vup` - world's 'up' vector. Camera's vertical horizontal axis will be parallel to the world.
    * `vfov` - FOV in degrees along vertical axis.
    * `aspect` - Height to Width ratio.
    */
    pub fn new(look_from: Vec3, look_at: Vec3, vup: Vec3, vfov: f32, aspect: f32) -> CameraSimple {
        let w: Vec3 = (look_from - look_at).unit_vector();
        let u: Vec3 = vup.cross(&w).unit_vector();
        let v: Vec3 = w.cross(&u).unit_vector();
        let theta: f32 = vfov * std::f32::consts::PI / 180e0;
        let half_height: f32 = (theta/2e0).tan();
        let half_width: f32 = aspect * half_height;
        CameraSimple {
        origin: look_from,
        lower_left_corner: look_from - u * half_width - v * half_height - w,
        horizontal: u * half_width * 2e0,
        vertical: v * half_height * 2e0,
        }
    }
}

impl CameraRay for CameraSimple {
    fn get_ray(&self, s: f32, t: f32) -> Ray{
        Ray::new(self.origin, self.lower_left_corner + self.horizontal * s + self.vertical * t - self.origin)
    }
}

pub struct CameraFocus{
    origin: Vec3,
    lower_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    u: Vec3,
    v: Vec3,
    _w: Vec3,
    lens_radius: f32,
}

impl CameraFocus{
    /**
    # Arguments
    * `look_from` - camera origin.
    * `look_at` - point at which the camera points.
    * `vup` - world's 'up' vector. Camera's vertical horizontal axis will be parallel to the world.
    * `vfov` - FOV in degrees along vertical axis.
    * `aspect` - Height to Width ratio.
    * `aperture` - len's aperture/diameter.
    * `focus_dist` - distance to the plane that is being focused.
    */
    pub fn new(look_from: Vec3, look_at: Vec3, vup: Vec3, vfov: f32, aspect: f32, aperture: f32, focus_dist: f32) -> CameraFocus {
        let w: Vec3 = (look_from - look_at).unit_vector();
        let u: Vec3 = vup.cross(&w).unit_vector();
        let v: Vec3 = w.cross(&u).unit_vector();
        let theta: f32 = vfov * std::f32::consts::PI / 180e0;
        let half_height: f32 = (theta/2e0).tan();
        let half_width: f32 = aspect * half_height;
        CameraFocus {
            origin: look_from,
            lower_left_corner: look_from - (u * half_width + v * half_height + w) * focus_dist,
            horizontal: u * half_width * focus_dist * 2e0,
            vertical: v * half_height * focus_dist * 2e0,
            u, v, _w: w,
            lens_radius: aperture / 2e0,
        }
    }
}

impl CameraRay for CameraFocus{
    fn get_ray(&self, s: f32, t: f32) -> Ray{
        let rd: Vec3 = random_in_unit_disk() * self.lens_radius;
        let offset: Vec3 = self.u * rd.x() + self.v * rd.y();
        Ray::new(self.origin + offset, self.lower_left_corner + self.horizontal * s + self.vertical * t - self.origin - offset)
    }
}

fn random_in_unit_disk() -> Vec3{
    let mut rng = rand::thread_rng();
    let mut p: Vec3 = Vec3::new(2e0, 2e0, 2e0);
    while p.square_length() >= 1e0 {
        p = Vec3::new(rng.gen::<f32>(), rng.gen::<f32>(), 0e0) * 2e0 - Vec3::new(1e0, 1e0, 0e0);
    }
    return p;
}

pub trait CameraRay{
    fn get_ray(&self, s: f32, t: f32) -> Ray;
}