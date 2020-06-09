#[macro_use]
extern crate ray_tracing;

use ray_tracing::camera::CameraSettings;
use ray_tracing::image::Image;
use ray_tracing::material::{Dielectric, Lambertian, Metal};
use ray_tracing::raytrace_image;
use ray_tracing::world::{Sphere, World};
use ray_tracing::{Color, Point3, Vec3};

fn main() {
    let start_time = std::time::Instant::now();

    // Do work
    let camera = CameraSettings::cover_camera();
    let world = World::cover_world();
    let image = raytrace_image(world, camera, 1000, 600);
    //let image = create_cover();
    image.write_png("image.png");

    // Print time
    let end_time = std::time::Instant::now();
    let duration = end_time - start_time;
    println!("Took {:?}", duration);
}

#[allow(dead_code)]
fn create_cover() -> Image {
    let camera = CameraSettings::cover_camera();
    let world = World::cover_world();
    raytrace_image(world, camera, 1920, 1080)
}
