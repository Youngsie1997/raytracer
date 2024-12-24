use super::Colour;
use std::io::Write;
use std::{fs::File, io::BufWriter};

pub fn write_colour(output_buffer: &mut BufWriter<File>, pixel_colour: &Colour) {
    let r = (255.999 * pixel_colour.r()) as u8;
    let g = (255.999 * pixel_colour.g()) as u8;
    let b = (255.999 * pixel_colour.b()) as u8;

    let _bytes_written = output_buffer
        .write(format!("{r} {g} {b}\n").as_bytes())
        .unwrap();
}
