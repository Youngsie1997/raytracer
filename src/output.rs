use crate::{Colour, Interval};
use std::io::Write;
use std::{fs::File, io::BufWriter};

pub fn write_colour(output_buffer: &mut BufWriter<File>, pixel_colour: &Colour) {
    let r = pixel_colour.x();
    let g = pixel_colour.y();
    let b = pixel_colour.z();

    const INTENSITY: Interval = Interval {
        min: 0.000,
        max: 0.999,
    };

    let rbyte = (256.0 * INTENSITY.clamp(*r)) as u8;
    let gbyte = (256.0 * INTENSITY.clamp(*g)) as u8;
    let bbyte = (256.0 * INTENSITY.clamp(*b)) as u8;
    let _bytes_written = output_buffer
        .write(format!("{rbyte} {gbyte} {bbyte}\n").as_bytes())
        .unwrap();
}
