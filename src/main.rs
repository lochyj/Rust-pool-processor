include!("./util.rs");

use image::{ImageBuffer, Luma};

fn main() {
    let img = image::open("src/images/formattedTable.jpg").unwrap();

    let gray_scale = img_to_gray_scale(&img);

    let mut new_image: ImageBuffer<_, Vec<_>> = ImageBuffer::new(gray_scale.width(), gray_scale.height());

    for (x, y, pixel_luminocity) in gray_scale.enumerate_pixels() {
        let white_threshold: u8 = 210;
        if pixel_luminocity[0] > white_threshold {
            new_image.put_pixel(x, y, image::Luma([255u8]));
        }
    }

    let ball_locations = return_ball_locations(&new_image);

    println!("{}", ball_locations.len());

    new_image.save("src/images/grayScale.jpg").unwrap();
}

fn return_ball_locations(img: &ImageBuffer<Luma<u8>, Vec<u8>>) -> Vec<(u32, u32)> {
    let mut ball_locations: Vec<(u32, u32)> = Vec::new();

    for (x, y, pixel_luminocity) in img.enumerate_pixels() {
        let white: u8 = 255;
        if pixel_luminocity[0] < white {
            ball_locations.push((x, y));
        }
    }

    return ball_locations;
}



