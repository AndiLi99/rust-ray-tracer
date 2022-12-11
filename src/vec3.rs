use std::ops::{Add, Div, Mul, Neg, Sub};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Vec3(pub f64, pub f64, pub f64);

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
