use crate::hittable::{HitRecord, Hittable};
use crate::ray::Ray;
use crate::vec3::{Point, Vec3};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Sphere {
    center: Point,
    radius: f64,
}
impl Sphere {
    pub fn new(center: Point, radius: f64) -> Sphere {
        Sphere {
            center: center,
            radius: radius,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let orig_to_center: Vec3 = ray.origin() - self.center;
        let a: f64 = ray.direction().length_squared();
        let half_b: f64 = ray.direction().dot(orig_to_center);
        let c: f64 = orig_to_center.length_squared() - self.radius.powi(2);
        let determinant: f64 = half_b * half_b - a * c;
        if determinant < 0. {
            return None;
        }
        let mut root = (-half_b - determinant.sqrt()) / (a);
        if root < t_min || t_max < root {
            root = (-half_b + determinant.sqrt()) / (a);
            if root < t_min || t_max < root {
                return None;
            }
        }
        let hit_point: Point = ray.at(root);
        let outward_normal: Vec3 = (hit_point - self.center) / self.radius;
        Some(HitRecord::new(ray, root, outward_normal))
    }
}
