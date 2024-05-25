use crate::ray::ray::Ray;
use crate::vec3::Vec3;

#[derive(Clone, Copy)]
pub struct HitRecord {
  pub t: f32,
  pub p: Vec3,
  pub normal: Vec3,
  pub front_face: bool,
}

impl HitRecord {
  pub fn set_face_normal(&mut self, r: &Ray, outward_normal: &Vec3) {
    // Sets the hit record normal vector.
    // NOTE: the parameter `outward_normal` is assumed to have unit length.

    self.front_face = r.direction().dot(*outward_normal) < 0.0;
    self.normal = if self.front_face {*outward_normal} else {-(*outward_normal)};
  }
}

impl Hitable for HitRecord {

  
  fn hit(&self, _r: &Ray, _tmin: f32, _tmax: f32, _hit_record: &mut HitRecord) -> bool {
    return false
  }
}

pub trait Hitable : Sync {
    fn hit(&self, r: &Ray, tmin: f32, tmax: f32, hit_record: &mut HitRecord) -> bool;
  }
