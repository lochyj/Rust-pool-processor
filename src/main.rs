mod util;

include!("./util.rs");

use image::{ImageBuffer, Luma};



fn main() {
    // Image file location
    /* Old:
    * src/images/formattedTable.jpg
    */
    let image_loc = "src/images/setupFormattedTable.jpg";

    // Load the image from file
    let img = image::open(image_loc).unwrap();
    // Generate the grayscale image of var img
    let gray_scale = img_to_gray_scale(&img);

    // Define heat_map image and the new_image
    let mut heat_image: ImageBuffer<_, Vec<_>> = ImageBuffer::new(gray_scale.width(), gray_scale.height());
    let mut new_image: ImageBuffer<_, Vec<_>> = ImageBuffer::new(gray_scale.width(), gray_scale.height());

    for (x, y, pixel_luminocity) in gray_scale.enumerate_pixels() {
        // new_image is the same as gray_scale but with only the bright pixels
        let white_threshold: u8 = 190;
        if pixel_luminocity[0] > white_threshold {
            new_image.put_pixel(x, y, image::Luma([255u8]));
            
        }
        // Heatmap initialization
        heat_image.put_pixel(x, y, image::Luma([0u8]));
    }

    println!("Created black and white image");

    let ball_locations = return_heatmap(&new_image);

    println!("{}", ball_locations.len());

    for i in 0..ball_locations.len() {
        heat_image.put_pixel(ball_locations[i].0, ball_locations[i].1, image::Luma([(ball_locations[i].2).try_into().unwrap()]));
    }

    find_ball_from_heat_map(&heat_image);

    // Save the images to file
    // TODO: Remove compression from the file to remove the jagged edges of the final image and to make the preditction more accurate
    heat_image.save("src/images/heat_image.jpg").unwrap();
    new_image.save("src/images/grayScale.jpg").unwrap();
}





