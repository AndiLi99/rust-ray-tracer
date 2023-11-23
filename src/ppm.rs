use crate::vec3::Color;

pub fn write_color(color: Color, samples_per_pixel: i64) -> String{
    fn convert_to_int(f: f64) -> u8 {
        (255.999 * f.clamp(0.0, 0.999)) as u8
    }

    let scaled_color = color * 1.0 / samples_per_pixel as f64;

    let r = scaled_color.0;
    let g = scaled_color.1;
    let b = scaled_color.2;
    // translate floating point to [0, 255] range of ints
    format!(
        "{} {} {}",
        convert_to_int(r),
        convert_to_int(g),
        convert_to_int(b)
    )
}
