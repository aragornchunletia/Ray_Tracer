use crate::components::{Color, Point3, Ray};

pub fn hit_sphere(center:&Point3 , radius:f64 , r:&Ray) -> bool{
    let oc = center - r.origin() ;
    let a = r.direction().dot(r.direction()) ;
    let b = -2.0 * r.direction().dot(&oc) ;
    let c = oc.dot(&oc) - radius*radius ;
    let discriminant = b*b - 4.0*a*c ;

    discriminant >= 0.0

}

pub fn ray_color(r:&Ray) -> Color{
    let p:Point3 = Point3::new(&[0.0 , 0.0 , -1.0]) ;
    if hit_sphere(&p, 0.5, r){
        return Color::from_owned([1.0 , 0.0 , 0.0])
    }

    let unit_direction = r.direction().unit_vector() ;
    let a = 0.5*(unit_direction.y() + 1.0) ;
    (1.0 -a) * Color::from_owned([1.0 , 1.0 , 1.0]) + a * Color::from_owned([0.5 , 0.7 , 1.0])
}

