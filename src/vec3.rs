pub mod vec3 {

    use std::ops::{Add, Sub, Neg, Mul, Div, Index, IndexMut, AddAssign, MulAssign, DivAssign};
    use std::fmt;

    #[derive(Copy, Clone)]
    pub struct Vec3 {
        pub array: [f64; 3],
    }

    impl Vec3 {
        pub fn new(x: f64, y:f64, z:f64) -> Vec3 {
            Vec3 { array: [x, y, z] }
        }

        pub fn x(self) -> f64 {
            self.array[0]
        }

        pub fn y(self) -> f64 {
            self.array[1]
        }

        pub fn z(self) -> f64 {
            self.array[2]
        }

        pub fn length_squared(self) -> f64 {
            self.array[0]*self.array[0] + self.array[1]*self.array[1] + self.array[2]*self.array[2]
        }

        pub fn length(self) -> f64 {
            self.length_squared().sqrt()
        }

        pub fn dot(self, u: Vec3, v: Vec3) -> f64 {
            u.x() * v.x() + u.y() * v.y() + u.z() * v.z() 
        }

        pub fn cross(self, u: Vec3, v: Vec3) -> Vec3 {
            Vec3{array: [ u[1] * v[2] - u[2] * v[1],
                          u[2] * v[0] - u[0] * v[2],
                          u[0] * v[1] - u[1] * v[0]]}
           
        }
    }

    impl Add for Vec3 {
        type Output = Vec3;
    
        fn add(self, other: Vec3) -> Vec3 {
            Vec3 {array: [self.x() + other.x(), self.y() + other.y(), self.z() + other.z()]}
        }
    }

    impl Sub for Vec3 {
        type Output = Vec3;
    
        fn sub(self, other: Vec3) -> Vec3 {
            Vec3 {array: [self.x() - other.x(), self.y() - other.y(), self.z() - other.z()]}
        }
    } 

    impl Neg for Vec3 {
        type Output = Vec3;
    
        fn neg(self) -> Vec3 {
            Vec3 {array: [-self.x(), -self.y(), -self.z()]}
        }
    } 

    impl Mul for Vec3 {
        type Output = Vec3;
    
        fn mul(self, other: Vec3) -> Vec3 {
            Vec3 {array: [self.x() * other.x(), self.y() * other.y(), self.z() * other.z()]}
        }
    } 

    impl Mul<f64> for Vec3 {
        type Output = Vec3;
    
        fn mul(self, other: f64) -> Vec3 {
            Vec3 {array: [self.x() * other, self.y() * other, self.z() * other]}
        }
    } 

    
    impl Div<f64> for Vec3 {
        type Output = Vec3;
    
        fn div(self, other: f64) -> Vec3 {
            Vec3 {array: (self * (1.0/other)).array}
        }
    } 

    impl fmt::Display for Vec3 {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "x:{} y:{} z:{}", self.array[0], self.array[1], self.array[2])
        }
    }

    impl Index<u32> for Vec3 {
        type Output = f64;
    
        fn index(&self, index: u32) -> &Self::Output {
            match index {
                0 => &self.array[0],
                1 => &self.array[1],
                2 => &self.array[2],
                //intentionally do a panic if this happens
                _ => &self.array[3],
            }
        }
    }
    
    impl IndexMut<u32> for Vec3 {
        fn index_mut(&mut self, index: u32) -> &mut Self::Output {
            match index {
                0 => &mut self.array[0],
                1 => &mut self.array[1],
                2 => &mut self.array[2],
                _ => &mut self.array[1],
            }
        }
    }
 
    impl AddAssign for Vec3 {
        fn add_assign(&mut self, other: Self) {
            *self = Self {
                array: [self.array[0] + other.array[0], 
                        self.array[1] + other.array[1],
                        self.array[2] + other.array[2]],
            };
        }
    }

    impl MulAssign<f64> for Vec3 {
        fn mul_assign(&mut self, other: f64) {
            *self = Self {
                array: [self.array[0] * other, 
                        self.array[1] * other,
                        self.array[2] * other],
            };
        }
    }


    impl DivAssign<f64> for Vec3 {
        fn div_assign(&mut self, other: f64) {
            *self = Self {
                array: [self.array[0] / other, 
                        self.array[1] / other,
                        self.array[2] / other],
            };
        }
    }

pub type Point3 = Vec3;

pub fn unit_vector(v: Vec3) -> Vec3 {
    v / v.length()
}

}