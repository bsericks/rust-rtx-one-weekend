
use std::vec::Vec;
use crate::Vec3;
use crate::hitable::HitRecord;
use crate::ray::ray::Ray;
use crate::hitable::Hitable;

pub struct HittableList {
    pub objects: Vec<HitRecord>,
  }

impl HittableList {
    pub fn new() -> HittableList {
      HittableList { objects : Vec::new() }
    }

    pub fn clear(&mut self)
    {
        self.objects.clear();
    }

    pub fn add(&mut self, object: &HitRecord)
    {
        self.objects.push(*object);
    }

    pub fn hit(&self, r: &Ray, ray_tmin: f64, ray_tmax: f64, rec: &mut HitRecord) -> bool {
       let mut temp_rec = HitRecord { t : 0.0, 
                                  p : Vec3::new(0.0, 0.0, 0.0), 
                                  normal : Vec3::new(0.0, 0.0, 0.0), 
                                  front_face : true  };
       let mut hit_anything = false;
       let mut closest_so_far = ray_tmax;

      for object in &self.objects {
        if object.hit(r, ray_tmin as f32, closest_so_far as f32, &mut temp_rec) {
          hit_anything = true;
          closest_so_far = temp_rec.t as f64;
          *rec = temp_rec.clone();
        }
      
      }  
       //for (const auto& object : objects) {
       //    if (object->hit(r, ray_tmin, closest_so_far, temp_rec)) {
       //        hit_anything = true;
       //        closest_so_far = temp_rec.t;
       //        rec = temp_rec;
       //    }
       //}

        return hit_anything;
    }

}