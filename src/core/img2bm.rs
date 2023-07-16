use std::fs::FileType;
use std::ops::Shl;
use image::{DynamicImage, RgbaImage};
use image::imageops::FilterType;
use crate::core::bitmap::Bitmap;
use crate::core::params::Params;
use crate::core::scale_type::ScaleType;
use crate::core::threshold::RangeInc;


const CHANNEL_MAX: f32 = 255.0;
const BYTE_MAX: u8 = 255;

pub fn img2bm(
    image: &RgbaImage,
    params: &Params,
) -> Bitmap {
    let resized = scale_to(image, params);
    let resized_width = resized.width() as i32;
    let resized_height = resized.height() as i32;
    let x_offset = (resized_width - params.width as i32) / 2;
    let output_height = match params.scale_type {
        ScaleType::FillCenter | ScaleType::FitBottom => resized_height,
        ScaleType::FitCenter => resized_height + (params.height as i32 - resized_height) / 2,
    };
    let mut chunk = 0u8;
    let mut current_bit = 0u8;
    let mut bytes = Vec::<u8>::new();
    bytes.push(0x00);
    let mut lum_sum: f32 = 0.0;
    for y in 0..output_height {
        for x in 0..params.width {
            let src_x = x as i32 + x_offset;
            let mut make_visible = false;
            if src_x < 0 || src_x >= resized_width || y >= resized_height {
                make_visible = params.background_visible;
            } else {
                make_visible = is_pixel_black(&resized, src_x as u32, y as u32, &params.threshold);
            }
            if params.inverse {
                make_visible = !make_visible;
            }
            if make_visible {
                chunk += 1u8.shl(current_bit);
            }
            current_bit += 1;
            if current_bit == 8 {
                bytes.push(chunk);
                current_bit = 0;
                chunk = 0;
            }
        }
        if current_bit != 0 {
            bytes.push(chunk);
            current_bit = 0;
            chunk = 0;
        }
    }

    Bitmap {
        width: params.width,
        height: output_height as u8,
        bytes,
    }
}

fn is_pixel_black(image: &RgbaImage, x: u32, y: u32, threshold: &RangeInc) -> bool {
    let pixel = image.get_pixel(x, y).0;
    let r = pixel[0] as f32 / CHANNEL_MAX;
    let g = pixel[1] as f32 / CHANNEL_MAX;
    let b = pixel[2] as f32 / CHANNEL_MAX;
    let a = pixel[3] as f32 / CHANNEL_MAX;

    let luminance = (0.299 * r * r + 0.587 * g * g + 0.114 * b * b).sqrt() * a;
    if !threshold.is_max() {
        if luminance > threshold.end() {
            return false
        } else if luminance < threshold.start() {
            return true
        }
    }
    let rnd = rand::random::<f32>();
    let threshold = threshold.start() + threshold.size() * rnd;
    return threshold > luminance;
}

fn scale_to(image: &RgbaImage, params: &Params) -> RgbaImage {
    let dynamic = DynamicImage::from(image.clone());
    let resized = match params.scale_type {
        ScaleType::FillCenter => dynamic.resize_to_fill(params.width as u32, params.height as u32, FilterType::Nearest),
        ScaleType::FitCenter | ScaleType::FitBottom => dynamic.resize(params.width as u32, params.height as u32, FilterType::Nearest),
    };
    return resized.to_rgba8();
}

// unused:

fn is_pixel_black2(image: &RgbaImage, x: u32, y: u32, threshold: f32, sum: f32) -> (bool, f32) {
    let pixel = image.get_pixel(x, y).0;
    let r = pixel[0] as f32 / CHANNEL_MAX;
    let g = pixel[1] as f32 / CHANNEL_MAX;
    let b = pixel[2] as f32 / CHANNEL_MAX;

    let luminance = (0.299 * r * r + 0.587 * g * g + 0.114 * b * b).sqrt();
    let sum = sum + luminance * threshold * 2.0;
    if sum >= 1.0 {
        (false, sum - 1.0)
    } else {
        (true, sum)
    }
}

fn lum_sum(image: &RgbaImage, x: i32, y: i32) -> f32 {
    let width = image.width() as i32;
    let height = image.height() as i32;
    let mut sum = 0.0;
    let mut count: f32 = 0.0;
    for dy in -1..1 {
        let y = y + dy;
        for dx in -1..1 {
            let x = x + dx;
            if x < 0 || x >= width || y < 0 || y >= height {
                continue
            }
            let pixel = image.get_pixel(x as u32, y as u32).0;
            let r = pixel[0] as f32 / CHANNEL_MAX;
            let g = pixel[1] as f32 / CHANNEL_MAX;
            let b = pixel[2] as f32 / CHANNEL_MAX;
            sum += (0.299 * r * r + 0.587 * g * g + 0.114 * b * b).sqrt();
            count += 1.0;
        }
    }
    sum / count
}


