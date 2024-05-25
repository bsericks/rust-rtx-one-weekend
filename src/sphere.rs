
use crate::hitable::HitRecord;
use crate::hitable::Hitable;
use crate::ray::ray::Ray;
use crate::vec3::Vec3;

pub struct Sphere {
    pub center: Vec3,
    pub radius: f32
  }
  
    impl Hitable for Sphere {
        fn hit(&self, r: &Ray, _tmin: f32, _tmax: f32, _hit_record: &mut HitRecord) -> bool {
            let oc = self.center - *r.origin();
            let a = r.direction().length_squared();
            let h = (*r.direction()).dot(oc);
            let c = oc.length_squared() - self.radius*self.radius;
            let discriminant = h*h - a*c;
            
            if discriminant < 0.0 {
                return false;
            }

            let sqrtd = discriminant.sqrt();

            let mut root = (h - sqrtd) / a;
            if root <= _tmin || _tmax <= root {
                root = (h + sqrtd) / a;
                if root <= _tmin || _tmax <= root {
                    return false;
                }
            }
        
            _hit_record.t = root;
            _hit_record.p = r.at(_hit_record.t);
            let outward_normal = (_hit_record.p - self.center) / self.radius;
            _hit_record.set_face_normal(r, &outward_normal);

            return true;
        
        }
    }