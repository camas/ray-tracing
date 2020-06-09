use crate::material::{Dielectric, Lambertian, Material, Metal};
use crate::ray::Ray;
use crate::{Color, Point3, Vec3};
use rand::distributions::{Distribution, Standard, Uniform};
use rand::rngs::ThreadRng;
use rand::Rng;

/// Container for all objects in a scene
#[derive(Default)]
pub struct World<'a> {
    pub hittables: Vec<Box<dyn Hittable + Sync + 'a>>,
}

impl<'a> World<'a> {
    /// Generates the cover image world
    pub fn cover_world() -> Self {
        let mut world = World::default();

        // Ground
        let material = Lambertian::new(color!(0.5, 0.5, 0.5));
        let shape = Sphere::new(point3!(0., -1000., 0.), 1000., material);
        world.add(shape);

        // Random small balls
        let mut rng = rand::thread_rng();
        for a in -11..11 {
            for b in -11..11 {
                let center = point3!(
                    a as f64 + 0.9 * rng.sample::<f64, _>(Standard),
                    0.2,
                    b as f64 + 0.9 * rng.sample::<f64, _>(Standard)
                );
                if (center - point3!(4., 0.2, 0.)).length() <= 0.9 {
                    continue;
                }

                let mat_choice = rng.sample::<f64, _>(Standard);
                let mat: Box<dyn Material + Sync> = if mat_choice < 0.5 {
                    // Diffuse
                    let albedo = random_color(&mut rng, 0., 1.) * random_color(&mut rng, 0., 1.);
                    Box::new(Lambertian::new(albedo))
                } else if mat_choice < 0.75 {
                    // Metal
                    let albedo = random_color(&mut rng, 0.5, 1.);
                    let fuzz = random_f64(&mut rng, 0., 0.5);
                    Box::new(Metal::new(albedo, fuzz))
                } else {
                    // Glass
                    Box::new(Dielectric::new(1.5))
                };
                let sphere = Sphere::new_boxed(center, 0.2, mat);
                world.add(sphere);
            }
        }

        // Big balls
        world.add(Sphere::new(point3!(0., 1., 0.), 1., Dielectric::new(1.5)));
        world.add(Sphere::new(
            point3!(-4., 1., 0.),
            1.,
            Lambertian::new(color!(0.4, 0.2, 0.1)),
        ));
        world.add(Sphere::new(
            point3!(4., 1., 0.),
            1.,
            Metal::new(color!(0.7, 0.6, 0.5), 0.),
        ));

        world
    }

    pub fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        self.hittables
            .iter()
            .filter_map(|hittable| hittable.as_ref().hit(ray, t_min, t_max))
            .min_by(|rec_a, rec_b| rec_a.t.partial_cmp(&rec_b.t).unwrap())
    }

    pub fn add<T: Hittable + Sync + 'a>(&mut self, hittable: T) {
        self.hittables.push(Box::new(hittable));
    }
}

/// The object can be raytraced
pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
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
}

/// Inefficient way to generate a random color in a range
fn random_color(rng: &mut ThreadRng, from: f64, to: f64) -> Color {
    let dist = Uniform::from(from..to);
    color!(dist.sample(rng), dist.sample(rng), dist.sample(rng))
}

/// Inefficient way to generate a random f64 in a range
fn random_f64(rng: &mut ThreadRng, from: f64, to: f64) -> f64 {
    let dist = Uniform::from(from..to);
    dist.sample(rng)
}
