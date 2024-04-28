
pub mod color {

use crate::vec3::vec3::Vec3;
use std::io::{self, Write};

pub type Color = Vec3;

pub fn write_color( cout: &mut io::StdoutLock, color: Color )
{
    let r = color.x();
    let g = color.y();
    let b = color.z();

    let rbyte = (255.999 * r) as u32;
    let gbyte = (255.999 * g) as u32;
    let bbyte = (255.999 * b) as u32;

    let _ = cout.write_fmt(format_args!("{} {} {} \n", rbyte, gbyte, bbyte));

}

}