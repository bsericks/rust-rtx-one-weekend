use crate::rtweekend::random_double;
use std::f32;
use std::ops::*;

#[derive(Clone, Copy, Debug)]
pub struct Vec3 {
    pub e: [f32; 3],
}

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Vec3 {
        Vec3 { e: [x, y, z] }
    }

    pub fn x(&self) -> f32 {
        self.e[0]
    }

    pub fn y(&self) -> f32 {
        self.e[1]
    }

    pub fn z(&self) -> f32 {
        self.e[2]
    }

    pub fn r(&self) -> f32 {
        self.e[0]
    }

    pub fn g(&self) -> f32 {
        self.e[1]
    }

    pub fn b(&self) -> f32 {
        self.e[2]
    }

    pub fn length(&self) -> f32 {
        (self.length_squared()).sqrt()
    }

    pub fn length_squared(&self) -> f32 {
        self.e[0] * self.e[0] + self.e[1] * self.e[1] + self.e[2] * self.e[2]
    }

    pub fn near_zero(&self) -> bool {
        let s = 1e-8;
        (self.e[0].abs() < s) && (self.e[0].abs() < s) && (self.e[0].abs() < s)
    }

    pub fn dot(&self, v2: Vec3) -> f32 {
        self.e[0] * v2.e[0] + self.e[1] * v2.e[1] + self.e[2] * v2.e[2]
    }

    pub fn cross(&self, v2: Vec3) -> Vec3 {
        Vec3 {
            e: [
                self.e[1] * v2.e[2] - self.e[2] * v2.e[1],
                -(self.e[0] * v2.e[2] - self.e[2] * v2.e[0]),
                self.e[0] * v2.e[1] - self.e[1] * v2.e[0],
            ],
        }
    }
}

impl Add for Vec3 {
    type Output = Vec3;

    fn add(self, other: Vec3) -> Vec3 {
        Vec3 {
            e: [
                self.e[0] + other.e[0],
                self.e[1] + other.e[1],
                self.e[2] + other.e[2],
            ],
        }
    }
}

impl Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, other: Vec3) -> Vec3 {
        Vec3 {
            e: [
                self.e[0] - other.e[0],
                self.e[1] - other.e[1],
                self.e[2] - other.e[2],
            ],
        }
    }
}

impl Mul for Vec3 {
    type Output = Vec3;

    fn mul(self, other: Vec3) -> Vec3 {
        Vec3 {
            e: [
                self.e[0] * other.e[0],
                self.e[1] * other.e[1],
                self.e[2] * other.e[2],
            ],
        }
    }
}

impl Div for Vec3 {
    type Output = Vec3;

    fn div(self, other: Vec3) -> Vec3 {
        Vec3 {
            e: [
                self.e[0] / other.e[0],
                self.e[1] / other.e[1],
                self.e[2] / other.e[2],
            ],
        }
    }
}

impl Mul<Vec3> for f32 {
    type Output = Vec3;

    fn mul(self, v: Vec3) -> Vec3 {
        Vec3 {
            e: [self * v.e[0], self * v.e[1], self * v.e[2]],
        }
    }
}

impl Div<f32> for Vec3 {
    type Output = Vec3;

    fn div(self, t: f32) -> Vec3 {
        Vec3 {
            e: [self.e[0] / t, self.e[1] / t, self.e[2] / t],
        }
    }
}

impl Mul<f32> for Vec3 {
    type Output = Vec3;

    fn mul(self, t: f32) -> Vec3 {
        Vec3 {
            e: [self.e[0] * t, self.e[1] * t, self.e[2] * t],
        }
    }
}

impl Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Vec3 {
        Vec3 {
            e: [-self.e[0], -self.e[1], -self.e[2]],
        }
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, other: Vec3) {
        self.e[0] += other.e[0];
        self.e[1] += other.e[1];
        self.e[2] += other.e[2];
    }
}

impl SubAssign for Vec3 {
    fn sub_assign(&mut self, other: Vec3) {
        self.e[0] -= other.e[0];
        self.e[1] -= other.e[1];
        self.e[2] -= other.e[2];
    }
}

