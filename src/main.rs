mod vec3;

use indicatif::{ProgressBar, ProgressStyle};

use crate::vec3::vec3::Vec3;

fn main() {
    let image_width = 256;
    let image_height = 256;

    println!("P3");
    println!("{} {}", image_width, image_height);
    println!("255");

    let v = Vec3::new(1.0, 2.0, 3.0);

    let v2 = Vec3::new(1.0, 2.0, 3.3);

    let mut v3 = v+v2;

    println!("vec3[x] = {}", v3[2]);
    v3[2] = 1.111;
    println!("vec3[x] = {}", v3[2]);

    // Create a new progress bar with a specified length
    let pb = ProgressBar::new(image_height);

    pb.set_style(ProgressStyle::with_template("[{elapsed_precise}] {bar:100.cyan/blue} {pos:>7}/{len:7} {msg}")
        .unwrap()
        .progress_chars("##-"));

    for j in 0..image_height {
        pb.inc(1);

        for i in 0..image_width {
            let r = (i as f64 / (image_width-1) as f64) as f64;
            let g = (j as f64 / (image_height-1) as f64) as f64;

            let b = 0.0;

            let ir = (255.999 * r) as u32;
            let ig = (255.999 * g) as u32;
            let ib = (255.999 * b) as u32;
            
            //println!("{} {} {}", ir, ig, ib);
        }
    }

}
