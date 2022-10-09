
/*
 *   Grayscale converter
 */

fn img_to_gray_scale(img: &image::DynamicImage) -> image::GrayImage {
    return image::imageops::grayscale(img);
}

/*
 * Returns the heatmap of the image
 */

fn return_heatmap(img: &ImageBuffer<Luma<u8>, Vec<u8>>) -> Vec<(u32, u32, i32)> {
    let mut ball_locations: Vec<(u32, u32, i32)> = Vec::new();
    let white: u8 = 255;
    let luma_white: Luma<u8> = image::Luma([255u8]);

    println!("Starting scan");

    // X
    let x_scan: i32 = 50;
    let x_scan_iter: u32 = 50;

    // Y
    let y_scan: i32 = 50;
    let y_scan_iter: u32 = 50;

    for (x, y, pixel_luminocity) in img.enumerate_pixels() {

        if pixel_luminocity[0] == white {

            let mut white_num: i32 = 0;
            
            for i in 0..x_scan_iter {
                for j in 0..y_scan_iter {
                    if (x>i && y >j) &&  (x<img.width()-i && y<img.height()-j) {
                        if img.get_pixel(x+i, y+j) == &luma_white {
                            white_num += 1;
                        }
                        if img.get_pixel(x-i, y-j) == &luma_white {
                            white_num += 1;
                        }
                    } 
                }
                
                ball_locations.push((x, y, (white_num* 5 / (x_scan + y_scan))));
                
            }

        }
    }

    println!("Scan returned");

    return ball_locations;
}