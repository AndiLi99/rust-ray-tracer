use crate::ray::Ray;
use crate::vec3::{Point, Vec3};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct HitRecord {
    p: Point,
    normal: Vec3,
    t: f64,
    front_face: bool,
}

impl HitRecord {
    pub fn new(ray: Ray, t: f64, outward_normal: Vec3) -> HitRecord {
        let front_face: bool = ray.direction().dot(outward_normal) <= 0.;
        let normal = if front_face {
            outward_normal
        } else {
            -outward_normal
        };
        HitRecord {
            p: ray.at(t),
            normal: normal,
            t: t,
            front_face: front_face,
        }
    }
    pub fn p(self) -> Point {
        self.p
    }
    pub fn normal(self) -> Vec3 {
        self.normal
    }
    pub fn t(self) -> f64 {
        self.t
    }
    pub fn front_face(self) -> bool {
        self.front_face
    }
}

pub trait Hittable {
    fn hit(&self, ray: Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}
