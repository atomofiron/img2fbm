mod core;

use std::fs::{File, OpenOptions};
use std::{env, fs, io};
use std::ffi::OsStr;
use std::fmt::{Debug, Display, Formatter};
use std::io::Write;
use std::num::{IntErrorKind, ParseIntError};
use std::ops::{Range, RangeInclusive, RangeTo, Rem, Shl, Shr};
use std::path::PathBuf;
use clap::{Parser, Arg, ArgAction, ArgMatches, Command, CommandFactory};
use clap::error::ErrorKind;
use image::{ColorType, DynamicImage, GenericImage, GenericImageView, GrayImage, ImageFormat, Luma, Rgba, RgbImage};
use image::codecs::pnm::ArbitraryTuplType;
use image::codecs::pnm::ArbitraryTuplType::Grayscale;
use image::ColorType::{Rgb8, Rgba8};
use image::DynamicImage::ImageRgba8;
use image::imageops::FilterType;
use crate::core::config::Cli;


const ARG_INPUT_PATH: &str = "input_path";
const ARG_THRESHOLD: &str = "threshold";
const ARG_HEIGHT: &str = "height";
const ARG_PREVIEW: &str = "preview";
const ARG_INVERSE: &str = "inverse";
const ARG_BACKGROUND: &str = "background";
const ARG_REMOVE_BACKGROUND: &str = "remove_background_color";

const BACKGROUND_WHITE: &str = "white";
const BACKGROUND_BLACK: &str = "black";

const EXT_PNG: &str = ".png";
const EXT_BM: &str = ".bm";

const TARGET_WIDTH: u32 = 128;
const MAX_HEIGHT: u32 = 64;
const MAX_HEIGHT_STR: &str = "64";

const CHANNEL_MAX: f32 = 255.0;
const BYTE_MAX: u8 = 255;
const BYTE_LIMIT: u16 = 256;

