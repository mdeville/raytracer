use nalgebra::base::Vector3;

pub trait Material {
    ambient: f64,
    diffuse: f64,
    specular: f64,
    exponent: f64   
}