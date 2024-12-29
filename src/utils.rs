use crate::vec3::Vec3;
use rand::Rng;

pub fn lerp(t: f64, start: Vec3, end: Vec3) -> Vec3 {
    (1.0 - t) * start + t * end
}

pub fn random_double() -> f64 {
    rand::thread_rng().gen()
}

pub fn random_double_between(min: f64, max: f64) -> f64 {
    rand::thread_rng().gen_range(min..max)
}