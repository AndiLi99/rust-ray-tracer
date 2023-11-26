use crate::hittable::HitRecord;
use crate::ray::Ray;
use crate::vec3::{Color, Vec3};

pub struct ScatterRecord {
    // The scattered output ray
    ray: Ray,
    // The color attenuation indicating color
    attenuation: Color,
}

impl ScatterRecord {
    // Getter method for the ray field
    pub fn ray(&self) -> &Ray {
        &self.ray
    }

    // Getter method for the attenuation field
    pub fn attenuation(&self) -> &Color {
        &self.attenuation
    }
}

pub trait Scatterable {
    fn scatter(self, in_ray: Ray, hit_record: HitRecord) -> ScatterRecord;
}
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Material {
    Lambertian(Lambertian),
}

impl Scatterable for Material {
    fn scatter(self, in_ray: Ray, hit_record: HitRecord) -> ScatterRecord {
        match self {
            Material::Lambertian(l) => l.scatter(in_ray, hit_record),
        }
    }
}
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Lambertian {
    color: Color,
}

impl Scatterable for Lambertian {
    fn scatter(self, in_ray: Ray, hit_record: HitRecord) -> ScatterRecord {
        let direction = hit_record.normal() + Vec3::random_vec_on_unit_sphere();
        ScatterRecord {
            ray: Ray::new(hit_record.p(), direction),
            attenuation: self.color,
        }
    }
}

impl Lambertian {
    pub fn new(color: Color) -> Lambertian {
        Lambertian { color: color }
    }
}