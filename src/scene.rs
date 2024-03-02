use crate::lights::Light;
use crate::objects::Object;
use crate::utils::{Hit, Ray};

pub struct Scene {
    pub objects: Vec<Box<dyn Object>>,
    pub lights: Vec<Box<dyn Light>>,
}

impl Scene {
    pub fn new() -> Self {
        Scene {
            objects: Vec::new(),
            lights: Vec::new(),
        }
    }

    pub fn find_intersecting_object(&self, ray: Ray) -> Option<(&Box<dyn Object>, Hit)> {
        let mut min_dist = f64::INFINITY;
        let mut min_hit: Option<Hit> = None;
        let mut closest: Option<&Box<dyn Object>> = None;
        for object in self.objects.iter() {
            if let Some(hit) = object.intersect(ray) {
                if hit.distance < min_dist {
                    min_hit = Some(hit);
                    closest = Some(object);
                    min_dist = hit.distance;
                }
            }
        }
        match (closest, min_hit) {
            (Some(x), Some(y)) => Some((x, y)),
            _ => None,
        }
    }
}
