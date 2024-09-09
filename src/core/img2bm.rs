use std::ops::Range;
use image::{DynamicImage, GrayImage, RgbaImage};
use image::imageops::FilterType;
use crate::core::params::background::Background;
use crate::core::bitmap::Bitmap;
use crate::core::params::alignment::Alignment;
use crate::core::params::params::Params;
use crate::core::params::scale_type::ScaleType;
use crate::core::params::threshold::Threshold;
use crate::ext::range_ext::for_each;
use crate::ext::image_ext::Resizing;


const MAX_RADIUS: f32 = 4.0;

pub fn img2bm(image: &RgbaImage, params: &Params) -> Bitmap {
    let resized = resize(image, params).to_luma8();
    let mut bitmap = create_bitmap(&resized, params);
    if params.threshold.dark > 0.0 {
        process_dark(params, &resized, &mut bitmap);
    }
    if !params.threshold.is_empty() {
        // todo replace with sorted pixels
        process(&params.threshold, &resized, &mut bitmap, 0.0..0.1);
        process(&params.threshold, &resized, &mut bitmap, 0.1..0.2);
        process(&params.threshold, &resized, &mut bitmap, 0.2..0.4);
        process(&params.threshold, &resized, &mut bitmap, 0.4..0.65);
        process(&params.threshold, &resized, &mut bitmap, 0.65..0.1);
    }
    if params.background != Background::Invisible {
        process_outside_and_inverting(&resized, &mut bitmap, params.background);
    }
    if params.inverse {
        bitmap.invert();
    }
    return bitmap;
}

fn create_bitmap(image: &GrayImage, params: &Params) -> Bitmap {
    let mut dx = 0;
    if params.alignment != Alignment::Left {
        dx = image.width() as i32 - params.width as i32;
    }
    if params.alignment != Alignment::Right {
        dx = dx / 2 + dx % 2;
    }
    let mut dy = 0;
    if params.alignment != Alignment::Top {
        dy = image.height() as i32 - params.height as i32;
    }
    if params.alignment != Alignment::Bottom {
        dy = dy / 2 + dy % 2;
    }
    return Bitmap::new(params.width, params.height, dx, dy);
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
    background: Background,
) {
    let height_dif = bitmap.height as u32 - resized.height();
    let width_dif = bitmap.width as u32 - resized.width();
    // if the source width or height is odd and alignment is centered,
    // number of outside pixels is above=below+1 and on the left=on the right+1
    let top_edge = height_dif / 2 + height_dif % 2;
    let bottom_edge = bitmap.height as u32 - height_dif / 2;
    let left_edge = width_dif / 2 + width_dif % 2;
    let right_edge = bitmap.width as u32 - width_dif / 2;
    for_each_luminance(resized, bitmap, |bitmap, x, y, outside, /*luminance*/_| {
        match () {
            _ if !outside => (),
            _ if background == Background::Visible => bitmap.set(x, y),
            _ if background == Background::Start && (x < left_edge || y < top_edge) => bitmap.set(x, y),
            _ if background == Background::End && (x >= right_edge || y >= bottom_edge) => bitmap.set(x, y),
            _ => (),
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
) where F: FnMut(&mut Bitmap, u32, u32, /*outside:*/bool, /*luminance:*/f32) {
    let width = bitmap.width;
    let height = bitmap.height;
    for_each(0..height as u32, 0..width as u32, |x,y| {
        if bitmap.get(x, y) { return; }
        let src_x = bitmap.get_src_x(x);
        let src_y = bitmap.get_src_y(y);
        if src_x < 0 || src_x >= image.width() as i32 || src_y < 0 || src_y >= image.height() as i32 {
            action(bitmap, x, y, true, 0.0);
            return;
        }
        let luminance = image.get_pixel(src_x as u32, src_y as u32).0[0] as f32 / 255.0;
        action(bitmap, x, y, false, luminance);
    });
}

fn resize(image: &RgbaImage, params: &Params) -> DynamicImage {
    let dynamic = DynamicImage::from(image.clone());
    let fill = match params.scale_type {
        ScaleType::Fill => true,
        ScaleType::Fit => false,
    };
    return Resizing::resize(&dynamic, params.width as u32, params.height as u32, fill, FilterType::Nearest);
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
