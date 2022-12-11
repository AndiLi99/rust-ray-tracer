use crate::vec3::Color;

pub fn write_color(color: Color, samples_per_pixel: i64) {
    fn convert_to_int(f: f64) -> u8 {
        (256. * f.clamp(0.0, 0.999)) as u8
    }

    let scaled_color = color * 1.0 / samples_per_pixel as f64;
    // translate floating point to [0, 255] range of ints
    println!(
        "{} {} {}",
        convert_to_int(scaled_color.0),
        convert_to_int(scaled_color.1),
        convert_to_int(scaled_color.2)
    );
}
