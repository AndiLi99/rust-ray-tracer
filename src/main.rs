use std::ops::{Add, Div, Mul, Neg, Sub};

fn hit_sphere(center: Point, radius: f64, ray: Ray) -> bool {
    let orig_to_center: Vec3 = ray.origin() - center;
    let a: f64 = ray.dir.dot(ray.dir);
    let b: f64 = 2. * ray.dir.dot(orig_to_center);
    let c: f64 = orig_to_center.dot(orig_to_center) - radius*radius;
    let determinant: f64 = b*b - 4.*a*c;
    determinant > 0.
}

fn lerp(t: f64, start: Vec3, end: Vec3) -> Vec3 {
    (1.0 - t) * start + t* end
}

fn ray_color(ray: Ray) -> Color {
    if hit_sphere(Vec3(0., 0., -1.), 0.5, ray) {
        return Vec3(1.0, 0., 0.);
    }
    let unit_direction: Vec3 = ray.direction().unit_vector();
    let t = 0.5*(unit_direction.y() + 1.0);
    let white: Color = Vec3(1.0, 1.0, 1.0);
    let blue: Color = Vec3(0.5, 0.7, 1.0);
    lerp(t, white, blue)
}
fn main() {
    // Image
    let aspect_ratio: f64 = 16.0/9.0; // width divided by height
    let image_width: i64 = 400;
    let image_height: i64 = (image_width as f64 / aspect_ratio) as i64;

    // Camera

    let viewport_height: f64 = 2.0;
    let viewport_width: f64 = aspect_ratio * viewport_height;
    let focal_length: f64 = 1.0;

    let origin: Point = Vec3(0.0, 0.0, 0.0);
    let horizontal = Vec3(viewport_width, 0.0, 0.0);
    let vertical = Vec3(0.0, viewport_height, 0.0);
    let lower_left_corner = origin - horizontal / 2.0 - vertical/2.0 - Vec3(0.0, 0.0, focal_length);



    // Render

    println!("P3");
    println!("{} {}", image_width, image_height);
    println!("255");
    for y in (0..image_height).rev() {
        eprint!("\rScanlines remaining: {}", y);
        for x in 0..image_width {
            let u: f64 = x as f64 / (image_width as f64 - 1.);
            let v: f64 = y as f64 / (image_height as f64 - 1.);
            let ray: Ray = Ray{orig: origin, dir: lower_left_corner + u*horizontal + v*vertical - origin};
            let color: Color = ray_color(ray);

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
    fn dot(self, rhs: Self) -> f64 {
        self.x() * rhs.x() +self.y() * rhs.y() + self.z() * rhs.z()
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
type Point = Vec3;
type Color = Vec3;

impl Point {
    fn point(x:f64, y:f64, z:f64) -> Point {
        Vec3(x, y, z)
    }
}
impl Color {
    fn color(x:f64, y:f64, z:f64) -> Color {
        Vec3(x, y, z)
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
struct Ray {
    orig: Point,
    dir: Vec3,
}

impl Ray {
    fn origin(self) -> Point {
        self.orig
    }
    fn direction(self) -> Vec3 {
        self.dir
    }
    fn at(self, t: f64) -> Point {
        self.orig + t*self.dir
    }
}
