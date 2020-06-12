use crate::hittable::{HitRecord, Hittable, MovingSphere, Sphere};
use crate::material::{Dielectric, Lambertian, Light, Material, Metal};
use crate::ray::Ray;
use crate::texture::{Checker, ImageTexture, SolidColor, StarTexture};
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
        let texture = SolidColor::new(color!(0.5, 0.5, 0.5));
        let material = Lambertian::new(texture);
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
                    let albedo = SolidColor::new(albedo);
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
            Lambertian::new(SolidColor::new(color!(0.4, 0.2, 0.1))),
        ));
        world.add(Sphere::new(
            point3!(4., 1., 0.),
            1.,
            Metal::new(color!(0.7, 0.6, 0.5), 0.),
        ));

        world
    }

    /// Generates the cover image world with some moving balls
    pub fn moving_cover_world() -> Self {
        let mut world = World::default();

        // Ground
        let material = Lambertian::new(SolidColor::new(color!(0.5, 0.5, 0.5)));
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
                    Box::new(Lambertian::new(SolidColor::new(albedo)))
                } else if mat_choice < 0.75 {
                    // Metal
                    let albedo = random_color(&mut rng, 0.5, 1.);
                    let fuzz = random_f64(&mut rng, 0., 0.5);
                    Box::new(Metal::new(albedo, fuzz))
                } else {
                    // Glass
                    Box::new(Dielectric::new(1.5))
                };
                if rng.gen_bool(0.25) {
                    let center1 = center + point3!(0., random_f64(&mut rng, 0.1, 0.3), 0.);
                    let sphere = MovingSphere::new_boxed(center, center1, 0., 1., 0.2, mat);
                    world.add(sphere);
                } else {
                    let sphere = Sphere::new_boxed(center, 0.2, mat);
                    world.add(sphere);
                }
            }
        }

        // Big balls
        world.add(Sphere::new(point3!(0., 1., 0.), 1., Dielectric::new(1.5)));
        world.add(Sphere::new(
            point3!(-4., 1., 0.),
            1.,
            Lambertian::new(SolidColor::new(color!(0.4, 0.2, 0.1))),
        ));
        world.add(Sphere::new(
            point3!(4., 1., 0.),
            1.,
            Metal::new(color!(0.7, 0.6, 0.5), 0.),
        ));

        world
    }

    /// Generates the cover image world with some moving balls
    pub fn checkered_cover_world() -> Self {
        let mut world = World::default();

        // Ground
        let texture = Checker::new(color!(0.2, 0.3, 0.1), color!(0.9, 0.9, 0.9));
        let material = Lambertian::new(texture);
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
                    Box::new(Lambertian::new(SolidColor::new(albedo)))
                } else if mat_choice < 0.75 {
                    // Metal
                    let albedo = random_color(&mut rng, 0.5, 1.);
                    let fuzz = random_f64(&mut rng, 0., 0.5);
                    Box::new(Metal::new(albedo, fuzz))
                } else {
                    // Glass
                    Box::new(Dielectric::new(1.5))
                };
                if rng.gen_bool(0.25) {
                    let center1 = center + point3!(0., random_f64(&mut rng, 0.1, 0.3), 0.);
                    let sphere = MovingSphere::new_boxed(center, center1, 0., 1., 0.2, mat);
                    world.add(sphere);
                } else {
                    let sphere = Sphere::new_boxed(center, 0.2, mat);
                    world.add(sphere);
                }
            }
        }

        // Big balls
        world.add(Sphere::new(point3!(0., 1., 0.), 1., Dielectric::new(1.5)));
        world.add(Sphere::new(
            point3!(-4., 1., 0.),
            1.,
            Lambertian::new(SolidColor::new(color!(0.4, 0.2, 0.1))),
        ));
        world.add(Sphere::new(
            point3!(4., 1., 0.),
            1.,
            Metal::new(color!(0.7, 0.6, 0.5), 0.),
        ));

        world
    }

    pub fn earth() -> Self {
        // Earth
        let mut world = World::default();
        let texture = ImageTexture::new("textures/earthmap.jpg");
        let material = Lambertian::new(texture);
        let shape = Sphere::new(point3!(0., 0., 0.), 2., material);
        world.add(shape);
        // Ground
        let texture = SolidColor::new(color!(0.1, 0.1, 0.1));
        let material = Lambertian::new(texture);
        let shape = Sphere::new(point3!(0., -1005., 0.), 1000., material);
        world.add(shape);
        // Light
        let texture = SolidColor::new(color!(1., 0., 0.));
        let material = Light::new(texture, color!(100., 20., 20.));
        let shape = Sphere::new(point3!(0., 3., 1.), 1., material);
        world.add(shape);
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

#[derive(Clone)]
pub struct AABB {
    pub min: Point3,
    pub max: Point3,
}

impl AABB {
    pub fn new(min: Point3, max: Point3) -> Self {
        Self { min, max }
    }

    pub fn surrounding_box(box_a: &Self, box_b: &Self) -> Self {
        let min = Point3::new(
            box_a.min.x.min(box_b.min.x),
            box_a.min.y.min(box_b.min.y),
            box_a.min.z.min(box_b.min.z),
        );
        let max = Point3::new(
            box_a.max.x.max(box_b.max.x),
            box_a.max.y.max(box_b.max.y),
            box_a.max.z.max(box_b.max.z),
        );
        AABB { min, max }
    }

    pub fn surrounding_option(box_a: Option<Self>, box_b: Option<Self>) -> Self {
        if let Some(box_a) = box_a {
            if let Some(box_b) = box_b {
                return Self::surrounding_box(&box_a, &box_b);
            }
            return box_a;
        }
        if let Some(box_b) = box_b {
            return box_b;
        }
        panic!("No bounding box!");
    }

    pub fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> bool {
        for i in 0..3 {
            let inv_d = 1. / ray.dir[i];
            let mut t0 = (self.min[i] - ray.origin[i]) * inv_d;
            let mut t1 = (self.max[i] - ray.origin[i]) * inv_d;
            if inv_d < 0. {
                t1 = std::mem::replace(&mut t0, t1)
            }
            let t_min = if t0 > t_min { t0 } else { t_min };
            let t_max = if t1 < t_max { t1 } else { t_max };
            if t_max < t_min {
                return false;
            }
        }
        true
    }
}

pub struct BvhNode<'a> {
    pub left: Box<dyn Hittable + Sync + 'a>,
    pub right: Box<dyn Hittable + Sync + 'a>,
    pub bounding_box: AABB,
}

