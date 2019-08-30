extern crate image;

use crate::image::GenericImageView;
use crate::image::FilterType;

fn main() {
    let mut img = match image::open("res/4.jpg") {
        Ok(a) => a,
        Err(_) => panic!("Incorrect image file"),
    };

    let (mut width, mut height) = img.dimensions();

    println!("Successfully loaded image!");
    println!("Image size: {} x {}", width, height);

    if width > 200 {
        println!("Image too big to show, Resizing!");
        img = img.resize(200, height, FilterType::Lanczos3);

        let d = img.dimensions();
        width = d.0;
        height = d.1; 
        println!("Resized image size: {} x {}", width, height);
    }

    for y in 0..height {
        for x in 0..width {
        
            if x == 0 {
                println!("");
            }
            let p = img.get_pixel(x,y);
            //print!("{:?}",p );
            
            print!("{}", bright_to_char((p[0] as f32 + p[1] as f32 + p[2] as f32) / 3.0));
        }
    }
}

fn bright_to_char(p: f32) -> char {
    
    let xs = ['`', '^', '"', ',', ':', ';', 'I', 'l', '!', 'i', '~', '+', '_', '-', '?', ']', '[', '}', '{', '1', ')', '(', '|', '\\', '/', 't', 'f', 'j', 'r', 'x', 'n', 'u', 'v', 'c', 'z','X','Y', 'U', 'J', 'C', 'L', 'Q', '0', 'O', 'Z', 'm', 'w', 'q', 'p', 'd', 'b', 'k', 'h', 'a', 'o', '*', '#', 'M', 'W', '&', '8', '%', 'B', '@', '$'];

    xs[(64.0*p/254.0) as usize]
}