mod lights;
mod objects;
mod utils;

use lights::{Directionnal, Point};
use minifb::{Key, Window, WindowOptions};
use nalgebra::base::*;
use nalgebra::geometry::Rotation3;
use objects::{Circle, Plane};
use std::f64::consts::PI;
use utils::{Camera, Scene};

const WIDTH: usize = 2560;
const HEIGHT: usize = 1600;

fn init_scene() -> Scene {
    let mut scene = Scene::new();

    let red_circle: Circle = Circle::new(
        Vector3::new(0.0, 3.0, -2.0),
        1.0,
        Vector3::new(1.0, 0.0, 0.0),
    );
    let green_circle: Circle = Circle::new(
        Vector3::new(0.0, 3.5, 1.5),
        0.3,
        Vector3::new(0.0, 1.0, 0.0),
    );
    let blue_circle: Circle = Circle::new(
        Vector3::new(1.0, 4.0, 0.0),
        1.0,
        Vector3::new(0.0, 0.0, 1.0),
    );
    scene.objects.push(Box::new(red_circle));
    scene.objects.push(Box::new(green_circle));
    scene.objects.push(Box::new(blue_circle));

    let bottom: Plane = Plane::new(
        Vector3::new(0.0, 0.0, -5.0),
        Vector3::new(0.0, 0.0, 1.0),
        Vector3::new(1.0, 1.0, 1.0),
    );
    let right: Plane = Plane::new(
        Vector3::new(5.0, 0.0, 0.0),
        Vector3::new(-1.0, 0.0, 0.0),
        Vector3::new(1.0, 1.0, 1.0),
    );
    let back: Plane = Plane::new(
        Vector3::new(0.0, 10.0, 0.0),
        Vector3::new(0.0, -1.0, 0.0),
        Vector3::new(1.0, 1.0, 1.0),
    );
    scene.objects.push(Box::new(bottom));
    scene.objects.push(Box::new(right));
    scene.objects.push(Box::new(back));

    let light1: Point = Point::new(
        Vector3::new(-4.0, 2.0, 4.9),
        Vector3::new(1.0, 1.0, 1.0),
        1.0,
    );
    //let light2: Directionnal = Directionnal::new(Vector3::new(0.0, 0.0, -1.0), Vector3::new(1.0, 1.0, 1.0), 1.0);
    scene.lights.push(Box::new(light1));
    //scene.lights.push(Box::new(light2));
    scene
}

fn main() {
    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];

    let mut window = Window::new(
        "Raytracer - ESC to exit",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    // Limit to max ~60 fps update rate
    window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

    // Init camera and scene
    let mut camera = Camera::new(WIDTH, HEIGHT, PI / 2.0);
    let rot = Rotation3::from_euler_angles(-0.4, 0.0, 0.0);
    camera.target = (rot * camera.target).normalize();
    camera.vertical = (rot * camera.vertical).normalize();
    let scene = init_scene();

    camera.start_rendering(scene, 10);
    // Rendering loop
    while window.is_open() && !window.is_key_down(Key::Escape) {
        for (idx, color) in camera.receiver.try_iter() {
            buffer[idx] = color;
        }
        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
    }
}
