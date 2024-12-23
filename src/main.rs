pub mod vector3;
use core::f64;
use std::fs::File;
use std::io::BufWriter;
use std::io::Write;
use vector3::Vector3;

fn main() {
    println!("Racetracer v1");

    // Image
    let image_width: u32 = 4096;
    let image_height: u32 = 2160;
    let yourvec = Vector3::new(4.0, 5.0, 9.0);
    println!("{yourvec}");
    dbg!(4.0 * yourvec);
    dbg!(yourvec * 4.0);

    // Render

    // Open file
    let file_name = "output.ppm";
    let output_file: File = File::create(file_name)
        .unwrap_or_else(|error| panic!("Error creating file {} ,{}", file_name, error));

    // Header
    let mut output_buffer: BufWriter<File> = BufWriter::new(output_file);
    let _written_bytes = output_buffer
        .write(format!("P3\n{image_width} {image_height}\n255\n").as_bytes())
        .unwrap();

    print!("Rendering");
    for j in 0..image_height {
        print!(".");
        for i in 0..image_width {
            let r = i as f64 / (image_width as f64 - 1.0);
            let g = j as f64 / (image_width as f64 - 1.0);
            let b = 0.0;

            let ir: u32 = (255.999 * r) as u32;
            let ig: u32 = (255.999 * g) as u32;
            let ib: u32 = (255.999 * b) as u32;

            let _bytes_written = output_buffer
                .write(format!("{ir} {ig} {ib}\n").as_bytes())
                .unwrap();
        }
    }
    println!("Done!");

    output_buffer.flush().expect("could not flush to file");
}
