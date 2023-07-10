mod core;

use std::fs::{create_dir, create_dir_all, File, OpenOptions};
use std::{env, fs, io};
use std::ffi::OsStr;
use std::fmt::{Debug, Display, format, Formatter};
use std::io::Write;
use std::num::{IntErrorKind, ParseIntError};
use std::ops::{Rem, Shl, Shr};
use std::path::PathBuf;
use clap::{Parser, Arg, ArgAction, ArgMatches, Command, CommandFactory};
use clap::error::ErrorKind;
use image::{ColorType, DynamicImage, GenericImage, GenericImageView, GrayImage, ImageFormat, Luma, Rgba, RgbImage};
use image::codecs::pnm::ArbitraryTuplType;
use image::codecs::pnm::ArbitraryTuplType::Grayscale;
use image::ColorType::{Rgb8, Rgba8};
use image::DynamicImage::ImageRgba8;
use image::imageops::FilterType;
use regex::Regex;
use crate::core::background::Background;
use crate::core::bitmap::Bitmap;
use crate::core::color::Color;
use crate::core::config::Cli;
use crate::core::img2bm::img2bm;
use crate::core::string_util::StringUtil;

use image::codecs::gif::{GifDecoder, GifEncoder};
use image::{ImageDecoder, AnimationDecoder};
use crate::core::path_ext::PathExt;


const ARG_THRESHOLD: &str = "threshold";
const ARG_PREVIEW: &str = "preview";
const ARG_REMOVE_BACKGROUND: &str = "remove_background_color";

const DOT: char = '.';
const SLASH: char = '/';

const EXT_PICTURE: [&str; 3] = ["png", "jpg", "jpeg"];
const EXT_BM: &str = "bm";
const EXT_PNG: &str = "png";
const EXT_GIF: &str = "gif";

pub const TARGET_WIDTH: u8 = 128;

fn main() {
    let cli = Cli::parse();

    let make_background_visible = match cli.background {
        None => false,
        Some(Background::Visible) => true,
        Some(Background::Invisible) => false,
    };

    let path_src = cli.path.to_string();
    let path_name = cli.path.get_path_name();
    let input_ext = cli.path.get_ext().to_lowercase();
    let preview_path_name = format!("{}_preview", cli.path.get_path_name());

    if EXT_PICTURE.contains(&input_ext.as_str()) {
        let image = image::open(path_src).unwrap().to_rgba8();
        let bitmap = img2bm(&image, cli.height, cli.inverse, make_background_visible, &cli.threshold);

        let path_bm = format!("{}.{}", path_name, EXT_BM);
        let mut file_dst = File::create(path_bm).unwrap();
        file_dst.write_all(bitmap.bytes.as_slice()).unwrap();

        if cli.preview {
            let preview = bm2preview(&bitmap);
            let preview_path = format!("{}.{}", preview_path_name, EXT_PNG);
            save_preview(&preview, preview_path.as_str());
        }
    } else if input_ext.as_str() == EXT_GIF {
        let mut preview_frames = Vec::<GrayImage>::new();
        let file = File::open(path_src).unwrap();
        let mut decoder = GifDecoder::new(file).unwrap();
        let mut count = 0u32;
        for frame in decoder.into_frames() {
            // todo use rayon
            let image = frame.unwrap().buffer().to_owned();
            let bitmap = img2bm(&image, cli.height, cli.inverse, make_background_visible, &cli.threshold);

            if cli.preview {
                preview_frames.push(bm2preview(&bitmap));
            }

            create_dir_all(path_name.as_str()).unwrap();
            let path_dst = format!("{}/frame_{}.{}", path_name, count, EXT_BM);
            let mut file_dst = File::create(path_dst).unwrap();
            file_dst.write_all(bitmap.bytes.as_slice()).unwrap();
            count += 1;
        }
        if cli.preview {
            let preview_path = format!("{}.{}", preview_path_name, EXT_GIF);
            preview_frames;
        }
    }
}

fn bm2preview(bitmap: &Bitmap) -> GrayImage {
    let width = bitmap.width as u32;
    let height = bitmap.height as u32;
    let mut image = GrayImage::new(width, height);
    for y in 0..height {
        for x in 0..width {
            let index = width * y + x;
            // +1 because of the first byte is extra 0x00
            let byte = bitmap.bytes.get(index as usize / 8 + 1).unwrap();
            let bit = byte.shr(index % 8) % 2 == 1;
            if !bit {
                image.put_pixel(x, y, Luma([255u8]));
            }
        }
    }
    return image;
}

fn save_preview(img: &GrayImage, name: &str) {
    image::save_buffer_with_format(
        name,
        img,
        img.width(),
        img.height(),
        ColorType::L8,
        ImageFormat::Png,
    ).unwrap();
}

// unused:

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
    return line;
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
                    if dx == 0 && dy == 0 { continue; }
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

fn remove_background(image: &mut DynamicImage, color: Color, to_visible: bool) {
    let target: u8 = if to_visible { 0 } else { 255 };
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
    return color_int;
}
