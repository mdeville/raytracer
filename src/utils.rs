use crate::lights::Light;
use crate::objects::Object;
use nalgebra::max;
use nalgebra::base::{Vector2, Vector3};
use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread;
use rayon::prelude::*;
use std::process;

fn vec2color(input: Vector3<f64>) -> u32 {
    let mut color: u32 = ((input.x * 255.0) as u32) << 16;
    color += ((input.y * 255.0) as u32) << 8;
    color + (input.z * 255.0) as u32
}

fn raytrace(scene: &Scene, origin: Vector3<f64>, ray: Vector3<f64>, depth: u32) -> Vector3<f64> {
    let mut min_distance = f64::INFINITY;
    let mut closest: Option<&Box<dyn Object + Sync + Send>> = None;
    for object in scene.objects.iter() {
        if let Some(distance) = object.intersect(origin, ray) {
            if distance < min_distance {
                min_distance = distance;
                closest = Some(object);
            }
        }
    }
    if let Some(closest) = closest {
        let hit_pos = origin + min_distance * ray;
        let normal = closest.normal(hit_pos);
        let reflection = closest.reflection();
        let refraction = closest.refraction();
        let mut color = Vector3::new(0.0, 0.0, 0.0);
        if reflection > 0.0 && depth > 0 {
            let reflection_ray = ray - 2.0 * (normal.dot(&ray)) * normal;
            let tmp = hit_pos + reflection_ray * 1e-4;
            color += raytrace(scene, tmp, reflection_ray, depth - 1) * reflection;
        }
        for light in scene.lights.iter() {
            let shadow_ray = light.shadow_ray(hit_pos);
            let light_distance = light.distance(hit_pos);
            let tmp = hit_pos + shadow_ray * 1e-4;
            let mut in_shadow = false;
            for object in scene.objects.iter() {
                if let Some(distance) = object.intersect(tmp, shadow_ray) {
                    in_shadow = distance < light_distance;
                    break;
                }
            }
            if !in_shadow {
                color += (normal.dot(&shadow_ray) * light.brightness() * light.color()).component_mul(&closest.color());
            }
        }
        return color.inf(&Vector3::new(1.0, 1.0, 1.0));
    }
    Vector3::new(0.0, 0.0, 0.0)
}

pub struct Scene {
    pub objects: Vec<Box<dyn Object + Sync + Send>>,
    pub lights: Vec<Box<dyn Light + Sync + Send>>,
}

impl Scene {
    pub fn new() -> Self {
        Scene {
            objects: Vec::new(),
            lights: Vec::new(),
        }
    }
}

pub struct Camera {
    pub eye: Vector3<f64>,
    pub target: Vector3<f64>,
    pub vertical: Vector3<f64>,
    pub viewport: Vector2<f64>,
    pub fov: f64,
    sender: Sender<(usize, Vec<u32>)>,
    pub receiver: Receiver<(usize, Vec<u32>)>,
}

impl Camera {
    pub fn new(width: usize, height: usize, fov: f64) -> Self {
        let (sender, receiver) = channel();

        Camera {
            eye: Vector3::new(0.0, -3.0, 2.0),
            target: Vector3::new(0.0, 1.0, 0.0).normalize(),
            vertical: Vector3::new(0.0, 0.0, 1.0),
            viewport: Vector2::new(width as f64, height as f64),
            fov,
            sender,
            receiver,
        }
    }

    pub fn start_rendering(&self, scene: Scene, depth: u32) {
        // Precalculations
        let height = self.viewport.y as usize;
        let width = self.viewport.x as usize;
        let b = self.target.cross(&self.vertical).normalize();
        let gx = (self.fov / 2.0).tan();
        let gy = gx * (self.viewport.y / self.viewport.x);
        let qx = (2.0 * gx) / self.viewport.x * b;
        let qy = (2.0 * gy) / self.viewport.y * self.vertical;
        let pm = self.target - gx * b + gy * self.vertical;

        // Helps the compiler to understand that this is safe
        let sender = self.sender.clone();
        let eye = self.eye;
        thread::spawn(move || {
            loop {
                let ls = sender.clone();
                (0..height).into_par_iter().for_each_with(ls, |s, i| {
                    let mut buffer: Vec<u32> = Vec::new();
                    for j in 0..width {
                        let ray = (pm + qx * j as f64 - qy * i as f64).normalize();
                        let color = raytrace(&scene, eye, ray, depth);
                        buffer.push(vec2color(color));
                    }
                    // Send rendered buffer to main
                    if s.send((i * width, buffer)).is_err() {
                        process::exit(0);
                    }
                })
            }
        });
    }
}
