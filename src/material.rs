use crate::color::color::Color;
use crate::hitable::HitRecord;
use crate::ray::ray::Ray;
use crate::rtweekend;
use crate::vec3::random_unit_vector;

#[derive(Clone, Copy)]
pub struct Material {}

impl Material {
    pub fn new() -> Material {
        Material {}
    }
}

pub trait Scatter: Sync {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, attenuation: &Color) -> bool;
}


pub struct Lambertian {
  albedo: Color,
}

impl Lambertian {
  pub fn new(albedo: Color ) -> Lambertian {
    Lambertian{albedo}
  }
}

impl Scatter for Lambertian {
  fn scatter<'a>(&'a self, r_in: &Ray, rec: &HitRecord, mut attenuation: &'a Color) -> bool {
    let mut scatter_direction = rec.normal + random_unit_vector();

     if scatter_direction.near_zero() {
        scatter_direction = rec.normal;
     }

    let scattered = Ray::new(rec.p, scatter_direction);
    attenuation = &self.albedo;
    true
  }
}