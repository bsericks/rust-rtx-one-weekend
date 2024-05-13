
pub mod color {

use std::fs::File;
use std::io::BufWriter;
use crate::vec3::Vec3;
use std::io::{Write};

pub type Color = Vec3;

pub fn write_color( writer: &mut BufWriter<File>, color: Color )
{
    let r = color.x();
    let g = color.y();
    let b = color.z();

    let rbyte = (255.999 * r) as u32;
    let gbyte = (255.999 * g) as u32;
    let bbyte = (255.999 * b) as u32;

    let line = format!("{} {} {} \n", rbyte, gbyte, bbyte); // Construct the line as a String
    let _ = writer.write_all(line.as_bytes()); // Write the line to the file
}

}