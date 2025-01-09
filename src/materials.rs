pub mod lambertian;
pub mod metal;
use crate::{Colour, HitRecord, Ray};

pub trait Material {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Colour,
        scattered: &mut Ray,
    ) -> bool;
}
