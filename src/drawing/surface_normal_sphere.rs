use crate::components::{ Color, HitRecord, HittableList, Ray, INFINITY};


pub fn ray_color(r : &Ray ,world:&HittableList ) -> Color {
    let mut rec = HitRecord::default() ;
    if world.hit(r , 0.0, INFINITY , &mut rec){
        return 0.5 * Color::new(&[rec.normal.x() + 1.0, rec.normal.y() + 1.0, rec.normal.z() + 1.0])
    }

    let unit_direction = r.direction().unit_vector() ;
    let a  = 0.5 * (unit_direction.y() + 1.0) ;
    return (1.0 -a) * (Color::from_owned([1.0 , 1.0, 1.0]) + (a*Color::from_owned([0.5 , 0.7 , 1.0]))) ;
}