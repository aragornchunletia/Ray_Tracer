use crate::components::{Color, Point3 , Ray};


fn double_hit_sphere(center: &Point3 , radius:f64 , r:&Ray ) -> f64{
    let oc = center - r.origin() ;
    let a = r.direction().length_square() ;
    let h = r.direction().dot(&oc) ;
    let c = oc.length_square() - radius*radius ;
    let discriminant = h*h - a*c ;

    if discriminant < 0.0{
        return -1.0 ;
    } else {
        (-h - discriminant.sqrt()) / a 
    }

}


pub fn ray_color(r : &Ray) -> Color {

    let radius = 0.5 ;
    let center = Point3::from_owned([0.0 , 0.0 , -1.0]) ;
    let t = double_hit_sphere(&center, radius, r) ;
    if t > 0.0 {
        let normal = (r.at(t) - center).unit_vector() ;
        return 0.5* Color::new(&[normal.x()+1.0 , normal.y()+1.0 , normal.z()+1.0]);
    }

    let unit_direction = r.direction().unit_vector();
    let a = 0.5 * (unit_direction.y() + 1.0) ;
    (1.0 -a ) * Color::from_owned([1.0 , 1.0 , 1.0]) + a*Color::new(&[0.5 , 0.7 , 1.0])
}