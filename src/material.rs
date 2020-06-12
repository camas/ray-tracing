use crate::hittable::HitRecord;
use crate::ray::Ray;
use crate::texture::Texture;
use crate::{rand_unit_vector, schlick};
use crate::{Color, Point3};
use rand::distributions::{Standard, Uniform};
use rand::rngs::ThreadRng;
use rand::Rng;

pub trait Material {
    fn scatter(
        &self,
        ray: &Ray,
        rec: &HitRecord,
        rng: &mut ThreadRng,
        uniform_unit: &Uniform<f64>,
    ) -> Option<(Ray, Color)>;

    fn emitted(&self, u: f64, v: f64, point: Point3) -> Color {
        color!(0., 0., 0.)
    }
}

pub struct Lambertian<'a> {
    albedo: Box<dyn Texture + Sync + 'a>,
}

impl<'a> Lambertian<'a> {
    pub fn new<T: Texture + Sync + 'a>(albedo: T) -> Self {
        Self {
            albedo: Box::new(albedo),
        }
    }
}

impl<'a> Material for Lambertian<'a> {
    fn scatter(
        &self,
        ray: &Ray,
        rec: &HitRecord,
        rng: &mut ThreadRng,
        uniform_unit: &Uniform<f64>,
    ) -> Option<(Ray, Color)> {
        let target: Point3 = rec.point + rec.normal.conv() + rand_unit_vector(rng, uniform_unit);
        let ray = Ray {
            origin: rec.point,
            dir: (target - rec.point).conv(),
            time: ray.time,
        };
        Some((ray, self.albedo.value(rec.u, rec.v, rec.point)))
    }
}

pub struct Metal {
    albedo: Color,
    fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Color, fuzz: f64) -> Self {
        Self {
            albedo,
            fuzz: fuzz.min(1.),
        }
    }
}

impl Material for Metal {
    fn scatter(
        &self,
        ray: &Ray,
        rec: &HitRecord,
        rng: &mut ThreadRng,
        uniform_unit: &Uniform<f64>,
    ) -> Option<(Ray, Color)> {
        let reflected = ray.dir.unit_vector().reflect(&rec.normal);
        let ray = Ray {
            origin: rec.point,
            dir: reflected + (self.fuzz * rand_unit_vector(rng, uniform_unit)).conv(),
            time: ray.time,
        };
        if ray.dir.dot(&rec.normal) <= 0. {
            return None;
        }
        Some((ray, self.albedo))
    }
}

pub struct Dielectric {
    ri: f64,
}

impl Dielectric {
    pub fn new(ri: f64) -> Self {
        Self { ri }
    }
}

impl Material for Dielectric {
    fn scatter(
        &self,
        ray: &Ray,
        rec: &HitRecord,
        rng: &mut ThreadRng,
        _: &Uniform<f64>,
    ) -> Option<(Ray, Color)> {
        let attuen = Color::new(1., 1., 1.);
        let etai_over_etat = if rec.front_face {
            1. / self.ri
        } else {
            self.ri
        };
        let unit_dir = ray.dir.unit_vector();
        let cos_theta = (-unit_dir).dot(&rec.normal).min(1.0);
        let sin_theta = (1. - cos_theta * cos_theta).sqrt();
        let dir = if etai_over_etat * sin_theta > 1. {
            unit_dir.reflect(&rec.normal)
        } else {
            let reflect_prob = schlick(cos_theta, self.ri);
            if rng.sample::<f64, _>(Standard) < reflect_prob {
                unit_dir.reflect(&rec.normal)
            } else {
                unit_dir.refract(&rec.normal, etai_over_etat)
            }
        };
        let ray = Ray {
            origin: rec.point,
            dir,
            time: ray.time,
        };
        Some((ray, attuen))
    }
}

pub struct Light<'a> {
    albedo: Box<dyn Texture + Sync + 'a>,
    color: Color,
}

impl<'a> Light<'a> {
    pub fn new<T: Texture + Sync + 'a>(albedo: T, color: Color) -> Self {
        Self {
            albedo: Box::new(albedo),
            color,
        }
    }
}

impl<'a> Material for Light<'a> {
    fn scatter(
        &self,
        ray: &Ray,
        rec: &HitRecord,
        rng: &mut ThreadRng,
        uniform_unit: &Uniform<f64>,
    ) -> Option<(Ray, Color)> {
        return None;
        let target: Point3 = rec.point + rec.normal.conv() + rand_unit_vector(rng, uniform_unit);
        let ray = Ray {
            origin: rec.point,
            dir: (target - rec.point).conv(),
            time: ray.time,
        };
        Some((ray, self.albedo.value(rec.u, rec.v, rec.point)))
    }

    fn emitted(&self, _: f64, _: f64, _: Point3) -> Color {
        self.color
    }
}
