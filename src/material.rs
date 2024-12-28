use crate::hittable::HitRecord;
use crate::ray::Ray;
use crate::utils;
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
    fn scatter(self, in_ray: Ray, hit_record: HitRecord) -> Option<ScatterRecord>;
}
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Material {
    Lambertian(Lambertian),
    Metal(Metal),
    Dielectric(Dielectric),
}

impl Scatterable for Material {
    fn scatter(self, in_ray: Ray, hit_record: HitRecord) -> Option<ScatterRecord> {
        match self {
            Material::Lambertian(l) => l.scatter(in_ray, hit_record),
            Material::Metal(m) => m.scatter(in_ray, hit_record),
            Material::Dielectric(d) => d.scatter(in_ray, hit_record),
        }
    }
}
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Lambertian {
    albedo: Color,
}

impl Scatterable for Lambertian {
    fn scatter(self, in_ray: Ray, hit_record: HitRecord) -> Option<ScatterRecord> {
        let direction = hit_record.normal() + Vec3::random_vec_on_unit_sphere();

        let direction = if direction.near_zero() {
            hit_record.normal()
        } else {
            direction
        };

        Some(ScatterRecord {
            ray: Ray::new(hit_record.p(), direction),
            attenuation: self.albedo,
        })
    }
}

impl Lambertian {
    pub fn new(color: Color) -> Lambertian {
        Lambertian { albedo: color }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Metal {
    albedo: Color,
    fuzz: f64,
}

impl Scatterable for Metal {
    fn scatter(self, in_ray: Ray, hit_record: HitRecord) -> Option<ScatterRecord> {
        let direction = in_ray.direction().reflect(hit_record.normal()) + Vec3::random_vec_on_unit_sphere()*self.fuzz*hit_record.normal().dot(-in_ray.direction());

        if hit_record.normal().dot(direction) > 0. {
            Some(ScatterRecord {
                ray: Ray::new(hit_record.p(), direction),
                attenuation: self.albedo,
            })
        } else {
            None
        }
    }
}

impl Metal {
    pub fn new(color: Color, fuzz:f64) -> Metal {
        Metal { albedo: color, fuzz: fuzz }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Dielectric {
    refraction_index: f64,
}

impl Dielectric {
    pub fn new(refraction_index: f64) -> Dielectric{
        Dielectric { refraction_index }
    }

    fn reflectance(cosine: f64, refraction_index: f64) -> f64 {
        // Shlick's approximation for reflectance
        let r0 = (1.- refraction_index) / (1.+refraction_index);
        let r0 = r0*r0;
        r0 + (1.-r0)*(1.-cosine).powi(5)
    }
}

impl Scatterable for Dielectric{
    fn scatter(self, in_ray: Ray, hit_record: HitRecord) -> Option<ScatterRecord> {
        let attenuation = Color::white();
        let ri = if hit_record.front_face() {
            1./self.refraction_index
        } else {
            self.refraction_index
        };

        let cos_theta = (-in_ray.direction()).dot(hit_record.normal()).min(1.);
        let sin_theta = (1. - cos_theta*cos_theta).sqrt();

        let cannot_refract: bool = ri * sin_theta > 1.;

        let direction = if cannot_refract || Dielectric::reflectance(cos_theta, ri) > utils::random_double(){
            // cannot refract, must reflect OR, sometimes a ray that would be refracted gets reflected instead, at reflectance rate
            in_ray.direction().reflect(hit_record.normal())
        } else {
            // can refract
            in_ray.direction().refract(hit_record.normal(), ri)
        };

        Some(ScatterRecord {
            ray: Ray::new(hit_record.p(), direction),
            attenuation,
        })
    }
}