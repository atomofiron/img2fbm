use std::ops::Range;
use image::{DynamicImage, GrayImage, RgbaImage};
use image::imageops::FilterType;
use crate::core::bitmap::Bitmap;
use crate::core::params::Params;
use crate::core::scale_type::ScaleType;
use crate::core::threshold::Threshold;
use crate::ext::range_ext::for_each;


const MAX_RADIUS: f32 = 4.0;

pub fn img2bm(image: &RgbaImage, params: &Params) -> Bitmap {
    let resized = scale_to(image, params);
    let resized_height = resized.height() as i32;
    let output_height = match params.scale_type {
        ScaleType::FillCenter | ScaleType::FitBottom => resized_height,
        ScaleType::FitCenter => resized_height + (params.height as i32 - resized_height) / 2,
    };
    let mut bitmap = Bitmap::new(params.width, output_height as u8);
    process_dark(params, &resized, &mut bitmap);
    if !params.threshold.is_empty() {
        process(&params.threshold, &resized, &mut bitmap, 0.0..0.1);
        process(&params.threshold, &resized, &mut bitmap, 0.1..0.2);
        process(&params.threshold, &resized, &mut bitmap, 0.2..0.4);
        process(&params.threshold, &resized, &mut bitmap, 0.4..0.65);
        process(&params.threshold, &resized, &mut bitmap, 0.65..0.1);
    }
    if params.background_visible {
        process_outside_and_inverting(&resized, &mut bitmap, params.background_visible);
    }
    if params.inverse {
        bitmap.invert();
    }
    return bitmap;
}

fn process_dark(params: &Params, resized: &GrayImage, bitmap: &mut Bitmap) {
    for_each_luminance(resized, bitmap, |bitmap, x, y, outside, luminance| {
        if !outside && luminance < params.threshold.dark {
            bitmap.set(x, y);
        }
    });
}

fn process_outside_and_inverting(
    resized: &GrayImage,
    bitmap: &mut Bitmap,
    background_visible: bool,
) {
    for_each_luminance(resized, bitmap, |bitmap, x, y, outside, luminance| {
        if background_visible && outside {
            bitmap.set(x, y);
        }
    });
}

fn process(threshold: &Threshold, resized: &GrayImage, bitmap: &mut Bitmap, range: Range<f32>) {
    for_each_luminance(resized, bitmap, |bitmap, x, y, outside, luminance| {
        if !outside && threshold.contains(luminance) {
            let luminance = (luminance - threshold.dark) / threshold.size();
            if !range.contains(&luminance) {
                return;
            }
            let already_bit_nearby = find_in_radius(bitmap, luminance, x as i32, y as i32);
            if !already_bit_nearby {
                bitmap.set(x, y);
            }
        }
    });
}

fn for_each_luminance<F>(
    image: &GrayImage,
    bitmap: &mut Bitmap,
    mut action: F,
) where F: FnMut(&mut Bitmap, u32, u32, bool/*is outside*/, f32) {
    let width = bitmap.width;
    let height = bitmap.height;
    for_each(0..height as u32, 0..width as u32, |x,y| {
        if bitmap.get(x, y) { return; }
        let bitmap_width = bitmap.width as u32;
        let dif = bitmap_width - image.width();
        let left = dif / 2;
        let right = bitmap_width - left - dif % 2;
        if x < left || x >= right {
            action(bitmap, x, y, true, 0.0);
            return;
        }
        let luminance = image.get_pixel(x - left, y).0[0] as f32 / 255.0;
        action(bitmap, x, y, false, luminance);
    });
}

fn scale_to(image: &RgbaImage, params: &Params) -> GrayImage {
    let dynamic = DynamicImage::from(image.clone());
    let resized = match params.scale_type {
        ScaleType::FillCenter => dynamic.resize_to_fill(params.width as u32, params.height as u32, FilterType::Nearest),
        ScaleType::FitCenter | ScaleType::FitBottom => dynamic.resize(params.width as u32, params.height as u32, FilterType::Nearest),
    };
    return resized.to_luma8();
}

pub fn find_in_radius(bitmap: &Bitmap, luminance: f32, x: i32, y: i32) -> bool {
    let radius = luminance * MAX_RADIUS;
    let half = radius as i32;
    for dy in -half..half {
        for dx in -half..half {
            let x = x + dx;
            let y = y + dy;
            if x < 0 || y < 0 || x as u8 >= bitmap.width || y as u8 >= bitmap.height {
                continue;
            } else if !bitmap.get(x as u32, y as u32) {
                continue;
            } else if radius >= ((dx*dx + dy*dy) as f32).sqrt() {
                return true;
            }
        }
    }
    return false;
}
