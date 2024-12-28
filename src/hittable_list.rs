use std::sync::Arc;

use crate::hittable::{HitRecord, Hittable};
use crate::ray::Ray;
pub struct HittableList {
    list: Vec<Arc<dyn Hittable>>,
}

impl Hittable for HittableList {
    fn hit(&self, ray: Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        self.list
            .iter()
            .filter_map(|x| x.hit(ray, t_min, t_max))
            .min_by(|a, b| a.t().partial_cmp(&b.t()).unwrap())
    }
}

impl HittableList {
    pub fn add(&mut self, hittable: Arc<dyn Hittable>) {
        self.list.push(hittable)
    }

    pub fn new() -> HittableList {
        HittableList { list: Vec::new() }
    }
}
