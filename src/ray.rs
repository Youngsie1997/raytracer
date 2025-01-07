use super::{Point3, Vector3};

#[derive(Default, Debug, Clone, Copy)]
pub struct Ray {
    origin: Point3,
    direction: Vector3,
}

impl Ray {
    pub fn new(origin: Point3, direction: Vector3) -> Self {
        Self { origin, direction }
    }

    pub fn origin(&self) -> &Point3 {
        &self.origin
    }

    pub fn direction(&self) -> &Vector3 {
        &self.direction
    }

    pub fn set_origin(&mut self, origin: Point3) {
        self.origin = origin;
    }

    pub fn set_direction(&mut self, direction: Vector3) {
        self.direction = direction;
    }

    // Methods

    pub fn at(self, t: f64) -> Point3 {
        *self.origin() + (t * *self.direction())
    }
}
