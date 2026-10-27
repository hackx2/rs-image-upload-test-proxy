// goal: sample_img & sample_video (maybe we should use bitmasking for the video options (720p & 480p) )
// sample_img is going to resize then save the img as webp - DONE AHHAAHAHAHAH
// sample_vid ~ we're going to probably use ffmpeg or something like that... um,, yeah!!! - NOT DONE :CC

use image::{DynamicImage, imageops::FilterType};

const SAMPLE_IMAGE_WIDTH: u32 = 850;
const SAMPLE_IMAGE_QUALITY: f32 = 80.0;

pub fn sample_image(bytes: &[u8]) -> Option<Vec<u8>> {
    let img = image::load_from_memory(bytes).ok()?;
    if img.width() <= SAMPLE_IMAGE_WIDTH {
        return None;
    }

    // uh
    let new_img = img.resize(SAMPLE_IMAGE_WIDTH, SAMPLE_IMAGE_WIDTH, FilterType::Lanczos3);

    let rgba = DynamicImage::ImageRgba8(new_img.to_rgba8());
    let encoded_image = webp::Encoder::from_image(&rgba)
        .ok()?
        .encode(SAMPLE_IMAGE_QUALITY);

    Some(encoded_image.to_vec())
}
