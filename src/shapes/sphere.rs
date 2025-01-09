use std::sync::Arc;

use crate::{dot, HitRecord, Hittable, Interval, Material, Point3, Ray};

#[derive(Clone)]
pub struct Sphere {
    pub center: Point3,
    pub radius: f64,
    pub mat: Arc<dyn Material>,
}

impl Sphere {
    /// Creates a new Sphere at the specified location with the specified radius.
    /// # Examples
    /// ```
    /// let s = raytracer::Sphere::new(raytracer::Point3::new(1.0, 2.0, 3.0), 5.0);
    /// assert_eq!(s.center, raytracer::Point3::new(1.0, 2.0, 3.0));
    /// assert_eq!(s.radius, 5.0);
    /// ```
    pub fn new(center: Point3, radius: f64, mat: Arc<dyn Material>) -> Self {
        Self {
            center,
            radius: f64::max(radius, 0.0),
            mat,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
        let oc = self.center - *r.origin();
        let a = r.direction().length_squared();
        let h = dot(r.direction(), &oc);
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = h * h - a * c;
        if discriminant < 0.0 {
            return false;
        }

        let sqrtd = f64::sqrt(discriminant);

        let mut root = (h - sqrtd) / a;
        if !ray_t.surrounds(root) {
            root = (h + sqrtd) / a;
            if !ray_t.surrounds(root) {
                return false;
            }
        }
        rec.t = root;
        rec.p = r.at(rec.t);
        let outward_normal = (rec.p - self.center) / self.radius;
        rec.set_face_normal(r, &outward_normal);
        rec.mat = self.mat.clone();
        true
    }
}