fn main() {
    let cli = Cli::parse();

    let background = matches.get_one::<String>(ARG_BACKGROUND).unwrap();
    let background_black = match background.as_str() {
        BACKGROUND_BLACK => true,
        BACKGROUND_WHITE => false,
        _ => panic!("Unknown background value {}", background),
    };

    let path_src = matches.get_one::<String>(ARG_INPUT_PATH).unwrap().to_string();
    let mut path_dst = path_src.clone();
    if path_dst.ends_with(EXT_PNG) {
        path_dst = format!("{}{}", path_dst.substring(0..(path_dst.len() - EXT_PNG.len())), EXT_BM);
    }

    let resized = image::open(path_src).unwrap().to_rgb8();

    let x_offset = (resized.width() as i32 - TARGET_WIDTH as i32) / 2;
    let image_width = resized.width() as i32;
    let mut chunk = 0u8;
    let mut current_bit = 0u8;
    let mut bytes = Vec::<u8>::new();
    bytes.push(0x00);
    let mut lum_sum: f32 = 0.0;
    for y in 0..cli.height {
        for x in 0..TARGET_WIDTH {
            let src_x = x as i32 + x_offset;
            let mut make_black = false;
            if src_x < 0 || src_x >= image_width {
                make_black = background_black;
            } else {
                //(make_black, lum_sum) = is_pixel_black(&resized, src_x as u32, y, 0.5, lum_sum);
                make_black = is_pixel_black(&resized, src_x as u32, y);
            }
            if cli.inverse {
                make_black = !make_black;
            }
            if make_black {
                chunk += 1u8.shl(current_bit);
            }
            /*if let Some(preview) = &mut preview {
                let value = if make_black { 0u8 } else { 255u8 };
                preview.put_pixel(x, y, Luma([value]))
            }*/
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

    /*if let Some(preview) = preview {
        image::save_buffer_with_format(
            "preview.png",
            &preview,
            preview.width(),
            preview.height(),
            ColorType::L8,
            ImageFormat::Png,
        ).unwrap();
    }*/

    let mut file_dst = File::create(path_dst).unwrap();
    file_dst.write_all(bytes.as_slice()).unwrap();
}

fn save_preview(img: &RgbImage, name: &str) {
    image::save_buffer_with_format(
        name,
        img,
        img.width(),
        img.height(),
        ColorType::Rgb8,
        ImageFormat::Png,
    ).unwrap();
}

fn graphic(matches: &ArgMatches, mut image: DynamicImage, height: u32, background_black: bool) {
    let bg_color = matches.get_one::<String>(ARG_REMOVE_BACKGROUND)
        .map(|it| Color::parse(color_to_u32(it.as_str())));
    let threshold = matches.get_one::<String>(ARG_THRESHOLD).unwrap()
        .parse::<u32>().unwrap()
        .max(0).min(100)
        as f32 / 100.0;

    let with_preview = matches.get_count(ARG_PREVIEW) > 0;

    if let Some(color) = bg_color {
        remove_background(&mut image, color, background_black);
        if with_preview {
            save_preview(&image.to_rgb8(), "removed.png");
        }
    }
    disagreement(&mut image);
    if with_preview {
        save_preview(&image.to_rgb8(), "disagreement.png");
    }

    let width = height * image.width() / image.height();
    let mut resized = if image.height() != height {
        image.resize(width, height, FilterType::Triangle)
    } else { image };

    disagreement(&mut resized);
    let resized = resized.to_rgb8();
    if with_preview {
        save_preview(&resized, "resized.png");
    }

    let mut preview = match with_preview {
        true => Some(DynamicImage::ImageLuma8(GrayImage::new(TARGET_WIDTH, height)).to_luma8()),
        _ => None
    };
}

struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    pub fn parse(value: u32) -> Color {
        let limit = BYTE_LIMIT as u32;
        Color {
            r: (value / limit.pow(2) % limit) as u8,
            g: (value / limit % limit.pow(2)) as u8,
            b: (value % limit.pow(3)) as u8,
        }
    }
}

fn try_get_pixel<T>(image: &T, x: i32, y: i32) -> Option<T::Pixel>
    where T: GenericImage
{
    match () {
        _ if x < 0 => None,
        _ if y < 0 => None,
        _ if x >= image.width() as i32 => None,
        _ if y >= image.height() as i32 => None,
        _ => Some(image.get_pixel(x as u32, y as u32))
    }
}

fn for_each<T, F>(image: &T, action: F)
    where
        T: GenericImage,
        F: Fn(u32, u32, T::Pixel) /*+ Copy*/,
{
    for y in 0..image.height() {
        for x in 0..image.width() {
            action(x, y, image.get_pixel(x, y));
        }
    }
}

fn for_each_mut<T, F>(image: &mut T, action: F)
    where
        T: GenericImage,
        F: Fn(&mut T, T::Pixel) -> Option<T::Pixel> /*+ Copy*/,
{
    for y in 0..image.height() {
        for x in 0..image.width() {
            let new = action(image, image.get_pixel(x, y));
            if let Some(pixel) = new {
                image.put_pixel(x, y, pixel)
            }
        }
    }
}

fn abs_dif(first: u8, second: u8) -> u8 {
    let mut dif = first as i16 - second as i16;
    dif *= dif.signum();
    dif as u8
}

fn copy_line(image: &DynamicImage, y: u32) -> Vec<[u8; 4]> {
    let mut line = Vec::new();
    for x in 0..image.width() {
        line.push(image.get_pixel(x, y).0)
    }
    return line
}

fn disagreement(image: &mut DynamicImage) {
    let src = image.clone();
    for y in 0..image.height() {
        for x in 0..image.width() {
            let current = image.get_pixel(x, y).0;
            let mut dif_r: i32 = 0;
            let mut dif_g: i32 = 0;
            let mut dif_b: i32 = 0;
            for dy in -1..1 {
                let y = y as i32 + dy;
                for dx in -1..1 {
                    let x = x as i32 + dx;
                    if dx == 0 && dy == 0 { continue }
                    if let Some(pixel) = try_get_pixel(&src, x, y) {
                        let pixel = pixel.0;
                        dif_r += current[0] as i32 - pixel[0] as i32;
                        dif_g += current[1] as i32 - pixel[1] as i32;
                        dif_b += current[2] as i32 - pixel[2] as i32;
                    }
                }
            }
            let max_dif = dif_r.max(dif_g).max(dif_b);
            let new: [u8; 4] = [
                ((255 - current[0] as i32) * (max_dif / 255) + current[0] as i32) as u8,
                ((255 - current[1] as i32) * (max_dif / 255) + current[1] as i32) as u8,
                ((255 - current[2] as i32) * (max_dif / 255) + current[2] as i32) as u8,
                current[3],
            ];
            image.put_pixel(x, y, Rgba(new))
        }
    }
}

fn remove_background(image: &mut DynamicImage, color: Color, to_black: bool) {
    let target: u8 = if to_black { 0 } else { 255 };
    for_each_mut(image, |image, pixel| {
        let r = pixel[0];
        let g = pixel[1];
        let b = pixel[2];
        match () {
            _ if abs_dif(color.r, r) > 50 => None,
            _ if abs_dif(color.g, g) > 50 => None,
            _ if abs_dif(color.b, b) > 50 => None,
            _ if abs_dif(abs_dif(color.r, color.g), abs_dif(r, g)) > 30 => None,
            _ if abs_dif(abs_dif(color.g, color.b), abs_dif(g, b)) > 30 => None,
            _ => Some(Rgba([target, target, target, pixel.0[3]]))
        }
    });
}

fn color_to_u32(color: &str) -> u32 {
    if color.len() != 6 && color.len() != 8 {
        panic!("Color must contains 6 or 8 0..f-chars ({})", color);
    }
    let mut color_int = u32::from_str_radix(color, 16).unwrap();
    if color.len() == 6 {
        color_int += 0xff000000;
    }
    return color_int
}

fn is_pixel_black(image: &RgbImage, x: u32, y: u32) -> bool {
    let pixel = image.get_pixel(x, y).0;
    let r = pixel[0] as f32 / CHANNEL_MAX;
    let g = pixel[1] as f32 / CHANNEL_MAX;
    let b = pixel[2] as f32 / CHANNEL_MAX;

    let luminance = (0.299 * r * r + 0.587 * g * g + 0.114 * b * b).sqrt();
    return luminance < 0.5
}

fn is_pixel_black2(image: &RgbImage, x: u32, y: u32, threshold: f32, sum: f32) -> (bool, f32) {
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

fn lum_sum(image: &RgbImage, x: i32, y: i32) -> f32 {
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



trait StringUtil {
    fn substring(&self, range: Range<usize>) -> Self;
}

impl StringUtil for String {
    fn substring(&self, range: Range<usize>) -> Self {
        String::from(&self[range])
    }
}