impl<'a> BvhNode<'a> {
    /// Creates a search tree from a list of `Hittable`s
    ///
    /// Works recursively
    pub fn make_tree(
        mut hittables: Vec<Box<dyn Hittable + Sync + 'a>>,
        t0: f64,
        t1: f64,
        rng: &mut ThreadRng,
    ) -> BvhNode<'a> {
        let dim: usize = rng.gen_range(0, 3);
        hittables.sort_by(|a, b| {
            a.bounding_box(t0, t1).unwrap().min[dim]
                .partial_cmp(&b.bounding_box(t0, t1).unwrap().min[dim])
                .unwrap()
        });
        match hittables.len() {
            1 => panic!(),
            2 => {
                let left = hittables.pop().unwrap();
                let right = hittables.pop().unwrap();
                let bounding_box =
                    AABB::surrounding_option(left.bounding_box(t0, t1), right.bounding_box(t0, t1));

                BvhNode {
                    left,
                    right,
                    bounding_box,
                }
            }
            3 => {
                let left = hittables.pop().unwrap();
                let right = Self::make_tree(hittables, t0, t1, rng);
                let bounding_box =
                    AABB::surrounding_option(left.bounding_box(t0, t1), right.bounding_box(t0, t1));
                BvhNode {
                    left,
                    right: Box::new(right),
                    bounding_box,
                }
            }
            _ => {
                let mid = hittables.len() / 2;
                let left_hittables = hittables.split_off(mid);
                let right_hittables = hittables;
                let left = Self::make_tree(left_hittables, t0, t1, rng);
                let right = Self::make_tree(right_hittables, t0, t1, rng);
                let bounding_box =
                    AABB::surrounding_option(left.bounding_box(t0, t1), right.bounding_box(t0, t1));
                BvhNode {
                    left: Box::new(left),
                    right: Box::new(right),
                    bounding_box,
                }
            }
        }
    }
}

impl<'a> Hittable for BvhNode<'a> {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        if !self.bounding_box.hit(ray, t_min, t_max) {
            return None;
        }

        let left_hit = self.left.hit(ray, t_min, t_max);
        let right_hit = self.right.hit(ray, t_min, t_max);
        match (left_hit, right_hit) {
            (Some(left_rec), Some(right_rec)) => {
                if left_rec.t < right_rec.t {
                    Some(left_rec)
                } else {
                    Some(right_rec)
                }
            }
            (Some(left_rec), None) => Some(left_rec),
            (None, Some(right_rec)) => Some(right_rec),
            (None, None) => None,
        }
    }

    fn bounding_box(&self, t0: f64, t1: f64) -> Option<AABB> {
        Some(self.bounding_box.clone())
    }
}