impl MulAssign for Vec3 {
    fn mul_assign(&mut self, other: Vec3) {
        self.e[0] *= other.e[0];
        self.e[1] *= other.e[1];
        self.e[2] *= other.e[2];
    }
}

impl DivAssign for Vec3 {
    fn div_assign(&mut self, other: Vec3) {
        self.e[0] /= other.e[0];
        self.e[1] /= other.e[1];
        self.e[2] /= other.e[2];
    }
}

impl MulAssign<f32> for Vec3 {
    fn mul_assign(&mut self, t: f32) {
        self.e[0] *= t;
        self.e[1] *= t;
        self.e[2] *= t;
    }
}

impl DivAssign<f32> for Vec3 {
    fn div_assign(&mut self, t: f32) {
        let k = 1.0 / t;

        self.e[0] *= k;
        self.e[1] *= k;
        self.e[2] *= k;
    }
}

impl Index<usize> for Vec3 {
    type Output = f32;

    fn index(&self, i: usize) -> &f32 {
        &self.e[i]
    }
}

impl IndexMut<usize> for Vec3 {
    fn index_mut(&mut self, i: usize) -> &mut f32 {
        &mut self.e[i]
    }
}

pub fn unit_vector(v: Vec3) -> Vec3 {
    v / v.length()
}

pub fn random_in_unit_disk() -> Vec3 {
    loop {
        let p = Vec3::new(random_double(-1.0,1.0), random_double(-1.0,1.0), 0.0);
        if p.length_squared() < 1.0 {
            return p;
        }
    }
}

pub type Point3 = Vec3;

pub fn random() -> Vec3 {
    Vec3::new(
        random_double(0.0, 1.0),
        random_double(0.0, 1.0),
        random_double(0.0, 1.0),
    )
}

pub fn random_with_range(min: f32, max: f32) -> Vec3 {
    Vec3::new(
        random_double(min, max),
        random_double(min, max),
        random_double(min, max),
    )
}

pub fn random_in_unit_sphere() -> Vec3 {
    loop {
        let p = random_with_range(-1.0, 1.0);
        if p.length_squared() < 1.0 {
            return p;
        }
    }
}

pub fn random_unit_vector() -> Vec3 {
    unit_vector(random_in_unit_sphere())
}

pub fn random_on_hemisphere(normal: Vec3) -> Vec3 {
    let on_unit_sphere = random_unit_vector();
    if on_unit_sphere.dot(normal) > 0.0 {
        return on_unit_sphere;
    } else {
        return -on_unit_sphere;
    }
}

pub fn reflect(v: &Vec3, n: &Vec3) -> Vec3 {
    return *v - v.dot(*n) * 2.0 * (*n);
}

pub fn refract(uv: &Vec3, n: &Vec3, etai_over_etat: f32) -> Vec3 {
    let cos_theta = (-uv.dot(*n)).min(1.0);
    let r_out_perp = etai_over_etat * ((*uv) + cos_theta * (*n));
    let r_out_parallel = -(((1.0 - r_out_perp.length_squared()).abs()).sqrt()) * (*n);
    return r_out_perp + r_out_parallel;
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cross_product1() {
        let a = Vec3::new(1.0, 0.0, 0.0);
        let b = Vec3::new(0.0, 1.0, 0.0);

        let c = a.cross(b);
        assert_eq!(0.0, c.x());
        assert_eq!(0.0, c.y());
        assert_eq!(1.0, c.z());
    }

    #[test]
    fn cross_product2() {
        let a = Vec3::new(2.0, 3.0, 4.0);
        let b = Vec3::new(5.0, 6.0, 7.0);

        let c = a.cross(b);
        assert_eq!(-3.0, c.x());
        assert_eq!(6.0, c.y());
        assert_eq!(-3.0, c.z());
    }

    #[test]
    fn reflect_test() {

        let v = Vec3::new(1.0, 2.0, 3.0);
        let n = Vec3::new(0.0, 0.0, 1.0);

        let c: Vec3 = reflect(&v, &unit_vector(n));
        assert_eq!(1.0, c.x());
        assert_eq!(2.0, c.y());
        assert_eq!(-3.0, c.z());

    }
}
