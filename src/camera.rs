use crate::ray::Ray;
use crate::{Point3, Vec3};
use rand::distributions::{Distribution, Uniform};
use rand::rngs::ThreadRng;

#[derive(Default)]
pub struct CameraSettings {
    pub look_from: Point3,
    pub look_at: Point3,
    pub vup: Vec3,
    pub vfov: f64,
    pub aperture: f64,
    pub focus_dist: f64,
    pub t0: f64,
    pub t1: f64,
}

impl CameraSettings {
    pub fn cover_camera() -> Self {
        CameraSettings {
            look_from: point3!(13., 2., 3.),
            look_at: point3!(),
            vup: vec3!(0., 1., 0.),
            vfov: 20.,
            aperture: 0.1,
            focus_dist: 10.,
            t0: 0.,
            t1: 1.,
        }
    }
}

pub struct Camera {
    origin: Point3,
    lower_left_corner: Point3,
    horizontal: Vec3,
    vertical: Vec3,
    u: Point3,
    v: Point3,
    lens_radius: f64,
    time_dist: Uniform<f64>,
}

impl Camera {
    /// Creates a new camera
    ///
    /// # Args
    ///
    /// - `look_from` - The position of the camera
    ///
    /// - `look_at` - The point the camera is looking at
    ///
    /// - `vup` - The vertial up of the camera. Use (0, 1, 0) if you don't know what this is
    ///
    /// - `vfov` - Vertical field of view in degrees
    ///
    /// - `aspect_ratio` - The aspect ratio
    ///
    /// - `aperture`
    ///
    /// - `focus_dist`
    pub fn new(settings: &CameraSettings, aspect_ratio: f64) -> Self {
        let theta = settings.vfov.to_radians();
        let h = (theta / 2.).tan();
        let viewport_height = 2. * h;
        let viewport_width = aspect_ratio * viewport_height;

        let w = (settings.look_from - settings.look_at).unit_vector();
        let u = settings.vup.conv::<Point3>().cross(&w).unit_vector();
        let v = w.cross(&u);

        let origin = settings.look_from;
        let horizontal = settings.focus_dist * viewport_width * u;
        let vertical = settings.focus_dist * viewport_height * v;
        let lower_left_corner =
            origin - (horizontal / 2.).conv() - (vertical / 2.).conv() - settings.focus_dist * w;

        let time_dist = Uniform::from(settings.t0..settings.t1);

        Camera {
            origin,
            lower_left_corner,
            horizontal: horizontal.conv(),
            vertical: vertical.conv(),
            u,
            v,
            lens_radius: settings.aperture / 2.,
            time_dist,
        }
    }

    pub fn get_ray(&self, s: f64, t: f64, rng: &mut ThreadRng, uniform_unit: &Uniform<f64>) -> Ray {
        let rd = self.lens_radius * random_in_unit_disk(rng, uniform_unit);
        let offset = self.u * rd.x + self.v * rd.y;
        let time = self.time_dist.sample(rng);
        Ray {
            origin: self.origin + offset,
            dir: self.lower_left_corner.conv::<Vec3>() + s * self.horizontal + t * self.vertical
                - self.origin.conv()
                - offset.conv(),
            time,
        }
    }
}

fn random_in_unit_disk(rng: &mut ThreadRng, uniform_unit: &Uniform<f64>) -> Vec3 {
    loop {
        let tmp = vec3!(uniform_unit.sample(rng), uniform_unit.sample(rng), 0.);
        if tmp.length_squared() < 1. {
            return tmp;
        }
    }
}
