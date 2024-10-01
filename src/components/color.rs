use std::fs::File;
use std::io::{self, Write};
use crate::components::vec3::Vec3;

pub type Color = Vec3 ;
pub fn write_color(color: Color, file: &mut File) -> io::Result<()> {

    // Get the color components
    let r = color.x();
    let g = color.y();
    let b = color.z();

    // Convert the color components to the range 0-255
    let ir = (255.999 * r).round() as u32;
    let ig = (255.99 * g).round() as u32;
    let ib = (255.999 * b).round() as u32;

    // Write the color as space-separated values
    writeln!(file, "{} {} {}", ir, ig, ib)?;

    Ok(())
}
