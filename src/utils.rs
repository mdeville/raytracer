use crate::scene::Scene;

use nalgebra::Vector3;

#[derive(Copy, Clone)]
pub struct Ray {
    pub origin: Vector3<f64>,
    pub direction: Vector3<f64>,
}

#[derive(Copy, Clone)]
pub struct Hit {
    pub position: Vector3<f64>,
    pub distance: f64,
    pub normal: Vector3<f64>,
}

pub fn quadratic_solve(a: f64, b: f64, c: f64) -> Option<f64> {
    let discr = b * b - 4.0 * a * c;
    if discr < 0.0 {
        return None;
    }
    let dist = (-b - discr.sqrt()) / (2.0 * a);
    if dist < 0.0 {
        let dist = (-b + discr.sqrt()) / (2.0 * a);
        if dist < 0.0 {
            return None;
        }
    }
    Some(dist)
}

fn reflect(ray: Vector3<f64>, normal: Vector3<f64>) -> Vector3<f64> {
    2.0 * (normal.dot(&ray)) * normal - ray
}

pub fn raytrace(scene: &Scene, ray: Ray, depth: u32) -> Vector3<f64> {
    //Find closest object in scene
    let (object, hit) = match scene.find_intersecting_object(ray) {
        Some((object, hit)) => (object, hit),
        _ => return Vector3::new(0.0, 0.0, 0.0),
    };

    //TODO: ambient light
    let mut out_color = Vector3::new(0.0, 0.0, 0.0);

    // Compute reflection
    let reflection = object.reflection();
    if reflection < 1.0 {
        for light in scene.lights.iter() {
            let shadow_dir = light.shadow_ray(hit.position);
            let light_distance = light.distance(hit.position);
            let shadow_ray = Ray {
                origin: hit.position + shadow_dir * 1e-4,
                direction: shadow_dir,
            };
            let mut in_shadow = false;
            for object in scene.objects.iter() {
                if let Some(shadow_hit) = object.intersect(shadow_ray) {
                    if shadow_hit.distance < light_distance {
                        in_shadow = true;
                        break;
                    }
                }
            }
            if !in_shadow {
                let reflection_ray = reflect(shadow_ray.direction, hit.normal);
                out_color +=
                    (hit.normal.dot(&shadow_ray.direction) * light.brightness() * light.color())
                        .component_mul(&object.color())
                        + reflection_ray.dot(&-ray.direction).max(0.0).powf(10.0)
                            * light.color()
                            * light.brightness();
            }
        }
    }

    if reflection > 0.0 && depth > 0 {
        let reflection_dir = reflect(-ray.direction, hit.normal);
        let ray = Ray {
            origin: hit.position + reflection_dir * 1e-4,
            direction: reflection_dir,
        };
        out_color += raytrace(scene, ray, depth - 1) * reflection;
    }

    // Return final color
    out_color.inf(&Vector3::new(1.0, 1.0, 1.0))
}
