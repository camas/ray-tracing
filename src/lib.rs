use crate::camera::{Camera, CameraSettings};
use crate::image::Image;
use crate::ray::Ray;
use crate::world::World;
use indicatif::ParallelProgressIterator;
use rand::distributions::{Distribution, Standard, Uniform};
use rand::rngs::ThreadRng;
use rand::Rng;
use rayon::prelude::*;
use std::fmt::Display;
use std::iter::Sum;
use std::ops::{Add, AddAssign, Div, DivAssign, Index, Mul, MulAssign, Neg, Sub};

// Create basic Vec3 structs
// They all behave the same but have different identifiers and can't be interchanged directly
pub trait VecType {
    fn from_params(a: f64, b: f64, c: f64) -> Self;
}
macro_rules! vec3_struct {
    ($name:ident, $x:ident, $y:ident, $z:ident, $macro_name:ident) => {
        #[macro_export]
        macro_rules! $macro_name {
            ($a:expr, $b:expr, $c: expr) => {
                $name::new($a, $b, $c)
            };
            () => {
                $name::new(0., 0., 0.)
            };
        }

        #[derive(Clone, Copy, Debug, Default, PartialEq)]
        pub struct $name {
            $x: f64,
            $y: f64,
            $z: f64,
        }

        impl $name {
            /// Initalize with default values
            pub fn new($x: f64, $y: f64, $z: f64) -> Self {
                Self { $x, $y, $z }
            }

            /// Returns the length of the vector
            pub fn length(&self) -> f64 {
                self.length_squared().sqrt()
            }

            pub fn length_squared(&self) -> f64 {
                self.$x * self.$x + self.$y * self.$y + self.$z * self.$z
            }

            /// The dot product
            pub fn dot(&self, other: &Self) -> f64 {
                self.$x * other.$x + self.$y * other.$y + self.$z * other.$z
            }

            /// The cross product
            pub fn cross(&self, other: &Self) -> Self {
                Self {
                    $x: self.$y * other.$z - self.$z * other.$y,
                    $y: self.$z * other.$x - self.$x * other.$z,
                    $z: self.$x * other.$y - self.$y * other.$x,
                }
            }

            /// Unit vector
            pub fn unit_vector(&self) -> Self {
                self.clone() / self.length()
            }

            /// Convert to related types
            pub fn conv<T: VecType>(self) -> T {
                T::from_params(self.$x, self.$y, self.$z)
            }
        }

        impl Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
                write!(f, "{} {} {}", self.$x, self.$y, self.$z)
            }
        }

        impl VecType for $name {
            fn from_params(a: f64, b: f64, c: f64) -> Self {
                Self {
                    $x: a,
                    $y: b,
                    $z: c,
                }
            }
        }

        impl Index<usize> for $name {
            type Output = f64;

            fn index(&self, i: usize) -> &Self::Output {
                match i {
                    0 => &self.$x,
                    1 => &self.$y,
                    2 => &self.$z,
                    _ => panic!("Invalid vec3 index"),
                }
            }
        }

        impl Neg for $name {
            type Output = Self;

            fn neg(self) -> Self::Output {
                Self {
                    $x: -self.$x,
                    $y: -self.$y,
                    $z: -self.$z,
                }
            }
        }

        impl Neg for &$name {
            type Output = $name;

            fn neg(self) -> Self::Output {
                $name {
                    $x: -self.$x,
                    $y: -self.$y,
                    $z: -self.$z,
                }
            }
        }

        impl Add for $name {
            type Output = Self;

            fn add(self, other: Self) -> Self {
                Self {
                    $x: self.$x + other.$x,
                    $y: self.$y + other.$y,
                    $z: self.$z + other.$z,
                }
            }
        }

        impl Add for &$name {
            type Output = $name;

            fn add(self, other: Self) -> Self::Output {
                $name {
                    $x: self.$x + other.$x,
                    $y: self.$y + other.$y,
                    $z: self.$z + other.$z,
                }
            }
        }

        impl Sub for $name {
            type Output = Self;

            fn sub(self, other: Self) -> Self {
                Self {
                    $x: self.$x - other.$x,
                    $y: self.$y - other.$y,
                    $z: self.$z - other.$z,
                }
            }
        }

        impl Mul for $name {
            type Output = Self;

            fn mul(self, value: Self) -> Self::Output {
                Self {
                    $x: self.$x * value.$x,
                    $y: self.$y * value.$y,
                    $z: self.$z * value.$z,
                }
            }
        }

        impl Mul<f64> for $name {
            type Output = Self;

            fn mul(self, value: f64) -> Self::Output {
                Self {
                    $x: self.$x * value,
                    $y: self.$y * value,
                    $z: self.$z * value,
                }
            }
        }

        impl Mul<$name> for f64 {
            type Output = $name;

            fn mul(self, value: $name) -> Self::Output {
                $name {
                    $x: value.$x * self,
                    $y: value.$y * self,
                    $z: value.$z * self,
                }
            }
        }

        impl Mul<&$name> for f64 {
            type Output = $name;

            fn mul(self, value: &$name) -> Self::Output {
                $name {
                    $x: value.$x * self,
                    $y: value.$y * self,
                    $z: value.$z * self,
                }
            }
        }

        impl Div<f64> for $name {
            type Output = Self;

            fn div(self, value: f64) -> Self::Output {
                Self {
                    $x: self.$x / value,
                    $y: self.$y / value,
                    $z: self.$z / value,
                }
            }
        }

        impl AddAssign for $name {
            fn add_assign(&mut self, other: Self) {
                self.$x += other.$x;
                self.$y += other.$y;
                self.$z += other.$z;
            }
        }

        impl MulAssign<f64> for $name {
            fn mul_assign(&mut self, value: f64) {
                self.$x *= value;
                self.$y *= value;
                self.$z *= value;
            }
        }

        impl DivAssign<f64> for $name {
            fn div_assign(&mut self, value: f64) {
                *self *= 1_f64 / value;
            }
        }

        impl Sum<Self> for $name {
            fn sum<I>(iter: I) -> Self
            where
                I: Iterator<Item = Self>,
            {
                iter.fold(Self::default(), |a, b| Self {
                    $x: a.$x + b.$x,
                    $y: a.$y + b.$y,
                    $z: a.$z + b.$z,
                })
            }
        }
    };
}

