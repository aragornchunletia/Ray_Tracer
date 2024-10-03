use crate::components::{Color, Point3, Ray , Vec3};

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

#[cfg(test)]
mod tests {
    use super::*;  // Import necessary items from the parent module

    #[test]
    fn test_ray_hits_sphere_directly() {
        let sphere_center = Point3::new(&[0.0, 0.0, -1.0]);
        let radius = 0.5;

        // Ray pointing directly at the sphere
        let origin1 = Point3::new(&[0.0, 0.0, 0.0]);  // Camera at the origin
        let direction1 = Vec3::new(&[0.0, 0.0, -1.0]);  // Directly towards the sphere
        let ray1 = Ray::new(origin1, direction1);

        // Ensure the discriminant is positive
        assert!(hit_sphere(&sphere_center, radius, &ray1), "Ray should hit the sphere directly");
    }

    #[test]
    fn test_ray_hits_sphere_with_offset() {
        let sphere_center = Point3::new(&[0.0, 0.0, -1.0]);
        let radius = 0.5;

        // Ray with slight offset
        let origin2 = Point3::new(&[0.0, 0.5, 0.0]);  // Slightly above the origin
        let direction2 = Vec3::new(&[0.0, -0.5, -1.0]);  // Pointing towards the sphere
        let ray2 = Ray::new(origin2, direction2.unit_vector());

        // Ensure the discriminant is positive
        assert!(hit_sphere(&sphere_center, radius, &ray2), "Ray with slight offset should hit the sphere");
    }
}
