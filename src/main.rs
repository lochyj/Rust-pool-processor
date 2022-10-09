include!("./util.rs");

use image::{ImageBuffer, Luma, GenericImageView};

fn main() {
    // Load the image from file
    let img = image::open("src/images/formattedTable.jpg").unwrap();
    // Generate the grayscale image of var img
    let gray_scale = img_to_gray_scale(&img);

    // Define heat_map image and the new_image
    let mut heat_image: ImageBuffer<_, Vec<_>> = ImageBuffer::new(gray_scale.width(), gray_scale.height());
    let mut new_image: ImageBuffer<_, Vec<_>> = ImageBuffer::new(gray_scale.width(), gray_scale.height());

    for (x, y, pixel_luminocity) in gray_scale.enumerate_pixels() {
        let white_threshold: u8 = 190;
        if pixel_luminocity[0] > white_threshold {
            new_image.put_pixel(x, y, image::Luma([255u8]));
            
        }
        let white_threshold2: u8 = 255;
        if pixel_luminocity[0] == white_threshold2 {
            heat_image.put_pixel(x, y, image::Luma([255u8]));
        }
    }

    println!("Created black and white image");

    let ball_locations = return_heatmap(&new_image);

    println!("{}", ball_locations.len());

    for i in 0..ball_locations.len() {
        heat_image.put_pixel(ball_locations[i].0, ball_locations[i].1, image::Luma([(ball_locations[i].2).try_into().unwrap()]));
    }

    heat_image.save("src/images/heat_image.jpg").unwrap();
    new_image.save("src/images/grayScale.jpg").unwrap();
}





