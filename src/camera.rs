use crate::scene::Scene;
use crate::utils::{Ray, raytrace};
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
                        let ray = Ray {
                            origin: eye,
                            direction: (pm + qx * j as f64 - qy * i as f64).normalize(),
                        };
                        let color = raytrace(&scene, ray, depth);
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
