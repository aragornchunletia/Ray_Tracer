use super::vec3::{Point3, Vec3};

pub struct Ray{
    origin: Point3,
    direction: Vec3,
}

impl Ray{
    pub fn new(origin:Point3 , direction: Vec3) -> Self{
        Self{origin, direction}
    }

    pub fn origin(&self) -> &Point3{
        &self.origin
    }

    pub fn direction(&self) -> &Vec3{
        &self.direction
    }

    pub fn at (&self , t:f64) -> Vec3{
        return self.origin + t * self.direction;
    }
}