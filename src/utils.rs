use crate::vec3::Vec3;

pub fn lerp(t: f64, start: Vec3, end: Vec3) -> Vec3 {
    (1.0 - t) * start + t * end
}
