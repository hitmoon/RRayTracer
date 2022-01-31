use std::io::Write;
use crate::vec3::Color;

pub fn write_color<T> (out: &mut T, pixel_color: Color) -> std::io::Result<()> where T: Write {
    let r = (255.999 * pixel_color.x()) as i32;
    let g = (255.999 * pixel_color.y()) as i32;
    let b = (255.999 * pixel_color.z()) as i32;
    out.write(format!("{} {} {}\n", r, g, b).as_bytes())?;
    out.flush()
}
