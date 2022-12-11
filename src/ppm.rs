use crate::vec3::Color;

pub fn write_color(color: Color) {
    fn convert_to_int(f: f64) -> u8 {
        (255.999 * f) as u8
    }
    // translate floating point to [0, 255] range of ints
    println!(
        "{} {} {}",
        convert_to_int(color.0),
        convert_to_int(color.1),
        convert_to_int(color.2)
    );
}
