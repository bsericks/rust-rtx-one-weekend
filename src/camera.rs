
use crate::hitable::Hitable;
use crate::color::color::Color;
use crate::material::Lambertian;
use crate::ray::ray::Ray;
use crate::rtweekend::degrees_to_radians;
use crate::vec3;
use crate::hitable;
use crate::interval;
use crate::rtweekend::INFINITY;
use crate::color::color::write_color;
use crate::vec3::random_in_unit_disk;

use std::fs::File;
use std::io::BufWriter;
use std::io::{self, Write};
use std::sync::Arc;
use indicatif::{ProgressBar, ProgressStyle};
use crate::rtweekend::random_double;

#[derive(Clone, Copy)]
pub struct Camera {
    pub aspect_ratio: f32,
    pub image_width: u32,
    pub samples_per_pixel: u32,
    pub max_depth: u32,
    pub vfov: f32,
    pub lookfrom: vec3::Point3,
    pub lookat: vec3::Point3,
    pub vup: vec3::Vec3,
    pub defocus_angle: f32,
    pub focus_dist: f32,
    image_height: u32,
    camera_center: vec3::Point3,
    pixel00_loc: vec3::Point3,
    pixel_delta_u: vec3::Vec3,
    pixel_delta_v: vec3::Vec3,
    pixel_samples_scale: f32,
    defocus_disk_u: vec3::Vec3,
    defocus_disk_v: vec3::Vec3,
}

impl Camera {

    pub fn new() -> Self {
        Self { aspect_ratio: 1.0,
                image_width: 400,
                image_height: 400,
                max_depth: 10,
                vfov: 90.0,
                lookfrom: vec3::Point3::new(0.0,0.0,0.0),
                lookat: vec3::Point3::new(0.0,0.0,-1.0),
                vup: vec3::Vec3::new(0.0,1.0,0.0),
                defocus_angle: 0.0,
                focus_dist: 0.0,
                samples_per_pixel: 100,
                pixel_samples_scale: 1.0/100.0,
                camera_center: vec3::Vec3::new(0.0,0.0,0.0),
                pixel00_loc: vec3::Point3::new(0.0,0.0,0.0),
                pixel_delta_u: vec3::Vec3::new(0.0,0.0,0.0),
                pixel_delta_v: vec3::Vec3::new(0.0,0.0,0.0),
                defocus_disk_u: vec3::Vec3::new(0.0,0.0,0.0),
                defocus_disk_v: vec3::Vec3::new(0.0,0.0,0.0), }
    }
    
    pub fn render(&mut self, world: &dyn Hitable) -> std::io::Result<()> {
        let _cout = io::stdout().lock();


        self.initialize();

        let file = File::create("pic.ppm")?;
        let mut writer = BufWriter::new(file);

        let _ = writer.write_all(b"P3\n");
        let line = format!("{} {}\n", self.image_width, self.image_height); // Construct the line as a String
        let _ = writer.write_all(line.as_bytes()); // Write the line to the file
        let _ = writer.write_all(b"255\n");

        // Create a new progress bar with a specified length
        let pb = ProgressBar::new(self.image_height as u64);

        pb.set_style(ProgressStyle::with_template("[{elapsed_precise}] {bar:100.cyan/blue} {pos:>7}/{len:7} {msg}")
            .unwrap()
            .progress_chars("##-"));

        for j in 0..self.image_height {
            pb.inc(1);

            for i in 0..self.image_width {

                let mut pixel_color: Color = Color::new(0.0, 0.0, 0.0);

                for _sample in 0..self.samples_per_pixel {
                    let r: Ray = self.get_ray(i, j);
                    pixel_color += self.ray_color(&r, self.max_depth, world);
    
                    //if i == 200 && j == 112 {
                    //    print!("Ray: {} {} {} \n", r.direction()[0], r.direction()[1], r.direction()[2]);
                    //    print!("Ray: {} {} {} \n", r.origin()[0], r.origin()[1], r.origin()[2]);
                    //}
                }
                

                write_color(&mut writer, self.pixel_samples_scale * pixel_color);
            }
        }
        Ok(())
    }

