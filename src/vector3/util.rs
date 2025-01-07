use std::fmt::Display;

use super::Vector3;

impl Display for Vector3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "x: {} y: {} z: {}", self.x(), self.y(), self.z())
    }
}

// Dot Product

pub fn dot(u: &Vector3, v: &Vector3) -> f64 {
    u.x() * v.x() + u.y() * v.y() + u.z() * v.z()
}
// Cross Product

pub fn cross(u: &Vector3, v: &Vector3) -> Vector3 {
    Vector3::new(
        u.y() * v.z() - u.z() * v.y(),
        u.z() * v.x() - u.x() * v.z(),
        u.x() * v.y() - u.y() * v.x(),
    )
}

pub fn unit_vector(v: &Vector3) -> Vector3 {
    *v / v.length()
}
