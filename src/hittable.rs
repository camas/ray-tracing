use crate::material::{Dielectric, Lambertian, Material, Metal};
use crate::ray::Ray;
use crate::world::AABB;
use crate::{Color, Point3, Vec3};
use rand::distributions::{Distribution, Standard, Uniform};
use rand::rngs::ThreadRng;
use rand::Rng;

/// The object can be raytraced
pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
    fn bounding_box(&self, t0: f64, t1: f64) -> Option<AABB>;
}

/// Records a raytrace hit
pub struct HitRecord<'a> {
    pub point: Point3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
    pub material: &'a dyn Material,
}

/// A sphere
pub struct Sphere<'a> {
    center: Point3,
    radius: f64,
    material: Box<dyn Material + Sync + 'a>,
}

impl<'a> Sphere<'a> {
    pub fn new<T: Material + Sync + 'a>(center: Point3, radius: f64, material: T) -> Self {
        Self {
            center,
            radius,
            material: Box::new(material),
        }
    }

    pub fn new_boxed(center: Point3, radius: f64, material: Box<dyn Material + Sync + 'a>) -> Self {
        Self {
            center,
            radius,
            material,
        }
    }
}

impl<'a> Hittable for Sphere<'a> {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = ray.origin - self.center;
        let a = ray.dir.length_squared();
        let half_b = oc.dot(&ray.dir.conv());
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;
        if discriminant < 0. {
            return None;
        }

        let root = discriminant.sqrt();
        let t = (-half_b - root) / a;
        if t > t_min && t < t_max {
            let point = ray.at(t);
            let normal = (point - self.center) / self.radius;
            let front_face = ray.dir.dot(&normal.conv()) < 0.;
            let normal = if front_face { normal } else { -normal };
            return Some(HitRecord {
                t,
                point,
                normal: normal.conv(),
                front_face,
                material: self.material.as_ref(),
            });
        }

        let t = (-half_b + root) / a;
        if t > t_min && t < t_max {
            let point = ray.at(t);
            let normal = (point - self.center) / self.radius;
            let front_face = ray.dir.dot(&normal.conv()) < 0.;
            let normal = if front_face { normal } else { -normal };
            return Some(HitRecord {
                t,
                point,
                normal: normal.conv(),
                front_face,
                material: self.material.as_ref(),
            });
        }

        None
    }

    fn bounding_box(&self, t0: f64, t1: f64) -> Option<AABB> {
        Some(AABB {
            min: self.center - point3!(self.radius, self.radius, self.radius),
            max: self.center + point3!(self.radius, self.radius, self.radius),
        })
    }
}

pub struct MovingSphere<'a> {
    center0: Point3,
    center1: Point3,
    t0: f64,
    t1: f64,
    radius: f64,
    material: Box<dyn Material + Sync + 'a>,
}

impl<'a> MovingSphere<'a> {
    pub fn new<T: Material + Sync + 'a>(
        center0: Point3,
        center1: Point3,
        t0: f64,
        t1: f64,
        radius: f64,
        material: T,
    ) -> Self {
        Self {
            center0,
            center1,
            t0,
            t1,
            radius,
            material: Box::new(material),
        }
    }

    pub fn new_boxed(
        center0: Point3,
        center1: Point3,
        t0: f64,
        t1: f64,
        radius: f64,
        material: Box<dyn Material + Sync + 'a>,
    ) -> Self {
        Self {
            center0,
            center1,
            t0,
            t1,
            radius,
            material,
        }
    }

    pub fn center(&self, t: f64) -> Point3 {
        self.center0 + ((t - self.t0) / (self.t1 - self.t0)) * (self.center1 - self.center0)
    }
}

impl<'a> Hittable for MovingSphere<'a> {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let center = self.center(ray.time);
        let oc = ray.origin - center;
        let a = ray.dir.length_squared();
        let half_b = oc.dot(&ray.dir.conv());
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;
        if discriminant < 0. {
            return None;
        }

        let root = discriminant.sqrt();
        let t = (-half_b - root) / a;
        if t > t_min && t < t_max {
            let point = ray.at(t);
            let normal = (point - center) / self.radius;
            let front_face = ray.dir.dot(&normal.conv()) < 0.;
            let normal = if front_face { normal } else { -normal };
            return Some(HitRecord {
                t,
                point,
                normal: normal.conv(),
                front_face,
                material: self.material.as_ref(),
            });
        }

        let t = (-half_b + root) / a;
        if t > t_min && t < t_max {
            let point = ray.at(t);
            let normal = (point - center) / self.radius;
            let front_face = ray.dir.dot(&normal.conv()) < 0.;
            let normal = if front_face { normal } else { -normal };
            return Some(HitRecord {
                t,
                point,
                normal: normal.conv(),
                front_face,
                material: self.material.as_ref(),
            });
        }

        None
    }

    fn bounding_box(&self, t0: f64, t1: f64) -> Option<AABB> {
        let box0 = AABB {
            min: self.center(t0) - point3!(self.radius, self.radius, self.radius),
            max: self.center(t0) + point3!(self.radius, self.radius, self.radius),
        };
        let box1 = AABB {
            min: self.center(t1) - point3!(self.radius, self.radius, self.radius),
            max: self.center(t1) + point3!(self.radius, self.radius, self.radius),
        };

        Some(AABB::surrounding_box(box0, box1))
    }
}
