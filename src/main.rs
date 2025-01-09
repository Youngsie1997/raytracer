use raytracer::camera::Camera;
use raytracer::HittableList;
use raytracer::Point3;
use raytracer::Sphere;

fn main() {
    println!("Racetracer v1");

    // WORLD

    let mut world = HittableList::new();
    world.add(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5));
    world.add(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0));
    let mut cam: Camera = Camera::new();
    cam.aspect_ratio = 16.0 / 9.0;
    cam.image_width = 400;
    cam.samples_per_pixel = 100;
    cam.max_depth = 50;
    cam.render(&world);
}
