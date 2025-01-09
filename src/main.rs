use std::sync::Arc;

use raytracer::camera::Camera;
use raytracer::lambertian::Lambertian;
use raytracer::materials::metal::Metal;
use raytracer::Colour;
use raytracer::HittableList;
use raytracer::Point3;
use raytracer::Sphere;

fn main() {
    println!("Racetracer v1");

    // WORLD

    let mut world = HittableList::new();
    let material_ground = Arc::new(Lambertian::new(Colour::new(0.8, 0.8, 0.0)));
    let material_center = Arc::new(Lambertian::new(Colour::new(0.1, 0.2, 0.5)));
    let material_left = Arc::new(Metal::new(Colour::new(0.8, 0.8, 0.8)));
    let material_right = Arc::new(Metal::new(Colour::new(0.8, 0.6, 0.2)));
    world.add(Sphere::new(
        Point3::new(0.0, -100.5, -1.0),
        100.0,
        material_ground,
    ));
    world.add(Sphere::new(
        Point3::new(0.0, 0.0, -1.2),
        0.5,
        material_center,
    ));
    world.add(Sphere::new(
        Point3::new(-1.0, 0.0, -1.0),
        0.5,
        material_left,
    ));
    world.add(Sphere::new(
        Point3::new(1.0, 0.0, -1.0),
        0.5,
        material_right,
    ));

    let mut cam: Camera = Camera::new();
    cam.aspect_ratio = 16.0 / 9.0;
    cam.image_width = 400;
    cam.samples_per_pixel = 50;
    cam.max_depth = 50;
    cam.render(&world);
}
