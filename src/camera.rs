use crate::write_colour;
use std::{
    fs::File,
    io::{BufWriter, Write},
};

use crate::{unit_vector, Colour, HitRecord, Hittable, Interval, Point3, Ray, Vector3};

pub struct Camera {
    pub aspect_ratio: f64,
    pub image_width: i32,
    image_height: u32,
    center: Point3,
    pixel_origin: Point3,
    pixel_delta_u: Vector3,
    pixel_delta_v: Vector3,
}

impl Camera {
    pub fn render<T: Hittable>(&mut self, world: &T) {
        self.initialize();

        // Open file
        let file_name = "output.ppm";
        let output_file: File = File::create(file_name)
            .unwrap_or_else(|error| panic!("Error creating file {} ,{}", file_name, error));

        // Header
        let mut output_buffer: BufWriter<File> = BufWriter::new(output_file);
        let _written_bytes = output_buffer
            .write(format!("P3\n{} {}\n255\n", self.image_width, self.image_height).as_bytes())
            .unwrap();

        // Render image
        print!("Rendering");
        for j in 0..self.image_height {
            print!(".");
            for i in 0..self.image_width {
                let pixel_center = self.pixel_origin
                    + (i as f64 * self.pixel_delta_u)
                    + (j as f64 * self.pixel_delta_v);
                let ray_direction = pixel_center - self.center;
                let r = Ray::new(self.center, ray_direction);
                let pixel_colour = Self::ray_colour(&r, world);
                write_colour(&mut output_buffer, &pixel_colour);
            }
        }
        println!("Done!");
        output_buffer.flush().expect("could not flush to file");
    }

    pub fn new() -> Self {
        Self::default()
    }

    fn initialize(&mut self) {
        self.image_height = u32::max((self.image_width as f64 / self.aspect_ratio) as u32, 1);
        self.center = Point3::default();

        // Determine viewpoint dimensions.
        let focal_length = 1.0;
        let viewport_height = 2.0;
        let viewport_width = viewport_height * (self.image_width as f64 / self.image_height as f64);
        // Calculate vectors accross horizontal and down vertical viewport edges.
        let viewport_u = Vector3::new(viewport_width, 0.0, 0.0);
        let viewport_v = Vector3::new(0.0, -viewport_height, 0.0);
        // Calculte horizontal and vertical delta vectors from pixel to pixel
        self.pixel_delta_u = viewport_u / self.image_width as f64;
        self.pixel_delta_v = viewport_v / self.image_height as f64;
        let viewport_upper_left = self.center
            - Vector3::new(0.0, 0.0, focal_length)
            - viewport_u / 2.0
            - viewport_v / 2.0;
        self.pixel_origin = viewport_upper_left + 0.5 * (self.pixel_delta_u + self.pixel_delta_v);
    }

    pub fn ray_colour<T: Hittable>(r: &Ray, world: &T) -> Colour {
        let mut rec = HitRecord::default();
        if world.hit(r, Interval::build(0.0, f64::INFINITY), &mut rec) {
            return 0.5 * (rec.normal + Colour::new(1.0, 1.0, 1.0));
        }
        let unit_direction = unit_vector(r.direction());
        let a = 0.5 * (unit_direction.y() + 1.0);
        (1.0 - a) * Colour::new(1.0, 1.0, 1.0) + a * Colour::new(1.0, 0.7, 1.0)
    }
}

impl Default for Camera {
    fn default() -> Self {
        Self {
            aspect_ratio: 1.0,
            image_width: 100,
            image_height: Default::default(),
            center: Default::default(),
            pixel_origin: Default::default(),
            pixel_delta_u: Default::default(),
            pixel_delta_v: Default::default(),
        }
    }
}
