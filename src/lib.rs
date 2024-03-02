pub mod camera;
pub mod lights;
pub mod objects;
pub mod octtree;
pub mod scene;
pub mod utils;

use {lights::Point, objects::Cylinder, objects::Plane, objects::Sphere, scene::Scene};

use {nalgebra::Vector3, rand::Rng};

pub fn init_scene() -> Scene {
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
        Vector3::new(0.0, 5.0, 0.0),             // Position
        Vector3::new(1.0, 0.0, 1.0).normalize(), // Direction vector *NEEDS TO BE NORMALIZED*
        0.5,                                     // Width of the cylinder
        Vector3::new(0.5, 0.5, 0.8),             // Color scaled from 0 to 1 in RGB
        0.5,                                     // Reflection index
        0.0,                                     // Refraction index
    );
    scene.objects.push(Box::new(cylinder));

    for _ in 0..20 {
        let random_sphere: Sphere = Sphere::new(
            Vector3::new(
                rng.gen_range(-5.0..5.0),
                rng.gen_range(0.0..10.0),
                rng.gen_range(-5.0..5.0),
            ),
            rng.gen_range(0.01..1.0),
            Vector3::new(
                rng.gen_range(0.0..1.0),
                rng.gen_range(0.0..1.0),
                rng.gen_range(0.0..1.0),
            ),
            rng.gen_range(0.0..1.0),
            0.0,
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
