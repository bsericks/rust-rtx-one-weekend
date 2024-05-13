pub mod ray {

    use crate::vec3::Vec3;
    use crate::vec3::Point3;
    
    
    #[derive(Copy, Clone)]
    pub struct Ray {
        pub orig: Point3,
        pub dir: Vec3,
    }

    impl Ray {
        pub fn new(origin: Point3, direction:Vec3) -> Self {
            Self { orig: origin, dir: direction }
        }

        pub fn origin(&self) -> &Point3 {
            &self.orig
        }
        pub fn direction(&self) -> &Vec3 {
            &self.dir
        }

        pub fn at(self, t: f32) -> Point3 {
            self.orig + t*self.dir
        }

    }

}