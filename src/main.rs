include!("./util.rs");

use image::{ImageBuffer, Luma, GenericImageView};

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

    let balls = ImageBuffer::new(new_image.width(), new_image.height());

    let ball_locations = return_ball_locations(&new_image);

    for i in 0..ball_locations.len() {
        println!("Ball {} / {} is at: {:?}", i,ball_locations.len(), ball_locations[i]);
        if ball_locations[i] == (1627, 1372) {
            break;
        }
    }
    println!("{}", ball_locations.len());

    new_image.save("src/images/grayScale.jpg").unwrap();
}

fn return_ball_locations(img: &ImageBuffer<Luma<u8>, Vec<u8>>) -> Vec<(u32, u32)> {
    let mut ball_locations: Vec<(u32, u32)> = Vec::new();
    let white: u8 = 255;
    let luma_white: Luma<u8> = image::Luma([255u8]);

    for (x, y, pixel_luminocity) in img.enumerate_pixels() {
        
        let mut white_loc: Vec<(u32, u32)> = Vec::new();

        if pixel_luminocity[0] < white {

            if (x>5 && y>5) && (x<img.width()-5 && y<img.height()-5) {
                let mut white_is:bool = false;
                for i in 0..5 {
                    for j in 0..5 {
                        if img.get_pixel(x+i, y+j) == &luma_white {
                            white_is = true;
                        }
                    }
                    
                }
                if white_is == true {
                    //white_loc.push((x, y));
                    ball_locations.push((x, y));
                }
                
            }
        }
    }

    return ball_locations;
}



