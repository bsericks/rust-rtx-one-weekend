
pub mod color {

use std::fs::File;
use std::io::BufWriter;
use crate::vec3::Vec3;
use std::io::{Write};
use crate::Interval;

pub type Color = Vec3;


pub fn linear_to_gamma(linear_component: f32) -> f32 
{
    if linear_component > 0.0
    {
        return linear_component.sqrt();
    }

    return 0.0;
}

pub fn write_color( writer: &mut BufWriter<File>, color: Color )
{
    let r = linear_to_gamma(color.x());
    let g = linear_to_gamma(color.y());
    let b = linear_to_gamma(color.z());


    let intensity = Interval::new_with_bounds(0.000, 0.999);
    let rbyte = (255.999 * intensity.clamp(r)) as u32;
    let gbyte = (255.999 * intensity.clamp(g)) as u32;
    let bbyte = (255.999 * intensity.clamp(b)) as u32;

    let line = format!("{} {} {} \n", rbyte, gbyte, bbyte); // Construct the line as a String
    let _ = writer.write_all(line.as_bytes()); // Write the line to the file
}

}