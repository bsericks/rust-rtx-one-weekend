use rand::Rng;

pub const INFINITY : f32 = std::f32::INFINITY;
pub const PI : f32 = 3.1415926535897932385;


pub fn degrees_to_radians(degrees: f32) -> f32 {
    degrees * PI / 180.0
}


pub fn random_double(min: f32, max: f32) -> f32 {
    // a number from [-40.0, 13000.0)
    let mut rng = rand::thread_rng();
    rng.gen_range(min..max)
    
}