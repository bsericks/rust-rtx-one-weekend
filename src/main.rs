mod vec3;
mod color;
mod ray;
mod hitable;
mod sphere;
mod hittable_list;
mod interval;
mod rtweekend;
mod camera;
mod material;

use color::color::Color;
use material::Lambertian;
use material::Metal;

use crate::camera::Camera;
use std::sync::Arc;
use crate::material::Material;
use crate::sphere::Sphere;

use crate::vec3::Vec3;
use crate::vec3::Point3;

use crate::interval::Interval;
use crate::hittable_list::HittableList;

fn main() -> () {

    // World
    let mut world = HittableList::new();



    let material_ground = Arc::new(Lambertian::new(Color::new(0.8,0.8,0.0)));
    let material_center = Arc::new(Lambertian::new(Color::new(0.1,0.2,0.5)));
    let material_left   = Arc::new(Metal::new(Color::new(0.8,0.8,0.8)));
    let material_right  = Arc::new(Metal::new(Color::new(0.8,0.6,0.2)));

    let ground = Sphere { center: Point3::new(0.0,-100.5,-1.0), radius: 100.0, mat: material_ground};
    world.add(&ground);
    let center = Sphere { center: Point3::new(0.0,0.0,-1.2), radius: 0.5, mat: material_center};
    world.add(&center);
    let left = Sphere { center: Point3::new(-1.0,0.0,-1.0), radius: 0.5, mat: material_left};
    world.add(&left);
    let right = Sphere { center: Point3::new(1.0,0.0,-1.0), radius: 0.5, mat: material_right};
    world.add(&right);


    let mut cam = Camera::new();
    cam.aspect_ratio = 16.0/9.0;
    cam.image_width = 400;

    let _ = cam.render(&world);
}
