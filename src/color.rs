use crate::vec3::Color;
use crate::util;

pub fn write_color(buf: &mut Vec<u8>, index: usize, pixel_color: Color, samples_per_pixel: i32) -> std::io::Result<()> {
    let mut r = pixel_color.x() as i32;
    let mut g = pixel_color.y() as i32;
    let mut b = pixel_color.z() as i32;

    // Divide the color by the number of samples and gamma-correct for gamma=2.0.
    let scale = 1.0 / samples_per_pixel as f64;

    // Write the translated [0,255] value of each color component.
    r = (util::clamp(f64::sqrt(r as f64 * scale), 0.0, 0.999) * 256.0) as i32;
    g = (util::clamp(f64::sqrt(g as f64 * scale), 0.0, 0.999) * 256.0) as i32;
    b = (util::clamp(f64::sqrt(b as f64 * scale), 0.0, 0.999) * 256.0) as i32;

    buf[index * 3 + 0] = r.to_ne_bytes()[0];
    buf[index * 3 + 1] = g.to_ne_bytes()[0];
    buf[index * 3 + 2] = b.to_ne_bytes()[0];
    Ok(())
}
