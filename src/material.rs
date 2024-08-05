use crate::color::color::Color;
use crate::hitable::HitRecord;
use crate::ray::ray::Ray;
use crate::rtweekend::{self, random_double};
use crate::vec3::{random_unit_vector, reflect, refract, unit_vector, Vec3};

pub trait Material: Sync + Send {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool;
}

pub struct Lambertian {
    pub(crate) albedo: Color,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Lambertian {
        Lambertian { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        let mut scatter_direction = rec.normal + random_unit_vector();

        if scatter_direction.near_zero() {
            scatter_direction = rec.normal;
        }

        *scattered = Ray::new(rec.p, scatter_direction);
        *attenuation = self.albedo;
        true
    }
}

pub struct Metal {
    albedo: Color,
}

impl Metal {
    pub fn new(albedo: Color) -> Metal {
        Metal { albedo }
    }
}

impl Material for Metal {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        let mut reflected = reflect(r_in.direction(), &rec.normal);

        *scattered = Ray::new(rec.p, reflected);
        *attenuation = self.albedo;
        true
    }
}

pub struct Dielectric
{
    refraction_index: f32,
}

impl Dielectric {
    pub fn new(refraction_index: f32) -> Dielectric {
        Dielectric { refraction_index }
    }

    fn reflectance(cosine: f32, refraction_index: f32 ) -> f32 {
        let mut r0 = (1.0 - refraction_index) / (1.0 + refraction_index);
        r0 = r0*r0;
        return r0 + (1.0 - r0)*((1.0 - cosine).powf(5.0));
    }
}

impl Material for Dielectric {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        *attenuation = Color::new(1.0, 1.0, 1.0);
        let mut ri = self.refraction_index;
        if rec.front_face {
            ri = 1.0/self.refraction_index;
        }

        let unit_direction = unit_vector(*r_in.direction());
        let cos_theta = (-unit_direction.dot(rec.normal).min(1.0));
        let sin_theta = (1.0 - cos_theta*cos_theta).sqrt();

        let cannot_refract = ri*sin_theta > 1.0;
        
        //let refracted = refract(&unit_direction, &rec.normal, ri);

        let mut _direction = Vec3::new(1.0, 1.0, 1.0);

        if cannot_refract || Dielectric::reflectance(cos_theta, ri) > random_double(0.0, 1.0) {
            _direction = reflect(&unit_direction, &rec.normal);
        }
        else {
            _direction = refract(&unit_direction, &rec.normal, ri);
        }

        *scattered = Ray::new(rec.p, _direction);

        true
    }
}
