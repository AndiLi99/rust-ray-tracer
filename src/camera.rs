use crate::ray::Ray;
use crate::vec3::{Point, Vec3};

#[derive(Debug, Copy, Clone, PartialEq)]

pub struct Camera {
    aspect_ratio: f64,
    viewport_height: f64,
    viewport_width: f64,
    focal_length: f64,
    origin: Point,
    horizontal: Vec3,
    vertical: Vec3,
    lower_left_corner: Point,
    // camera frame basis vectors
    u: Vec3,
    v: Vec3,
    w: Vec3
}
///
/// vertical fov = degrees of vertical view
impl Camera {
    pub fn new(aspect_ratio: f64, v_fov: f64, lookfrom: Point, lookat: Point, vup: Vec3) -> Camera {
        // ration of height to focal length
        let focal_length = (lookfrom - lookat).length();

        let height_ratio = (v_fov.to_radians()/2.0).tan();
        let viewport_height = 2.0*height_ratio*focal_length;
        let viewport_width: f64 = aspect_ratio * viewport_height;
        
        let w = (lookfrom - lookat).unit_vector();
        let u = Vec3::cross(vup, w).unit_vector();
        let v = Vec3::cross(w, u);

        let horizontal = viewport_width*u;
        let vertical = viewport_height*v;
        let lower_left_corner =
            lookfrom - horizontal / 2.0 - vertical / 2.0 - focal_length*w;

        Camera {
            aspect_ratio,
            viewport_height,
            viewport_width: aspect_ratio * viewport_height,
            focal_length,
            origin: lookfrom,
            horizontal,
            vertical,
            lower_left_corner,
            u,
            v,
            w,
        }
    }
    pub fn get_ray(self, u: f64, v: f64) -> Ray {
        Ray::new(
            self.origin,
            self.lower_left_corner + u * self.horizontal + v * self.vertical - self.origin,
        )
    }
}
