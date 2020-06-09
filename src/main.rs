#[macro_use]
extern crate ray_tracing;

use ray_tracing::camera::Camera;
use ray_tracing::material::{Dielectric, Lambertian, Metal};
use ray_tracing::raytrace_image;
use ray_tracing::world::{Sphere, World};
use ray_tracing::{Color, Point3, Vec3};

fn main() {
    let camera = Camera::new(
        point3!(8., 0.4, 1.3),
        point3!(-3., 0.4, 0.),
        vec3!(0., 1., 0.),
        20.,
        16. / 9.,
        0.1,
        8.,
    );

    let world = World::cover_world();
    let image = raytrace_image(world, camera);
    image.write_png("image.png");
}
