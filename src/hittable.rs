use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::{Point, Vec3};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct HitRecord {
    // Location of the hit
    p: Point,
    // Normal vector perpindicular to the plane of hitting.
    // Always points outward to the ray that hit it
    normal: Vec3,
    // What the parameter of t was for the hit by the ray
    t: f64,
    // whether this hit the outward face or the inward face
    front_face: bool,
    material: Material,
}

impl HitRecord {
    pub fn new(ray: Ray, t: f64, outward_normal: Vec3, material: Material) -> HitRecord {
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
            material: material,
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
    pub fn material(self) -> Material {
        self.material
    }
}

pub trait Hittable: Send + Sync {
    fn hit(&self, ray: Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}
