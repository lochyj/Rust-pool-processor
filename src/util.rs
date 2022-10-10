
/*
 *   Grayscale converter
 */

fn img_to_gray_scale(img: &image::DynamicImage) -> image::GrayImage {
    return image::imageops::grayscale(img);
}

/*
 * Returns the heatmap vec of the image
 */

fn return_heatmap(img: &ImageBuffer<Luma<u8>, Vec<u8>>) -> Vec<(u32, u32, i32)> {
    // Heatmap vector
    let mut heat_image: Vec<(/* x -->*/ u32, /* y -->*/ u32, /* heat -->*/ i32)> = Vec::new();

    // Versions of white
    let white: u8 = 255;
    let luma_white: Luma<u8> = image::Luma([255u8]);

    // Debugging
    println!("Starting scan for {} pixels", img.enumerate_pixels().len());

    // X
    let x_scan: i32 = 50;
    let x_scan_iter: u32 = 50;

    // Y
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
                heat_image.push((x, y, (white_num* 5 / (x_scan + y_scan))));
                
            }

        }
    }

    // Debugging
    println!("Scan returned");

    return heat_image;
}

fn find_ball_from_heat_map(img: &ImageBuffer<Luma<u8>, Vec<u8>>) -> Vec<(u32, u32)> {

    // Ball locations vec
    let mut ball_locations: Vec<(u32, u32)> = Vec::new();

    let mut white_locations: Vec<(u32, u32)> = Vec::new();

    // Iterate over the pixels to make a vector of all of the white > Luma(100u8) pixels
    for (x, y, pixel_luminocity) in img.enumerate_pixels() {
        if pixel_luminocity[0] > 100 {
            white_locations.push((x, y));
        }
    }

    let ball_threshold: u32 = 100;

    // iterate over the white locations and group them into ball locations based on the ball_threshold
    for i in 0..white_locations.len() {
        let mut ball_location: (u32, u32) = (0, 0);
        let mut ball_location_count: u32 = 0;
        for j in 0..white_locations.len() {
            // Ensure the location is on the screen to prevent panic errors
            if (white_locations[i].0 > white_locations[j].0 - ball_threshold && white_locations[i].1 > white_locations[j].1 - ball_threshold) && (white_locations[i].0 < white_locations[j].0 + ball_threshold && white_locations[i].1 < white_locations[j].1 + ball_threshold) {
                ball_location.0 += white_locations[j].0;
                ball_location.1 += white_locations[j].1;
                ball_location_count += 1;
            }
        }
        ball_location.0 /= ball_location_count;
        ball_location.1 /= ball_location_count;
        ball_locations.push(ball_location);
    }

    println!("Found {} balls", ball_locations.len());

    // Change to ball_locations
    return white_locations;
}