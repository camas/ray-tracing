use crate::ray::Ray;
use crate::{Point3, Vec3};
use rand::distributions::{Distribution, Uniform};
use rand::rngs::ThreadRng;

#[derive(Default)]
pub struct Camera {
    origin: Point3,
    lower_left_corner: Point3,
    horizontal: Vec3,
    vertical: Vec3,
    u: Point3,
    v: Point3,
    lens_radius: f64,
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
    pub fn new(
        look_from: Point3,
        look_at: Point3,
        vup: Vec3,
        vfov: f64,
        aspect_ratio: f64,
        aperture: f64,
        focus_dist: f64,
    ) -> Self {
        let theta = vfov.to_radians();
        let h = (theta / 2.).tan();
        let viewport_height = 2. * h;
        let viewport_width = aspect_ratio * viewport_height;

        let w = (look_from - look_at).unit_vector();
        let u = vup.conv::<Point3>().cross(&w).unit_vector();
        let v = w.cross(&u);

        let origin = look_from;
        let horizontal = focus_dist * viewport_width * u;
        let vertical = focus_dist * viewport_height * v;
        let lower_left_corner =
            origin - (horizontal / 2.).conv() - (vertical / 2.).conv() - focus_dist * w;

        Camera {
            origin,
            lower_left_corner,
            horizontal: horizontal.conv(),
            vertical: vertical.conv(),
            u,
            v,
            lens_radius: aperture / 2.,
        }
    }

    pub fn cover_camera() -> Self {
        Camera::new(
            point3!(13., 2., 3.),
            point3!(),
            vec3!(0., 1., 0.),
            20.,
            16. / 9.,
            0.1,
            10.,
        )
    }

    pub fn get_ray(&self, s: f64, t: f64, rng: &mut ThreadRng, uniform_unit: &Uniform<f64>) -> Ray {
        let rd = self.lens_radius * random_in_unit_disk(rng, uniform_unit);
        let offset = self.u * rd.x + self.v * rd.y;
        Ray {
            origin: self.origin + offset,
            dir: self.lower_left_corner.conv::<Vec3>() + s * self.horizontal + t * self.vertical
                - self.origin.conv()
                - offset.conv(),
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
