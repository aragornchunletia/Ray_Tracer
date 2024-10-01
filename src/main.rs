#![allow(non_snake_case)]
#[allow(unused)]
mod image_generator;
mod components;
fn main() {
    match image_generator::create_image() {
        Ok(((), name)) => {
            println!("{} Image generated.", name);
        }
        Err(e) => {
            println!("Error generating image: {}", e);
        }
    }
}

