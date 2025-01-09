use crate::Colour;

use crate::Material;
use crate::Ray;
use crate::Vector3;

pub struct Metal {
    albedo: Colour,
}

impl Metal {
    pub fn new(albedo: Colour) -> Self {
        Self { albedo }
    }
}

impl Material for Metal {
    fn scatter(
        &self,
        r_in: &crate::Ray,
        rec: &crate::HitRecord,
        attenuation: &mut Colour,
        scattered: &mut crate::Ray,
    ) -> bool {
        let reflected = Vector3::reflect(r_in.direction(), &rec.normal);
        *scattered = Ray::new(rec.p, reflected);
        *attenuation = self.albedo;
        true
    }
}
