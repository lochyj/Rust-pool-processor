
/*
 *   Grayscale converter
 */

fn img_to_gray_scale(img: &image::DynamicImage) -> image::GrayImage {
    return image::imageops::grayscale(img);
}