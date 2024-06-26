
use std::vec::Vec;
use crate::Vec3;
use crate::hitable::HitRecord;
use crate::ray::ray::Ray;
use crate::hitable::Hitable;
use crate::Interval;


pub struct HittableList<'a> {
    pub objects: Vec<&'a dyn Hitable>,
  }

impl<'a> HittableList<'a> {
    pub fn new() -> HittableList<'a> {
      HittableList { objects : Vec::new() }
    }

    pub fn clear(&mut self)
    {
        self.objects.clear();
    }

    pub fn add(&mut self, object: &'a dyn Hitable)
    {
        self.objects.push(object);
    }

    //pub fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
       
    //}
}

impl Hitable for HittableList<'_> {
  fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
    let mut temp_rec = HitRecord { t : 0.0, 
      p : Vec3::new(0.0, 0.0, 0.0), 
      normal : Vec3::new(0.0, 0.0, 0.0), 
      front_face : true  };
        let mut hit_anything = false;
        let mut closest_so_far = ray_t.max;

        for object in &self.objects {
          
          if object.hit(r, Interval::new_with_bounds(ray_t.min as f32, closest_so_far as f32), &mut temp_rec) {
            hit_anything = true;
            closest_so_far = temp_rec.t as f32;
            *rec = temp_rec.clone();
          }

        }

        return hit_anything;
  }
}