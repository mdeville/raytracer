mod lights;
mod objects;
mod camera;
mod scene;
mod utils;

use lights::{Directionnal, Point};
use minifb::{Key, Window, WindowOptions};
use nalgebra::base::*;
use nalgebra::geometry::Rotation3;
use objects::{Sphere, Plane, Cone, Cylinder};
use std::f64::consts::PI;
use camera::Camera;
use scene::Scene;
use rand::Rng;

const WIDTH: usize = 1920;
const HEIGHT: usize = 1080;

fn init_scene() -> Scene {
    let mut scene = Scene::new();
    let mut rng = rand::thread_rng();

/*     let cone = Cone::new(
        Vector3::new(0.0, 10.0, 0.0),
        Vector3::new(0.0, 0.0, 1.0).normalize(),
        PI / 8.0,
        Vector3::new(0.5, 0.5, 0.8),
        0.0,
        0.0);
    scene.objects.push(Box::new(cone)); */

    let cylinder = Cylinder::new(
        Vector3::new(0.0, 5.0, 0.0),                // Position
        Vector3::new(1.0, 0.0, 1.0).normalize(),    // Direction vector *NEEDS TO BE NORMALIZED*
        0.5,                                        // Width of the cylinder
        Vector3::new(0.5, 0.5, 0.8),                // Color scaled from 0 to 1 in RGB
        0.5,                                        // Reflection index
        0.0                                         // Refraction index
    );
    scene.objects.push(Box::new(cylinder));

    for _ in 0..20 {
        let random_sphere: Sphere = Sphere::new(
            Vector3::new(rng.gen_range(-5.0..5.0), rng.gen_range(0.0..10.0), rng.gen_range(-5.0..5.0)),
            rng.gen_range(0.01..1.0),
            Vector3::new(rng.gen_range(0.0..1.0), rng.gen_range(0.0..1.0), rng.gen_range(0.0..1.0)),
            rng.gen_range(0.0..1.0),
            0.0
        );
        scene.objects.push(Box::new(random_sphere));
    }
   
    let bottom: Plane = Plane::new(
        Vector3::new(0.0, 0.0, -5.0),
        Vector3::new(0.0, 0.0, 1.0),
        Vector3::new(1.0, 0.5, 0.2),
        0.0,
        0.0,
    );
    scene.objects.push(Box::new(bottom));
    
    let back: Plane = Plane::new(
        Vector3::new(0.0, 10.0, 0.0),
        Vector3::new(0.0, -1.0, 0.0),
        Vector3::new(0.0, 0.5, 0.0),
        0.5,
        0.0,
    );
    scene.objects.push(Box::new(back));

    let light1: Point = Point::new(
        Vector3::new(-5.0, 0.0, 4.9),
        Vector3::new(1.0, 1.0, 1.0),
        1.0,
    );
/*     let light2: Point = Point::new(
        Vector3::new(-4.0, 3.0, 4.9),
        Vector3::new(1.0, 1.0, 1.0),
        0.5,
    ); */
    //let light2: Directionnal = Directionnal::new(Vector3::new(0.0, 0.0, -1.0), Vector3::new(1.0, 1.0, 1.0), 0.5);
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
        WindowOptions {borderless: true, ..Default::default()},
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

    camera.start_rendering(scene, 3);
    // Rendering loop
    while window.is_open() && !window.is_key_down(Key::Escape) {
        for (idx, line) in camera.receiver.try_iter() {
            buffer[idx..idx+WIDTH].copy_from_slice(&line);
        }
        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
    }
}
