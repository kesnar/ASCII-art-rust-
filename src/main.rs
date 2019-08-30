extern crate image;

use crate::image::GenericImageView;

fn main() {
    let img = match image::open("res/3.jpg") {
        Ok(a) => a,
        Err(_) => panic!("Incorrect image file"),
    };

    let (width, height) = img.dimensions();

    println!("Successfully loaded image!");
    println!("Image size: {} x {}", width, height);
}
