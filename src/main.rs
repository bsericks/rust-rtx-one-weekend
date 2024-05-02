mod vec3;
mod color;
mod ray;

use indicatif::{ProgressBar, ProgressStyle};

use crate::vec3::vec3::Vec3;
use crate::vec3::vec3::unit_vector;
use crate::vec3::vec3::Point3;
use crate::vec3::vec3::dot;
use crate::color::color::write_color;
use crate::color::color::Color;
use crate::ray::ray::Ray;
use std::io::{self, Write};



// bool hit_sphere(const point3& center, double radius, const ray& r) {
//     vec3 oc = center - r.origin();
//     auto a = dot(r.direction(), r.direction());
//     auto b = -2.0 * dot(r.direction(), oc);
//     auto c = dot(oc, oc) - radius*radius;
//     auto discriminant = b*b - 4*a*c;
//     return (discriminant >= 0);
// }

pub fn hit_sphere(center: Point3, radius: f64, r: &Ray) -> f64{
    let oc = center - *r.origin();
    let a = dot(*r.direction(), *r.direction());
    let b = -2.0 * dot(*r.direction(), oc);
    let c = dot(oc, oc) - radius*radius;
    let discriminant = b*b -4.0*a*c;

    if discriminant < 0.0 {
        return -1.0;
    } else {
        return (-b - discriminant.sqrt() ) / (2.0*a);
    }
}


pub fn ray_color(r: &Ray) -> Color {
//    if hit_sphere(Point3::new(0.0,0.0,-1.0), 0.5, r) {
//        return Color::new(1.0, 0.0, 0.0)
//    }

    let t = hit_sphere(Point3::new(0.0,0.0,-1.0), 0.5, r);
    
    if t > 0.0 {
        let N = unit_vector((r.at(t) - Vec3::new(0.0,0.0,-1.0)));
        //println!("N = {}", N);
        //println!()
        return Color::new(N.x()+1.0, N.y()+1.0, N.z()+1.0)*0.5
    }


    let unit_direction: Vec3 = unit_vector(*r.direction());
    let a = unit_direction.y()*0.5 + 1.0;
    Color::new(1.0, 1.0, 1.0)*(1.0-a) + Color::new(0.5, 0.7, 1.0)*a
}

fn main() {

    let aspect_ratio = 16.0/9.0 as f64;
    let image_width = 400 as u32;

    // Calculate image height, make sure its at least 1
    let _image_height = ((image_width as f64)/aspect_ratio) as u32;
    let image_height = if _image_height < 1 {1} else {_image_height};

    //Camera
    let focal_length = 1.0;
    let viewport_height = 2.0;
    let viewport_width = viewport_height * ((image_width/image_height) as f64);
    let camera_center = Point3::new(0.0,0.0,0.0);

    //Calculate the vectors across the horizontal and down the vertical viewport edges
    let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
    let viewport_v = Vec3::new(0.0, -viewport_width, 0.0);

    //Calculate h and v delta vectors from pixel to pixel
    let pixel_delta_u = viewport_u / (image_width as f64);
    let pixel_delta_v = viewport_v / (image_height as f64);

    //Calculate location of upper left pixel
    let viewport_upper_left = camera_center 
        - Vec3::new(0.0, 0.0, focal_length)
        - viewport_u/2.0 - viewport_v/2.0;

    let pixel00_loc = viewport_upper_left + ((pixel_delta_u + pixel_delta_v) * 0.5);

    let mut cout = io::stdout().lock();

    println!("P3");
    println!("{} {}", image_width, image_height);
    println!("255");

    // Create a new progress bar with a specified length
    let pb = ProgressBar::new(image_height as u64);

    pb.set_style(ProgressStyle::with_template("[{elapsed_precise}] {bar:100.cyan/blue} {pos:>7}/{len:7} {msg}")
        .unwrap()
        .progress_chars("##-"));

    for j in 0..image_height {
        pb.inc(1);

        for i in 0..image_width {
            
            let pixel_center = pixel00_loc + (pixel_delta_u * (i as f64)) + (pixel_delta_v * (j as f64));
            let ray_direction = pixel_center - camera_center;
            let r: Ray = Ray::new(camera_center, ray_direction);

            let pixel_color: Color = ray_color(&r);

            write_color(&mut cout, pixel_color);
        }
    }

}
