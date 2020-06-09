use crate::{Point3, Vec3};

pub struct Ray {
    pub origin: Point3,
    pub dir: Vec3,
}

impl Ray {
    pub fn new(origin: Point3, dir: Vec3) -> Self {
        Ray { origin, dir }
    }

    pub fn at(&self, time: f64) -> Point3 {
        self.origin + (self.dir * time).conv()
    }
}
