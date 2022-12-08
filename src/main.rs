use std::ops::{Add, Div, Mul, Neg, Sub};

fn main() {
    // Image

    let image_width: i64 = 256;
    let image_height: i64 = 256;

    // Render

    println!("P3");
    println!("{} {}", image_width, image_height);
    println!("255");
    for y in (0..image_height).rev() {
        eprint!("\rScanlines remaining: {}", y);
        for x in 0..image_width {
            let color: Color = Vec3(
                (x as f64) / ((image_height - 1) as f64),
                (y as f64) / ((image_width - 1) as f64),
                0.25,
            );

            write_color(color);
        }
    }
    eprintln!("\nDone.");
}

#[derive(Debug, Copy, Clone, PartialEq)]
struct Vec3(f64, f64, f64);

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
    fn dot(lhs: Self, rhs: Self) -> Self {
        Self(lhs.x() * rhs.x(), lhs.y() * rhs.y(), lhs.z() * rhs.z())
    }
    fn cross(lhs: Self, rhs: Self) -> Self {
        Self(
            lhs.y() * rhs.z() - lhs.z() * rhs.y(),
            lhs.z() * rhs.x() - lhs.x() * rhs.z(),
            lhs.x() * rhs.y() - lhs.y() * rhs.x(),
        )
    }
    fn length_squared(self) -> f64 {
        self.x() * self.x() + self.y() * self.y() + self.z() * self.z()
    }
    fn length(self) -> f64 {
        self.length_squared().sqrt()
    }
    fn unit_vector(self) -> Self {
        self / self.length()
    }
    fn x(self) -> f64 {
        self.0
    }
    fn y(self) -> f64 {
        self.1
    }
    fn z(self) -> f64 {
        self.2
    }
}

fn write_color(color: Color) {
    fn convert_to_int(f: f64) -> u8 {
        (255.999 * f) as u8
    }
    // translate floating point to [0, 255] range of ints
    println!(
        "{} {} {}",
        convert_to_int(color.0),
        convert_to_int(color.1),
        convert_to_int(color.2)
    );
}
type Point3 = Vec3;
type Color = Vec3;

#[derive(Debug, Copy, Clone, PartialEq)]
struct Ray {
    orig: Point3,
    dir: Vec3,
}

impl Ray {
    fn origin(self) -> Point3 {
        self.orig
    }
    fn direction(self) -> Vec3 {
        self.dir
    }
    fn at(self, t: f64) -> Point3 {
        self.orig + t*self.dir
    }
}
