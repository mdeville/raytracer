use nalgebra::base::Vector3;

pub trait Light {
    fn color(&self) -> Vector3<f64>;
    fn brightness(&self) -> f64;
    fn shadow_ray(&self, origin: Vector3<f64>) -> Vector3<f64>;
    fn distance(&self, origin: Vector3<f64>) -> f64;
}

pub struct Directionnal {
    direction: Vector3<f64>,
    color: Vector3<f64>,
    brightness: f64,
}

impl Directionnal {
    pub fn new(direction: Vector3<f64>, color: Vector3<f64>, brightness: f64) -> Self {
        Directionnal {
            direction,
            color,
            brightness,
        }
    }
}

impl Light for Directionnal {
    fn color(&self) -> Vector3<f64> {
        self.color
    }

    fn brightness(&self) -> f64 {
        self.brightness
    }

    fn shadow_ray(&self, _: Vector3<f64>) -> Vector3<f64> {
        -self.direction
    }

    fn distance(&self, _: Vector3<f64>) -> f64 {
        f64::INFINITY
    }
}

pub struct Point {
    position: Vector3<f64>,
    color: Vector3<f64>,
    brightness: f64,
}

impl Point {
    pub fn new(position: Vector3<f64>, color: Vector3<f64>, brightness: f64) -> Self {
        Point {
            position,
            color,
            brightness,
        }
    }
}

impl Light for Point {
    fn color(&self) -> Vector3<f64> {
        self.color
    }

    fn brightness(&self) -> f64 {
        self.brightness
    }

    fn shadow_ray(&self, origin: Vector3<f64>) -> Vector3<f64> {
        (self.position - origin).normalize()
    }

    fn distance(&self, origin: Vector3<f64>) -> f64 {
        (self.position - origin).norm()
    }
}
