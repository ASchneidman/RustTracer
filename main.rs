extern crate image;

use image::{ImageBuffer, RgbImage};



fn main() {
    let dim_x = 512;
    let dim_y = 512;
    // Construct a new RGB ImageBuffer with the specified width and height.
    let mut img: RgbImage = ImageBuffer::new(512, 512);


    for x in 0..dim_x {
        for y in 0..dim_y {
            let pixel = img.get_pixel_mut(x, y);
            *pixel = image::Rgb([(x % 255) as u8, (y % 255) as u8, 50 as u8])
        }
    }

    img.save("output.png").unwrap()
}