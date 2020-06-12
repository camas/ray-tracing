use crate::{Color, Point3, Vec3};
use rand::distributions::{Distribution, Standard};
use rand::{Rng, SeedableRng};
use std::collections::HashSet;
use std::path::Path;

pub trait Texture {
    fn value(&self, u: f64, v: f64, point: Point3) -> Color;
}

pub struct SolidColor {
    color: Color,
}

impl SolidColor {
    pub fn new(color: Color) -> Self {
        Self { color }
    }
}

impl Texture for SolidColor {
    fn value(&self, _: f64, _: f64, _: Point3) -> Color {
        self.color
    }
}

pub struct Checker {
    odd: Color,
    even: Color,
}

impl Checker {
    pub fn new(odd: Color, even: Color) -> Self {
        Self { odd, even }
    }
}

impl Texture for Checker {
    fn value(&self, u: f64, v: f64, point: Point3) -> Color {
        let sines = (10. * point.x).sin() * (10. * point.y).sin() * (10. * point.z).sin();
        if sines < 0. {
            self.odd
        } else {
            self.even
        }
    }
}

pub struct ImageTexture {
    pub data: image::RgbImage,
}

impl ImageTexture {
    pub fn new<P: AsRef<Path>>(path: P) -> Self {
        let data = image::open(path).expect("Error reading texture image file");
        let data = data.to_rgb();
        ImageTexture { data }
    }
}

impl Texture for ImageTexture {
    fn value(&self, u: f64, v: f64, point: Point3) -> Color {
        // Clamp input coords
        let u = u.max(0.).min(1.);
        let v = 1. - v.max(0.).min(1.);

        // Translate to image coords
        let x = (self.data.width() as f64 * u) as u32;
        let y = (self.data.height() as f64 * v) as u32;

        // Clamp image coords
        let x = if x >= self.data.width() {
            self.data.width() - 1
        } else {
            x
        };
        let y = if y >= self.data.height() {
            self.data.height() - 1
        } else {
            y
        };

        // Get pixel data
        let pixel = self.data.get_pixel(x, y);
        color!(
            pixel.0[0] as f64 / 256.,
            pixel.0[1] as f64 / 256.,
            pixel.0[2] as f64 / 256.
        )
    }
}

pub struct StarTexture {}

impl StarTexture {
    pub fn new(seed: u64, count: u32) -> StarTexture {
        // let mut rng = rand::rngs::StdRng::seed_from_u64(seed);
        // for _ in 0..count {
        //     let u: f64 = rng.sample(Standard);
        //     let v: f64 = rng.sample(Standard);
        //     stars.insert((u.to_be_bytes(), v.to_be_bytes()));
        // }
        StarTexture {}
    }
}

impl Texture for StarTexture {
    fn value(&self, u: f64, v: f64, point: Point3) -> Color {
        if hash_12(u, v) > 0.8 {
            color!(1., 1., 1.)
        } else {
            color!()
        }
    }
}

fn hash_12(a: f64, b: f64) -> f64 {
    let p3: Vec3 = (vec3!(a, b, a) * 0.1031).fract();
    let to_add = p3.dot(&vec3!(p3.y + 33.33, p3.z + 33.33, p3.x + 33.33));
    let p3 = point3!(p3.x + to_add, p3.y + to_add, p3.z + to_add);
    ((p3.x + p3.y) * p3.z).fract()
}
