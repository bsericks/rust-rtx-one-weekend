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
use material::Dielectric;
use material::Lambertian;
use material::Metal;
use rtweekend::random_double;

use crate::camera::Camera;
use std::sync::Arc;
use crate::sphere::Sphere;

use crate::vec3::Vec3;
use crate::vec3::Point3;

use crate::interval::Interval;
use crate::hittable_list::HittableList;

fn main() -> () {

    // World
    let mut world = HittableList::new();

    let material_ground = Arc::new(Lambertian::new(Color::new(0.5,0.5,0.5)));
    let ground = Sphere { center: Point3::new(0.0,-1000.0,0.0), radius: 1000.0, mat: material_ground};
    world.add(Box::new(ground));
    
    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = random_double(0.0, 1.0);
            let center = Point3::new((a as f32) + 0.9*random_double(0.0, 1.0), 0.2, (b as f32) + 0.9*random_double(0.0, 1.0));
            
            if (center - Point3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.8 {
                    //diffuse
                    let albedo = vec3::random() * vec3::random();
                    let sphere_material = Arc::new(Lambertian::new(albedo));
                    let sphere =  Sphere { center: center, radius: 0.2, mat: sphere_material};
                    world.add(Box::new(sphere));
                }
                else if choose_mat < 0.95 {
                    // metal
                    let albedo = vec3::random_with_range(0.5, 1.0);
                    let fuzz = random_double(0.0, 0.5);
                    let sphere_material = Arc::new(Metal::new(albedo, fuzz));
                    let sphere = Sphere { center, radius: 0.2, mat: sphere_material};
                    world.add(Box::new(sphere));
                }
                else {
                    // glass
                    let sphere_material = Arc::new(Dielectric::new(1.5));
                    let sphere = Sphere { center, radius: 0.2, mat: sphere_material};
                    world.add(Box::new(sphere));
                }
            }
        }
    }

    let material1 = Arc::new(Dielectric::new(1.5));
    let sphere1 = Sphere { center: Point3::new(0.0,1.0,0.0), radius: 1.0, mat: material1};
    world.add(Box::new(sphere1));

    let material2 = Arc::new(Lambertian::new(Color::new(0.4,0.2,0.1)));
    let sphere2 = Sphere { center: Point3::new(-4.0, 1.0, 0.0), radius: 1.0, mat: material2};
    world.add(Box::new(sphere2));

    let material3  = Arc::new(Metal::new(Color::new(0.7,0.6,0.5), 0.0));
    let sphere3 = Sphere { center: Point3::new(4.0,1.0,0.0), radius: 1.0, mat: material3};
    world.add(Box::new(sphere3));



    let mut cam = Camera::new();
    cam.aspect_ratio = 16.0/9.0;
    cam.image_width = 1200;
    cam.samples_per_pixel = 500;
    cam.max_depth = 50;
    cam.lookfrom = Point3::new(13.0,2.0,3.0);
    cam.lookat   = Point3::new(0.0,0.0,-1.0);
    cam.vup      = Point3::new(0.0,1.0,0.0);

    cam.defocus_angle = 0.6;
    cam.focus_dist    = 10.0;

    cam.vfov = 20.0;

    let _ = cam.render(&world);
}
