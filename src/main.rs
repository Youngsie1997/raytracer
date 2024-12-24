use core::f64;
use raytracer::write_colour;
use raytracer::Colour;
use std::fs::File;
use std::io::BufWriter;
use std::io::Write;

fn main() {
    println!("Racetracer v1");

    // Image Dimensions
    let image_width: u32 = 64;
    let image_height: u32 = 64;

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
            let r = i as f64 / (image_width as f64 - 1.0);
            let g = j as f64 / (image_width as f64 - 1.0);
            let b = 0.0;

            write_colour(&mut output_buffer, &Colour::new(r, g, b));
        }
    }
    println!("Done!");
    output_buffer.flush().expect("could not flush to file");
}
