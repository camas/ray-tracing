#[macro_use]
extern crate ray_tracing;

use ray_tracing::camera::CameraSettings;
use ray_tracing::material::{Dielectric, Lambertian, Metal};
use ray_tracing::raytrace_image;
use ray_tracing::world::{Sphere, World};
use ray_tracing::{Color, Point3, Vec3};

fn main() {
    let camera = CameraSettings::cover_camera();
    let world = World::cover_world();
    let image = raytrace_image(world, camera, 352, 240);
    image.write_png("image.png");
}
