use crate::{Point3, Vec3};

pub struct Ray {
    pub origin: Point3,
    pub dir: Vec3,
    pub time: f64,
}

impl Ray {
    pub fn new(origin: Point3, dir: Vec3, time: f64) -> Self {
        Ray { origin, dir, time }
    }

    pub fn at(&self, t: f64) -> Point3 {
        self.origin + (self.dir * t).conv()
    }
}
