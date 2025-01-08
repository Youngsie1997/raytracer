use core::f64;
use raytracer::unit_vector;
use raytracer::write_colour;
use raytracer::Colour;
use raytracer::HitRecord;
use raytracer::Hittable;
use raytracer::HittableList;
use raytracer::Interval;
use raytracer::Point3;
use raytracer::Ray;
use raytracer::Sphere;
use raytracer::Vector3;
use std::fs::File;
use std::io::BufWriter;
use std::io::Write;

fn main() {
    println!("Racetracer v1");

    fn ray_colour<T: Hittable>(r: &Ray, world: &T) -> Colour {
        let mut rec = HitRecord::default();
        if world.hit(r, Interval::build(0.0, f64::INFINITY), &mut rec) {
            return 0.5 * (rec.normal + Colour::new(1.0, 1.0, 1.0));
        }
        let unit_direction = unit_vector(r.direction());
        let a = 0.5 * (unit_direction.y() + 1.0);
        (1.0 - a) * Colour::new(1.0, 1.0, 1.0) + a * Colour::new(1.0, 0.7, 1.0)
    }

    // Image Dimensions
    let aspect_ratio: f64 = 16.0 / 9.0;
    let image_width: u32 = 1920;
    let image_height: u32 = u32::max((image_width as f64 / aspect_ratio) as u32, 1);

    // Camera

    let focal_length = 1.0;
    let viewport_height = 2.0;
    let viewport_width = viewport_height * (image_width as f64 / image_height as f64);
    let camera_center = Point3::default();

    // WORLD

    let mut world = HittableList::new();
    world.add(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5));
    world.add(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0));

    // Calculate vectors accross horizontal and down vertical viewport edges.
    let viewport_u = Vector3::new(viewport_width, 0.0, 0.0);
    let viewport_v = Vector3::new(0.0, -viewport_height, 0.0);

    // Calculte horizontal and vertical delta vectors from pixel to pixel
    let pixel_delta_u = viewport_u / image_width as f64;
    let pixel_delta_v = viewport_v / image_height as f64;

    // Calculate location of upper left pixel
    let viewport_upper_left =
        camera_center - Vector3::new(0.0, 0.0, focal_length) - viewport_u / 2.0 - viewport_v / 2.0;
    let pixel_origin = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

    // Open file
    let file_name = "output.ppm";
    let output_file: File = File::create(file_name)
        .unwrap_or_else(|error| panic!("Error creating file {} ,{}", file_name, error));

    // Header
    let mut output_buffer: BufWriter<File> = BufWriter::new(output_file);
    let _written_bytes = output_buffer
        .write(format!("P3\n{image_width} {image_height}\n255\n").as_bytes())
        .unwrap();

    // Render image
    print!("Rendering");
    for j in 0..image_height {
        print!(".");
        for i in 0..image_width {
            let pixel_center =
                pixel_origin + (i as f64 * pixel_delta_u) + (j as f64 * pixel_delta_v);
            let ray_direction = pixel_center - camera_center;
            let r = Ray::new(camera_center, ray_direction);
            let pixel_colour = ray_colour(&r, &world);
            write_colour(&mut output_buffer, &pixel_colour);
        }
    }
    println!("Done!");
    output_buffer.flush().expect("could not flush to file");
}
