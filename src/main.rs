use raytracer::{camera::Camera, init_scene};

use {
    minifb::{Key, Window, WindowOptions},
    nalgebra::geometry::Rotation3,
    std::f64::consts::PI,
};

const WIDTH: usize = 1920;
const HEIGHT: usize = 1080;

fn main() {
    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];

    let mut window = Window::new(
        "Raytracer - ESC to exit",
        WIDTH,
        HEIGHT,
        WindowOptions {
            borderless: true,
            ..Default::default()
        },
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
            buffer[idx..idx + WIDTH].copy_from_slice(&line);
        }
        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
    }
}
