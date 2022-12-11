use crate::vec3::{Point, Vec3};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Ray {
    orig: Point,
    dir: Vec3,
}

impl Ray {
    pub fn new(orig: Point, dir: Vec3) -> Self {
        Ray {
            orig: orig,
            dir: dir,
        }
    }
    pub fn origin(self) -> Point {
        self.orig
    }
    pub fn direction(self) -> Vec3 {
        self.dir
    }
    pub fn at(self, t: f64) -> Point {
        self.orig + t * self.dir
    }
}