    fn initialize(&mut self) -> () {
        
        // Calculate image height, make sure its at least 1
        let _image_height = ((self.image_width as f32)/self.aspect_ratio) as u32;
        self.image_height = if _image_height < 1 {1} else {_image_height};
        
        self.camera_center = self.lookfrom;

        //Camera
        let theta = degrees_to_radians(self.vfov);
        let h = (theta/2.0).tan();
        let viewport_height = 2.0 * h * self.focus_dist;
        //let viewport_width = viewport_height * ((image_width/image_height) as f32);
        let viewport_width = self.aspect_ratio*viewport_height;

        let w = vec3::unit_vector(self.lookfrom - self.lookat);
        let u = vec3::unit_vector(self.vup.cross(w));
        let v = w.cross(u);

        //Calculate the vectors across th horizontal and down the vertical viewport edges
        let viewport_u = viewport_width * u;
        let viewport_v = viewport_height * -v;

        //Calculate h and v delta vectors from pixel to pixel
        self.pixel_delta_u = viewport_u / (self.image_width as f32);
        self.pixel_delta_v = viewport_v / (self.image_height as f32);

        //Calculate location of upper left pixel
        //let viewport_upper_left = self.camera_center 
        //    - vec3::Vec3::new(0.0, 0.0, focal_length)
        //    - viewport_u/2.0 - viewport_v/2.0;

        let viewport_upper_left = self.camera_center - (self.focus_dist * w) - viewport_u/2.0 - viewport_v/2.0;

        let defocus_radius = self.focus_dist * (degrees_to_radians(self.defocus_angle / 2.0)).tan();
        self.defocus_disk_u = u * defocus_radius;
        self.defocus_disk_v = v * defocus_radius;

        self.pixel00_loc = viewport_upper_left + 0.5 * (self.pixel_delta_u + self.pixel_delta_v);
        
    }

    fn ray_color(&self, r: &Ray, depth: u32, world: &dyn Hitable) -> Color {
        if depth <= 0 {
            return Color::new(0.0,0.0,0.0);
        }

        let mut rec = hitable::HitRecord { t : 0.0, 
            p : vec3::Vec3::new(0.0, 0.0, 0.0), 
            normal : vec3::Vec3::new(0.0, 0.0, 0.0), 
            front_face : true, mat: Arc::new(Lambertian{ albedo: vec3::Vec3::new(0.0,0.0,0.0) })  };
        
        if world.hit(r, interval::Interval::new_with_bounds(0.001, INFINITY), &mut rec) {
            let mut scattered = Ray::new(vec3::Vec3::new(0.0,0.0,0.0), vec3::Vec3::new(0.0,0.0,0.0));
            let mut attenuation = Color::new(0.0, 0.0, 0.0);

            if rec.mat.scatter(r, &rec, &mut attenuation, &mut scattered) {
                //print!("Ray: {} {} {} \n", attenuation.x(), attenuation.y(), attenuation.z());
                //print!("Ray: {} {} {} \n", scattered.direction()[0], scattered.direction()[1], scattered.direction()[2]);
                //print!("Ray: {} {} {} \n", scattered.origin()[0], scattered.origin()[1], scattered.origin()[2]);
                return attenuation * self.ray_color(&scattered, depth-1, world);
            }
            return Color::new(0.0, 0.0, 0.0);


            //let direction = rec.normal + vec3::random_unit_vector();
            //return 0.5* self.ray_color(&Ray::new(rec.p, direction), depth-1, world);
            //return rec.normal + Color::new(1.0, 1.0, 1.0) * 0.5;
        }
    
        let unit_direction: vec3::Vec3 = vec3::unit_vector(*r.direction());
        let a = unit_direction.y()*0.5 + 1.0;
        Color::new(1.0, 1.0, 1.0)*(1.0-a) + Color::new(0.5, 0.7, 1.0)*a
    }

    fn get_ray(&self, i: u32, j: u32) -> Ray {
        // Construct a camera ray originating from the defocus disk and directed at a randomly
        // sampled point around the pixel location i, j.

        let offset = self.sample_square();
        let pixel_sample = self.pixel00_loc
                            + ((offset.x() + i as f32) * self.pixel_delta_u)
                            + ((offset.y() + j as f32) * self.pixel_delta_v);
        let _ray_origin = self.camera_center;

        let mut ray_origin = self.defocus_disk_sample();
        if self.defocus_angle <= 0.0 {
            ray_origin = self.camera_center;
        } 
        let ray_direction = pixel_sample - ray_origin;

        return Ray::new(ray_origin, ray_direction);

    }

    fn defocus_disk_sample(&self) -> vec3::Point3 {
        let p = random_in_unit_disk();
        self.camera_center + (p[0] * self.defocus_disk_u) + (p[1] * self.defocus_disk_v)
    }

    fn sample_square(&self) -> vec3::Vec3 {
        return vec3::Vec3::new(random_double(0.0, 1.0) - 0.5, random_double(0.0, 1.0) - 0.5, 0.0);
    }

}