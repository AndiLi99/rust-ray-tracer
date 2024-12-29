use crate::ray::Ray;
use crate::vec3::{Point, Vec3};

#[derive(Debug, Copy, Clone, PartialEq)]

pub struct Camera {
    aspect_ratio: f64,
    viewport_height: f64,
    viewport_width: f64,
    focus_distance: f64,
    origin: Point,
    horizontal: Vec3,
    vertical: Vec3,
    lower_left_corner: Point,
    // camera frame basis vectors
    u: Vec3,
    v: Vec3,
    w: Vec3,
    defocus_angle: f64,
    defocus_disk_u: Vec3,
    defocus_disk_v: Vec3,
}
///
/// vertical fov = degrees of vertical view
impl Camera {
    pub fn new_from_direction(aspect_ratio: f64, v_fov: f64, defocus_angle: f64, focus_distance: f64, lookfrom: Point, direction: Vec3, vup: Vec3) -> Camera{
        let lookat = lookfrom + direction.unit_vector()*focus_distance;
        Camera::new(aspect_ratio, v_fov, defocus_angle, focus_distance, lookfrom, lookat, vup)
    }
    pub fn new(aspect_ratio: f64, v_fov: f64, defocus_angle: f64, focus_distance: f64, lookfrom: Point, lookat: Point, vup: Vec3) -> Camera {
        // ration of height to focal focus distance
        let height_ratio = (v_fov.to_radians()/2.0).tan();
        let viewport_height = 2.0*height_ratio*focus_distance;
        let viewport_width: f64 = aspect_ratio * viewport_height;
        
        let w = (lookfrom - lookat).unit_vector();
        let u = Vec3::cross(vup, w).unit_vector();
        let v = Vec3::cross(w, u);

        let horizontal = viewport_width*u;
        let vertical = viewport_height*v;
        let lower_left_corner =
            lookfrom - horizontal / 2.0 - vertical / 2.0 - focus_distance*w;

        // Calculate the camera defocus disk basis vectors.
        let defocus_radius = focus_distance * (defocus_angle / 2.).to_radians().tan();
        let defocus_disk_u = u*defocus_radius;
        let defocus_disk_v = v*defocus_radius;

        Camera {
            aspect_ratio,
            viewport_height,
            viewport_width: aspect_ratio * viewport_height,
            focus_distance,
            origin: lookfrom,
            horizontal,
            vertical,
            lower_left_corner,
            u,
            v,
            w,
            defocus_angle,
            defocus_disk_u,
            defocus_disk_v,
        }
    }
    pub fn get_ray(self, u: f64, v: f64) -> Ray {
        let origin = if self.defocus_angle < 0. {
            self.origin
        } else {
            self.defocus_disk_sample()
        };

        Ray::new(
            origin,
            self.lower_left_corner + u * self.horizontal + v * self.vertical - origin,
        )
    }
    fn defocus_disk_sample(self) -> Point{
        let p = Vec3::random_vec_in_unit_disk();
        self.origin + p.0 * self.defocus_disk_u + p.1 * self.defocus_disk_v
    }
}
