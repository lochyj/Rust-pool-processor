use image::{ImageBuffer};

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

    new_image.save("src/images/grayScale.jpg").unwrap();
}

/*
 *   Grayscale converter
 */
fn img_to_gray_scale(img: &image::DynamicImage) -> image::GrayImage {
    return image::imageops::grayscale(img);
}