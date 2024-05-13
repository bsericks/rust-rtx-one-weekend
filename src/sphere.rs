
use crate::hitable::HitRecord;
use crate::hitable::Hitable;
use crate::ray::ray::Ray;
use crate::vec3::Vec3;

pub struct Sphere {
    pub center: Vec3,
    pub radius: f32
  }
  
    impl Hitable for Sphere {
        fn hit(&self, r: &Ray, tmin: f32, tmax: f32, hit_record: &mut HitRecord) -> bool {
            let oc = self.center - *r.origin();
            let a = r.direction().length_squared();
            let h = (*r.direction()).dot(oc);
            let c = oc.length_squared() - self.radius*self.radius;
            let discriminant = h*h - a*c;
            
            if discriminant < 0.0 {
                return false;
            } else {
                return true;
            }
        }
    }