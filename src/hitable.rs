use crate::ray::ray::Ray;
use crate::vec3::Vec3;

#[derive(Clone, Copy)]
pub struct HitRecord {
  pub t: f32,
  pub p: Vec3,
  pub normal: Vec3
}


pub trait Hitable : Sync {
    fn hit(&self, r: &Ray, tmin: f32, tmax: f32, hit_record: &mut HitRecord) -> bool;
  }