vec3_struct!(Vec3, x, y, z, vec3);
vec3_struct!(Color, red, green, blue, color);
vec3_struct!(Point3, x, y, z, point3);

impl Vec3 {
    pub fn reflect(&self, normal: &Vec3) -> Vec3 {
        let v = *self;
        v - 2. * v.dot(normal) * normal
    }

    pub fn refract(&self, normal: &Vec3, etai_over_etat: f64) -> Vec3 {
        let uv = *self;
        let cos_theta = (-uv).dot(normal);
        let r_out_parallel = etai_over_etat * (cos_theta * normal + uv);
        let r_out_perp = -(1. - r_out_parallel.length_squared()).sqrt() * normal;
        r_out_parallel + r_out_perp
    }
}

pub mod camera;
pub mod image;
pub mod material;
pub mod ray;
pub mod world;

pub fn raytrace_image(
    world: World,
    camera_settings: CameraSettings,
    image_width: u32,
    image_height: u32,
) -> Image {
    let aspect_ratio = image_width as f64 / image_height as f64;
    let samples_per_pixel = 100;
    let camera = Camera::new(camera_settings, aspect_ratio);

    // Setup progress bar
    let prog_bar = indicatif::ProgressBar::new(image_height as u64);
    prog_bar.set_style(indicatif::ProgressStyle::default_bar().template(
        "Rendering - Done {elapsed:>3} Estimated {eta:>3} {wide_bar} {pos:>4}/{len:4} Lines",
    ));

    let data: Vec<Vec<Color>> = (0..image_height)
        // Parallel iter over each line starting from the top
        .into_par_iter()
        .progress_with(prog_bar)
        .map(|j| {
            let mut rng = rand::thread_rng();
            let uniform_unit = Uniform::from(-1.0..1.0);
            (0..image_width)
                // For each pixel along the line
                .map(|i| {
                    (0..samples_per_pixel)
                        // For each sample
                        .map(|_| {
                            let u = (i as f64 + rng.sample::<f64, _>(Standard))
                                / (image_width - 1) as f64;
                            let v = (j as f64 + rng.sample::<f64, _>(Standard))
                                / (image_height - 1) as f64;
                            let ray = camera.get_ray(u, v, &mut rng, &uniform_unit);
                            ray_color(&ray, &world, &mut rng, &uniform_unit, 0)
                        })
                        .sum::<Color>()
                        / samples_per_pixel as f64
                })
                .collect::<Vec<Color>>()
        })
        .collect();

    let data = data.into_iter().rev().collect();

    Image {
        width: image_width,
        height: image_height,
        data,
    }
}

const MAX_CHILD_RAY_DEPTH: u32 = 50;

fn ray_color(
    ray: &Ray,
    world: &World,
    rng: &mut ThreadRng,
    uniform_unit: &Uniform<f64>,
    depth: u32,
) -> Color {
    if depth >= MAX_CHILD_RAY_DEPTH {
        return color!();
    }
    if let Some(rec) = world.hit(ray, 0.001, std::f64::INFINITY) {
        if let Some((ray, attenuation)) = rec.material.scatter(ray, &rec, rng, uniform_unit) {
            return attenuation * ray_color(&ray, world, rng, uniform_unit, depth + 1);
        }
        return color!();
    }
    let unit_dir = ray.dir.unit_vector();
    let t = 0.5 * (unit_dir.y + 1.);
    ((1.0 - t) * color!(1.0, 1.0, 1.0)) + (t * color!(0.5, 0.7, 1.0))
}

fn rand_unit_vector(rng: &mut ThreadRng, uniform_unit: &Uniform<f64>) -> Point3 {
    let a: f64 = rng.sample::<f64, _>(Standard) * 2. * std::f64::consts::PI;
    let z = uniform_unit.sample(rng);
    let r = (1. - z * z).sqrt();
    Point3 {
        x: r * a.cos(),
        y: r * a.sin(),
        z,
    }
}

fn schlick(cosine: f64, ref_idx: f64) -> f64 {
    let r0 = (1. - ref_idx) / (1. + ref_idx);
    let r0 = r0 * r0;
    r0 + (1. - r0) * (1. - cosine).powf(5.)
}
