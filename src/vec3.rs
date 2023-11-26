use rand::distributions::{Distribution, Standard};
use rand::Rng;
use std::ops::{Add, Div, Mul, Neg, Sub};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Vec3(pub f64, pub f64, pub f64);

impl Distribution<Vec3> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Vec3 {
        let (x, y, z) = rng.gen();
        Vec3(x, y, z)
    }
}

impl Add<Vec3> for f64 {
    type Output = Vec3;
    fn add(self, rhs: Vec3) -> Self::Output {
        Vec3(rhs.0 + self, rhs.1 + self, rhs.2 + self)
    }
}

impl Add for Vec3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self(self.x() + rhs.x(), self.y() + rhs.y(), self.z() + rhs.z())
    }
}
impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Self(self.x() - rhs.x(), self.y() - rhs.y(), self.z() - rhs.z())
    }
}

impl Mul for Vec3 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        Self(self.x() * rhs.x(), self.y() * rhs.y(), self.z() * rhs.z())
    }
}
impl Mul<f64> for Vec3 {
    type Output = Self;

    fn mul(self, t: f64) -> Self {
        Self(self.x() * t, self.y() * t, self.z() * t)
    }
}
impl Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Vec3 {
        rhs * self
    }
}
impl Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: f64) -> Self {
        (1. / rhs) * self
    }
}

impl Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Self::Output {
        self * -1.
    }
}
impl Vec3 {
    pub fn origin() -> Vec3 {
        Vec3(0.0, 0.0, 0.0)
    }
    pub fn dot(self, rhs: Self) -> f64 {
        self.x() * rhs.x() + self.y() * rhs.y() + self.z() * rhs.z()
    }
    pub fn cross(lhs: Self, rhs: Self) -> Self {
        Self(
            lhs.y() * rhs.z() - lhs.z() * rhs.y(),
            lhs.z() * rhs.x() - lhs.x() * rhs.z(),
            lhs.x() * rhs.y() - lhs.y() * rhs.x(),
        )
    }
    pub fn length_squared(self) -> f64 {
        self.x() * self.x() + self.y() * self.y() + self.z() * self.z()
    }
    pub fn length(self) -> f64 {
        self.length_squared().sqrt()
    }
    pub fn unit_vector(self) -> Self {
        self / self.length()
    }
    pub fn x(self) -> f64 {
        self.0
    }
    pub fn y(self) -> f64 {
        self.1
    }
    pub fn z(self) -> f64 {
        self.2
    }

    fn map_to_range(v: Vec3, min: f64, max: f64) -> Vec3 {
        min + (max - min) * v
    }
    pub fn random_vec_in_unit_sphere() -> Vec3 {
        let mut rng = rand::thread_rng();
        for _ in 0..10000 {
            let p: Vec3 = Vec3::map_to_range(rng.gen(), -1., 1.);
            if p.length_squared() < 1. {
                return p;
            }
        }
        panic!("Could not find a random vector in the unit sphere!")
    }
    pub fn random_vec_on_unit_sphere() -> Vec3 {
        Vec3::random_vec_in_unit_sphere().unit_vector()
    }
    pub fn random_vec_on_hemisphere(normal: Vec3) -> Vec3 {
        let on_unit_sphere = Vec3::random_vec_on_unit_sphere();
        if on_unit_sphere.dot(normal) > 0. {
            on_unit_sphere
        } else {
            -on_unit_sphere
        }
    }
    pub fn near_zero(&self) -> bool {
        let s = 1e-8;
        self.0.abs() < s && self.1.abs() < s && self.2.abs() < s
    }
    pub fn reflect(&self, normal: Vec3) -> Vec3{
        *self - 2.*self.dot(normal)*normal
    }
}

pub type Point = Vec3;
pub type Color = Vec3;

impl Point {
    pub fn point(x: f64, y: f64, z: f64) -> Point {
        Vec3(x, y, z)
    }
}
impl Color {
    pub fn color(x: f64, y: f64, z: f64) -> Color {
        Vec3(x, y, z)
    }
    pub fn black() -> Color {
        Vec3(0., 0., 0.)
    }
}
