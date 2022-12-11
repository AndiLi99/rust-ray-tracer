use crate::ray::Ray;
use crate::vec3::{Point, Vec3};

use rand::Rng;

#[derive(Debug, Copy, Clone, PartialEq)]

struct Camera {
    aspect_ratio: f64,
    viewport_height: f64,
    viewport_widgth: f64,
    focal_length: f64,
    origin: Point,
    horizontal: Vec3,
    vertical: Vec3,
    lower_left_corner: Point,
}

impl Camera {
    pub fn get_ray(self, u: f64, v: f64) -> Ray {
        Ray::new(
            self.origin,
            self.lower_left_corner + u * self.horizontal + v * self.vertical - self.origin,
        )
    }
}
