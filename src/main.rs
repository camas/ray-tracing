#[macro_use]
extern crate ray_tracing;

use ray_tracing::camera::CameraSettings;
use ray_tracing::image::Image;
use ray_tracing::material::{Dielectric, Lambertian, Metal};
use ray_tracing::raytrace_image;
use ray_tracing::world::World;
use ray_tracing::{Color, Point3, Vec3};

fn main() {
    let start_time = std::time::Instant::now();

    // Do work
    let camera = CameraSettings {
        look_from: point3!(10., 4., 0.),
        look_at: point3!(),
        vup: vec3!(0., 1., 0.),
        vfov: 90.,
        aperture: 0.1,
        focus_dist: 8.,
        t0: 0.,
        t1: 1.,
    };
    let world = World::earth();
    let image = raytrace_image(world, camera, 1920, 1080);
    //let image = create_cover();
    image.write_png("image.png");

    // Print time
    let end_time = std::time::Instant::now();
    let duration = end_time - start_time;
    println!("Took {:?}", duration);
}
