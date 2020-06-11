use crate::{Color, Point3};

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
