mod vec3;
mod color;
mod ray;
mod hitable;
mod sphere;
mod hittable_list;
mod interval;
mod rtweekend;

use crate::rtweekend::INFINITY;

use indicatif::{ProgressBar, ProgressStyle};

use std::fs::File;

use crate::sphere::Sphere;
use crate::hitable::HitRecord;
use std::io::BufWriter;
use crate::vec3::Vec3;
use crate::vec3::unit_vector;
use crate::vec3::Point3;
use crate::color::color::write_color;
use crate::color::color::Color;
use crate::ray::ray::Ray;
use crate::interval::Interval;
use crate::hittable_list::HittableList;
use std::io::{self, Write};

// pub fn hit_sphere(center: Point3, radius: f32, r: &Ray) -> f32{
//     let oc = center - *r.origin();

//     let a = r.direction().length_squared();
//     let h = (*r.direction()).dot(oc);
//     let c = oc.length_squared() - radius*radius;
//     let discriminant = h*h - a*c;
    
//     if discriminant < 0.0 {
//         return -1.0;
//     } else {
//         return (h - discriminant.sqrt()) / a;
//     }
//}


pub fn ray_color(r: &Ray, world: &HittableList) -> Color {

    let mut rec = HitRecord { t : 0.0, 
        p : Vec3::new(0.0, 0.0, 0.0), 
        normal : Vec3::new(0.0, 0.0, 0.0), 
        front_face : true  };
    
    if world.hit(r, Interval::new_with_bounds(0.0, INFINITY), &mut rec) {
        return rec.normal + Color::new(1.0, 1.0, 1.0) * 0.5;
    }

    let unit_direction: Vec3 = unit_vector(*r.direction());
    let a = unit_direction.y()*0.5 + 1.0;
    Color::new(1.0, 1.0, 1.0)*(1.0-a) + Color::new(0.5, 0.7, 1.0)*a
}

fn main() -> std::io::Result<()> {

    let aspect_ratio = 16.0/9.0 as f32;
    let image_width = 400 as u32;

    // Calculate image height, make sure its at least 1
    let _image_height = ((image_width as f32)/aspect_ratio) as u32;
    let image_height = if _image_height < 1 {1} else {_image_height};

    // World

    let mut world = HittableList::new();

    let sphere1 = Sphere { center: Point3::new(0.0,0.0,-1.0), radius: 0.5};
    world.add(&sphere1);
    let sphere2 = Sphere { center: Point3::new(0.0,-100.5,-1.0), radius: 100.0};
    world.add(&sphere2);


    //Camera
    let focal_length = 1.0;
    let viewport_height = 2.0;
    //let viewport_width = viewport_height * ((image_width/image_height) as f32);
    let viewport_width = aspect_ratio*viewport_height;
    let camera_center = Point3::new(0.0,0.0,0.0);

    //Calculate the vectors across the horizontal and down the vertical viewport edges
    let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
    let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);

    //Calculate h and v delta vectors from pixel to pixel
    let pixel_delta_u = viewport_u / (image_width as f32);
    let pixel_delta_v = viewport_v / (image_height as f32);

    //Calculate location of upper left pixel
    let viewport_upper_left = camera_center 
        - Vec3::new(0.0, 0.0, focal_length)
        - viewport_u/2.0 - viewport_v/2.0;

    let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

    let _cout = io::stdout().lock();

    let file = File::create("pic.ppm")?;
    let mut writer = BufWriter::new(file);

    let _ = writer.write_all(b"P3\n");
    let line = format!("{} {}\n", image_width, image_height); // Construct the line as a String
    let _ = writer.write_all(line.as_bytes()); // Write the line to the file
    let _ = writer.write_all(b"255\n");

    // Create a new progress bar with a specified length
    let pb = ProgressBar::new(image_height as u64);

    pb.set_style(ProgressStyle::with_template("[{elapsed_precise}] {bar:100.cyan/blue} {pos:>7}/{len:7} {msg}")
        .unwrap()
        .progress_chars("##-"));

    for j in 0..image_height {
        pb.inc(1);

        for i in 0..image_width {
            
            let pixel_center = pixel00_loc + (pixel_delta_u * (i as f32)) + (pixel_delta_v * (j as f32));
            let ray_direction = pixel_center - camera_center;
            let r: Ray = Ray::new(camera_center, ray_direction);

            let pixel_color: Color = ray_color(&r, &world);

            write_color(&mut writer, pixel_color);
        }
    }
    Ok(())
}
