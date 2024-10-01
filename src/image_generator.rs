use std::fs::File;
use std::io::{Write};
use std::path::{Path};
use anyhow::Result; // For better error handling
use crate::components::{vec3 , color , progress_bar , ray} ;
use crate::drawing;
use vec3::{Point3 , Vec3};


pub fn create_image() -> Result<((), String)> {
    let name = "red_sphere.ppm".to_string();
    let image_dir = Path::new(r"C:\Users\mchun\RustTutorials\ray_tracer_images");
    let file_path = image_dir.join(&name);
    
    let mut data_file = File::create(&file_path)?;

    let aspect_ratio = 16.0 / 9.0 ;
    let image_width = 400 ;
    let image_height = (image_width as f64 / aspect_ratio) as u32 ;
    let image_height = if image_height < 1 {1} else {image_height};

    let focal_length: f64 = 1.0 ;
    let viewport_height = 2.0 ;
    let viewport_width = viewport_height * (image_width / image_height) as f64 ;
    let camera_center:Point3 = Point3::from_owned([0.0 , 0.0 , 0.0]) ;

    let viewport_u = Vec3::from_owned([viewport_width, 0.0 , 0.0]) ;
    let viewport_v = Vec3::from_owned([0.0, -viewport_height, 0.0]) ;

    let pixel_delta_u = &viewport_u / (viewport_width as f64 );
    let pixel_delta_v = &viewport_v/ (viewport_height as f64 );

    let viewport_upper_left = camera_center - Vec3::new(&[0.0 , 0.0 , focal_length as f64])  
                                                - viewport_u / 2.0
                                                -viewport_v / 2.0;

    
    let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v) ;

    let pb = progress_bar::progressBar() ; 
    pb.set_message("Image generation started");
    pb.tick();
    
    // Write the PPM header
    writeln!(data_file, "P3\n{} {}\n255", image_width, image_height)?;

    // Loop to generate image pixel by pixel
    for j in 0..image_height {
        for i in 0..image_width {
            let i = i as f64 ;
            let j = j as f64 ;
            let pixel_center = pixel00_loc + (i * pixel_delta_u) + (j * pixel_delta_v) ;
            let ray_direction:Point3 = camera_center - pixel_center ;
            let ray = ray::Ray::new(camera_center, ray_direction);
            let color_pixel:color::Color = drawing::red_sphere::ray_color(&ray);
            let _ = color::write_color(color_pixel, &mut data_file) ;


        }
        pb.inc(1);
    }

    pb.set_message("Image generation completed");
    pb.finish_and_clear();
    
    Ok(((), name))
}
