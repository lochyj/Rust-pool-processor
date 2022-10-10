
/*
 *   Grayscale converter
 */

fn img_to_gray_scale(img: &image::DynamicImage) -> image::GrayImage {
    return image::imageops::grayscale(img);
}

/*
 * Returns the heatmap vec of the image
 */

fn return_heatmap(img: &image::ImageBuffer<image::Luma<u8>, Vec<u8>>) -> Vec<(u32, u32, i32)> {
    // Heatmap vector
    let mut heat_image: Vec<(/* x -->*/ u32, /* y -->*/ u32, /* heat -->*/ i32)> = Vec::new();

    // Versions of white
    let white: u8 = 255;
    let luma_white: image::Luma<u8> = image::Luma([255u8]);

    // Debugging
    println!("Starting scan for {} pixels", img.enumerate_pixels().len());

    // X <-- dunno how to convert from i32 to u32 safely
    let x_scan: i32 = 50;
    let x_scan_iter: u32 = 50;

    // Y <-- dunno how to convert from i32 to u32 safely
    let y_scan: i32 = 50;
    let y_scan_iter: u32 = 50;

    // Iterate over each pixel in the supplied image
    for (x, y, pixel_luminocity) in img.enumerate_pixels() {

        // Check if the pixel is white
        if pixel_luminocity[0] == white {

            // Define the white num for the number of white pixel surrounding the current pixel
            let mut white_num: i32 = 0;
            
            // Scan for the white pixels on the x axis
            for i in 0..x_scan_iter {
                // Scan for the white pixels on the y axis
                for j in 0..y_scan_iter {
                    // Ensure the location is on the screen to prevent panic errors
                    if (x>i && y >j) &&  (x<img.width()-i && y<img.height()-j) {
                        // Above
                        if img.get_pixel(x+i, y+j) == &luma_white {
                            white_num += 1;
                        }
                        // Below
                        if img.get_pixel(x-i, y-j) == &luma_white {
                            white_num += 1;
                        }
                    } 
                }
                
                // Push the heatmap location to the vector
                heat_image.push((x, y, (white_num * 4 / (x_scan + y_scan))));
                
            }

        }
    }

    // Debugging
    println!("Scan returned");

    return heat_image;
}

fn find_ball_from_heat_map(img: &image::ImageBuffer<image::Luma<u8>, Vec<u8>>) -> Vec<(u32, u32)> {

    // Ball locations vec
    let mut ball_locations: Vec<(u32, u32)> = Vec::new();

    let mut white_locations: Vec<(u32, u32)> = Vec::new();

    // utilise tensorflow to find the ball


    println!("Found {} balls", white_locations.len());

    // Change to ball_locations
    return white_locations;
}