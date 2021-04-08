use nalgebra::base::Vector3;

pub trait Object {
    fn intersect(&self, origin: Vector3<f64>, ray: Vector3<f64>) -> Option<f64>;
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
    fn intersect(&self, origin: Vector3<f64>, ray: Vector3<f64>) -> Option<f64> {
        let tmp = origin - self.center;
        let b = 2.0 * (ray.dot(&tmp));
        let c = tmp.dot(&tmp) - self.radius * self.radius;
        let discr = b * b - 4.0 * c;
        if discr < 0.0 {
            return None;
        }
        let dist = (-b - discr.sqrt()) / 2.0;
        if dist < 0.0 {
            let dist = (-b + discr.sqrt()) / 2.0;
            if dist < 0.0 {
                return None;
            }
        }
        Some(dist)
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
    fn intersect(&self, origin: Vector3<f64>, ray: Vector3<f64>) -> Option<f64> {
        let tmp = ray.dot(&self.normal);
        if tmp != 0.0 {
            let d = (self.position - origin).dot(&self.normal) / tmp;
            if d > 0.0 {
                return Some(d);
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
