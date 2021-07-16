use crate::utils::{Ray, Hit, quadratic_solve};

use nalgebra::base::Vector3;

pub trait Object {
    fn intersect(&self, ray: Ray) -> Option<Hit>;
    fn color(&self) -> Vector3<f64>;
    fn reflection(&self) -> f64;
    fn refraction(&self) -> f64;
    fn normal(&self, position: Vector3<f64>) -> Vector3<f64>;
}

pub struct Sphere {
    center: Vector3<f64>,
    radius: f64,
    color: Vector3<f64>,
    reflection: f64,
    refraction: f64
}

impl Sphere {
    pub fn new(center: Vector3<f64>, radius: f64, color: Vector3<f64>, reflection: f64, refraction: f64) -> Self {
        Sphere {
            center,
            radius,
            color,
            reflection,
            refraction
        }
    }
}

impl Object for Sphere {
    fn intersect(&self, ray: Ray) -> Option<Hit> {
        let tmp = ray.origin - self.center;
        let b = 2.0 * (ray.direction.dot(&tmp));
        let c = tmp.dot(&tmp) - self.radius * self.radius;
        if let Some(dist) = quadratic_solve(1.0, b, c) {
            let hit_pos = ray.origin + ray.direction * dist;
            return Some( Hit {
                position: hit_pos,
                distance: dist,
                normal: self.normal(hit_pos)
            })
        }
        None
    }

    fn color(&self) -> Vector3<f64> {
        self.color
    }

    fn reflection(&self) -> f64 {
        self.reflection
    }

    fn refraction(&self) -> f64 {
        self.refraction
    }

    fn normal(&self, position: Vector3<f64>) -> Vector3<f64> {
        (position - self.center).normalize()
    }
}

pub struct Plane {
    position: Vector3<f64>,
    normal: Vector3<f64>,
    color: Vector3<f64>,
    reflection: f64,
    refraction: f64
}

impl Plane {
    pub fn new(position: Vector3<f64>, normal: Vector3<f64>, color: Vector3<f64>, reflection: f64, refraction: f64) -> Self {
        Plane {
            position,
            normal,
            color,
            reflection,
            refraction
        }
    }
}

impl Object for Plane {
    fn intersect(&self, ray: Ray) -> Option<Hit> {
        let tmp = ray.direction.dot(&self.normal);
        if tmp != 0.0 {
            let d = (self.position - ray.origin).dot(&self.normal) / tmp;
            if d > 0.0 {
                let hit_pos = ray.origin + ray.direction * d;
                return Some( Hit {
                    position: hit_pos,
                    distance: d,
                    normal: self.normal(hit_pos)
                });
            }
        }
        None
    }

    fn color(&self) -> Vector3<f64> {
        self.color
    }

    fn reflection(&self) -> f64 {
        self.reflection
    }

    fn refraction(&self) -> f64 {
        self.refraction
    }

    fn normal(&self, _: Vector3<f64>) -> Vector3<f64> {
        self.normal
    }
}

pub struct Cone {
    position: Vector3<f64>,
    direction: Vector3<f64>,
    angle: f64,
    color: Vector3<f64>,
    reflection: f64,
    refraction: f64
}

impl Cone {
    pub fn new(position: Vector3<f64>, direction: Vector3<f64>, angle: f64, color: Vector3<f64>, reflection: f64, refraction: f64) -> Self {
        Cone {
            position,
            direction,
            angle,
            color,
            reflection,
            refraction
        }
    }
}

impl Object for Cone {
    fn intersect(&self, ray: Ray) -> Option<Hit> {
        let dv = ray.direction.dot(&self.direction);
        let co = ray.origin - self.direction;
        let cov = co.dot(&self.direction);
        let cos2 = self.angle.powf(2.0);
        let a = dv.powf(2.0) - cos2;
        let b = 2.0 * (dv * cov - ray.direction.dot(&co) * cos2);
        let c = cov.powf(2.0) - co.dot(&co) * cos2;
        if let Some(dist) = quadratic_solve(a, b, c) {
            let hit_pos = ray.origin + dist * ray.direction;
            if (hit_pos - self.position).dot(&self.direction) > 0.0 {
                return Some( Hit {
                    position: hit_pos,
                    distance: dist,
                    normal: self.normal(hit_pos)
                });
            }
        }
        None
    }

    fn color(&self) -> Vector3<f64> {
        self.color
    }

    fn reflection(&self) -> f64 {
        self.reflection
    }

    fn refraction(&self) -> f64 {
        self.refraction
    }

    fn normal(&self, hit_position: Vector3<f64>) -> Vector3<f64> {
        let adj = (self.position - hit_position).norm();
        let hyp = adj * self.angle.cos();
        let center = hyp * self.direction;
        (hit_position - center).normalize()
    }
}


pub struct Cylinder {
    position: Vector3<f64>,
    direction: Vector3<f64>,
    radius: f64,
    color: Vector3<f64>,
    reflection: f64,
    refraction: f64
}

impl Cylinder {
    pub fn new(position: Vector3<f64>, direction: Vector3<f64>, radius: f64, color: Vector3<f64>, reflection: f64, refraction: f64) -> Self {
        Cylinder {
            position,
            direction,
            radius,
            color,
            reflection,
            refraction
        }
    }
}

impl Object for Cylinder {
    fn intersect(&self, ray: Ray) -> Option<Hit> {
        let an = self.position.dot(&self.direction);
        let on = ray.origin.dot(&self.direction);
        let dn = ray.direction.dot(&self.direction);
        let a = 1.0 - dn.powf(2.0);
        let b = 2.0 * (ray.origin.dot(&ray.direction) - self.position.dot(&ray.direction) + an * dn - on * dn);
        let c = self.position.dot(&self.position) - 2.0 * self.position.dot(&ray.origin) + ray.origin.dot(&ray.origin) + an.powf(2.0) + 2.0 * an * on - on.powf(2.0) - self.radius.powf(2.0);
        if let Some(dist) = quadratic_solve(a, b, c) {
            let hit_pos = ray.origin + ray.direction * dist;
            return Some( Hit {
                position: hit_pos,
                distance: dist,
                normal: self.normal(hit_pos)
            })
        }
        None
    }

    fn color(&self) -> Vector3<f64> {
        self.color
    }

    fn reflection(&self) -> f64 {
        self.reflection
    }

    fn refraction(&self) -> f64 {
        self.refraction
    }

    fn normal(&self, hit_position: Vector3<f64>) -> Vector3<f64> {
        let tmp = hit_position - self.position;
        (tmp - tmp.dot(&self.direction) * self.direction).normalize()
    }
}