use std::ops::{Add, Div, Mul, Neg, Sub , Index , IndexMut} ;
use std::array::from_fn;
use std::fmt::Display;

#[derive(Debug, Clone , Copy, PartialEq)]

pub struct Vec3{
    values : [f32;3],
}

pub type Point3 = Vec3 ;

impl Vec3
{

    pub fn new (values:&[f32;3]) -> Vec3{
        Self { values: values.to_owned() }
    }

    pub fn from_owned(values: [f32; 3]) -> Vec3 {
        Self { values }
    }
    

    pub fn x(&self) -> &f32{
        &self.values[0]
    }

    pub fn y(&self) -> &f32{
        &self.values[1]
    }

    pub fn z(&self) -> &f32{
        &self.values[2]
    }

    pub fn length_square(&self) -> f32{
        self.values[0].powi(2) + self.values[1].powi(2) + self.values[2].powi(2)
    }

    pub fn length(&self) -> f32{
        self.length_square().sqrt()
    }


    pub fn dot(&self , other:&Vec3) -> f32{
        self.values[0] * other.values[0] + self.values[1] * other.values[1] + self.values[2] * other.values[2]
    }

    pub fn cross(&self , other:&Vec3) -> Vec3{
        Vec3::new(&[
            self.values[1] * other.values[2] - self.values[2] * other.values[1],
            self.values[2] * other.values[0] - self.values[0] * other.values[2],
            self.values[0] * other.values[1] - self.values[1] * other.values[0],
        ])
    }

    pub fn unit_vector(&self) -> Vec3{
        self / self.length() 
    }


}


impl Display for Vec3{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}, {})", self.values[0], self.values[1], self.values[2])
    }
}

impl Neg for Vec3{
 type Output =  Vec3;
 fn neg(self) -> Self::Output{
        Vec3::new(&[-self.values[0], -self.values[1], -self.values[2]])
    }
}

impl Index<usize> for Vec3{
    type Output = f32;
    fn index(&self, index: usize) -> &Self::Output {
        &self.values[index]
    }
}

impl IndexMut<usize> for Vec3{
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.values[index]
    }
}

impl Add for &Vec3{
    type Output = Vec3 ;

    fn add(self , rhs:Self) -> Self::Output{
        let res = from_fn(|i| self.values[i] + rhs.values[i]);
        Vec3::new(&res)
    }
}

impl Add for Vec3{
    type Output = Vec3 ;
    fn add(self , rhs:Self) -> Self::Output{
        &self + &rhs
    }
}


impl Sub for &Vec3{
    type Output = Vec3 ;

    fn sub(self , rhs:Self) -> Self::Output{
        let res = from_fn(|i| self.values[i] - rhs.values[i]);
        Vec3::new(&res)
    }
}

impl Sub for Vec3{
    type Output = Vec3 ;
    fn sub(self, rhs: Self) -> Self::Output {
        &self - &rhs
    }
}


impl Mul for &Vec3{
    type Output = Vec3 ;

    fn mul(self , rhs:Self) -> Self::Output{

        let res = from_fn(|i| (self.values[i] * rhs.values[i]));
        Vec3::new(&res)
    }
}

impl Mul for Vec3{
    type Output = Vec3 ;
    fn mul(self , rhs:Self) -> Self::Output{
        &self * &rhs
    }
}

impl Mul<f32> for &Vec3{
    type Output = Vec3 ;

    fn mul (self , scaler:f32) -> Self::Output{
        let res = from_fn(|i| self.values[i] * scaler);
        Vec3::new(&res)
    }
}

impl Mul<f32> for Vec3{ 
    type Output = Vec3 ;
    fn mul(self, scaler:f32) -> Self::Output{
        &self * scaler
    }
}

impl Mul<Vec3> for f32 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Vec3 {
        Vec3::new(&[
            self * rhs.x(),
            self * rhs.y(),
            self * rhs.z(),
        ])
    }
}

impl Div for &Vec3{
    type Output = Vec3 ;

    fn div(self, rhs:Self) -> Self::Output{
        let res = from_fn(|i|self.values[i] / rhs.values[i]);
        Vec3::new(&res)
    }
}

impl Div for Vec3{
    type Output = Vec3;
    fn div(self, rhs: Self) -> Self::Output {   
        &self / &rhs
    }
}

impl Div<f32> for &Vec3{
    type Output = Vec3 ;

    fn div(self, scaler:f32) -> Self::Output{
        let res = from_fn(|i| self.values[i] / scaler);
        Vec3::new(&res)
    }
}

impl Div<f32> for Vec3{
    type Output = Vec3 ;
    fn div(self, rhs: f32) -> Self::Output {
        &self / rhs 
    }
}





#[cfg(test)]
mod tests{
    use super::* ;

    #[test]
    fn add_test(){
        let a = Vec3::new(&[1.0 , 2.0 , 3.0]) ;
        let b = Vec3::new(&[0.1 , 0.2, 0.3]);
        assert_eq!(&a + &b , Vec3::new(&[1.1 , 2.2 , 3.3 ]));
    }
    #[test]
    fn sub_test(){
        let a = Vec3::new(&[1.5 , 2.5 , 3.5]) ;
        let b = Vec3::new(&[0.5 , 0.5, 0.5]);
        assert_eq!(&a - &b , Vec3::new(&[1.0 , 2.0 , 3.0 ]));
    }
    #[test]
    fn mul_test(){
        let a = Vec3::new(&[0.2 , 0.3 , 0.4]) ;
        let b = Vec3::new(&[0.1 , 0.2, 0.3]);
        assert_eq!(&a * &b , Vec3::new(&[0.020000001, 0.060000002, 0.120000005]));
    
    }

    #[test]
    fn div_test(){
        let a = Vec3::new(&[1.0 , 1.0 , 1.0]) ;
        let b = Vec3::new(&[0.5 , 0.2, 0.4]);
        assert_eq!(&a / &b , Vec3::new(&[2.0 , 5.0 , 2.5 ]));

    }

    #[test]
    fn scaler_mul_test(){
        let a = Vec3::new(&[1.0 , 2.0 , 3.0]) ;
        let b = &a.clone() * 3.0;
        assert_eq!(b , Vec3::new(&[3.0 , 6.0 , 9.0 ]));
    }

    #[test]

    fn scaler_div_test(){
        let a = Vec3::new(&[1.0 , 1.0 , 1.0]) ;
        let b = &a.clone() / 2.0;
        assert_eq!(b , Vec3::new(&[0.5 , 0.5 , 0.5 ]));
    }
    #[test]

    fn length_square_test(){
        let a = Vec3::new(&[1.0 , 2.0 , 3.0]) ;
        assert_eq!(a.length_square(), 14.0);
    }

    #[test]
    fn dot_product_test(){
        let a = Vec3::new(&[1.0 , 2.0 , 3.0]) ;
        let b = Vec3::new(&[4.0 , 5.0 , 6.0]) ;
        assert_eq!(a.dot(&b), 32.0);
    }

    #[test]
    fn cross_product_test(){
        let a = Vec3::new(&[1.0 , 2.0 , 3.0]) ;
        let b = Vec3::new(&[4.0 , 5.0 , 6.0]) ;
        assert_eq!(a.cross(&b), Vec3::new(&[-3.0, 6.0, -3.0]));
    }

}