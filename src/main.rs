mod vec3;
mod color;
mod ray;
mod hitable;
mod sphere;
mod hittable_list;
mod interval;
mod rtweekend;
mod camera;

use crate::camera::Camera;

use crate::sphere::Sphere;

use crate::vec3::Vec3;
use crate::vec3::Point3;

use crate::interval::Interval;
use crate::hittable_list::HittableList;

fn main() -> () {

    // World
    let mut world = HittableList::new();

    let sphere1 = Sphere { center: Point3::new(0.0,0.0,-1.0), radius: 0.5};
    world.add(&sphere1);
    let sphere2 = Sphere { center: Point3::new(0.0,-100.5,-1.0), radius: 100.0};
    world.add(&sphere2);

    let mut cam = Camera::new();
    cam.aspect_ratio = 16.0/9.0;
    cam.image_width = 400;

    let _ = cam.render(&world);
}
