pub mod operators;
pub mod util;
#[derive(Debug, Clone, Copy)]
pub struct Vector3 {
    pub e: [f64; 3],
}

pub type Point3 = Vector3;
pub type Colour = Vector3;
impl Vector3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Vector3 { e: [x, y, z] }
    }

    // Getters
    pub fn x(&self) -> &f64 {
        &self.e[0]
    }

    pub fn y(&self) -> &f64 {
        &self.e[1]
    }

    pub fn z(&self) -> &f64 {
        &self.e[2]
    }

    // Setters
    pub fn set_x(&mut self, x: f64) {
        self.e[0] = x;
    }

    pub fn set_y(&mut self, y: f64) {
        self.e[1] = y;
    }

    pub fn set_z(&mut self, z: f64) {
        self.e[2] = z;
    }

    //methods
    //
    pub fn length_squared(&self) -> f64 {
        self.x().powi(2) + self.y().powi(2) + self.z().powi(2)
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }
}

impl Default for Vector3 {
    fn default() -> Self {
        Vector3 { e: [0.0, 0.0, 0.0] }
    }
}

impl Colour {
    pub fn r(&self) -> &f64 {
        &self.e[0]
    }

    pub fn g(&self) -> &f64 {
        &self.e[1]
    }

    pub fn b(&self) -> &f64 {
        &self.e[2]
    }

    pub fn set_r(&mut self, r: f64) {
        self.e[0] = r;
    }

    pub fn set_g(&mut self, g: f64) {
        self.e[1] = g;
    }

    pub fn set_b(&mut self, b: f64) {
        self.e[2] = b;
    }
}
