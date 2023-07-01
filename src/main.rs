mod core;

use std::fs::{File, OpenOptions};
use std::{env, fs, io};
use std::ffi::OsStr;
use std::fmt::{Debug, Display, Formatter};
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


const ARG_THRESHOLD: &str = "threshold";
const ARG_PREVIEW: &str = "preview";
const ARG_REMOVE_BACKGROUND: &str = "remove_background_color";

const DOT: char = '.';
const SLASH: char = '/';

const EXT_PICTURE: &str = r"(.png|.jpg|.jpeg|.jpeg)$";
const EXT_BM: &str = ".bm";

pub const TARGET_WIDTH: u8 = 128;

fn main() {
    let cli = Cli::parse();

    let make_background_visible = match cli.background {
        None => false,
        Some(Background::Visible) => true,
        Some(Background::Invisible) => false,
    };

    let path_src = cli.path.into_os_string().into_string().unwrap();
    let mut path_dst = path_src.clone();
    let last_dot = path_dst.signed_last_index_of(DOT);
    let last_slash = path_dst.signed_last_index_of(SLASH);
    if last_dot > last_slash + 1 {
        path_dst = format!("{}{}", path_dst.substring(0..(last_dot as usize)), EXT_BM);
    }
    let ext_picture = Regex::new(EXT_PICTURE).unwrap();
    if ext_picture.is_match(path_src.as_str()) {
        let image = image::open(path_src).unwrap().to_rgb8();
        let bitmap = img2bm(image, cli.height, cli.inverse, make_background_visible);

        let mut file_dst = File::create(path_dst).unwrap();
        file_dst.write_all(bitmap.bytes.as_slice()).unwrap();
    }
}

fn bm2preview(bitmap: Bitmap) {
    /*if let Some(preview) = &mut preview {
        let value = if make_visible { 0u8 } else { 255u8 };
        preview.put_pixel(x, y, Luma([value]))
    }*/
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

fn graphic(matches: &ArgMatches, mut image: DynamicImage, height: u32, background_visible: bool) {
    let bg_color = matches.get_one::<String>(ARG_REMOVE_BACKGROUND)
        .map(|it| Color::parse(color_to_u32(it.as_str())));
    let threshold = matches.get_one::<String>(ARG_THRESHOLD).unwrap()
        .parse::<u32>().unwrap()
        .max(0).min(100)
        as f32 / 100.0;

    let with_preview = matches.get_count(ARG_PREVIEW) > 0;

    if let Some(color) = bg_color {
        remove_background(&mut image, color, background_visible);
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
        true => Some(DynamicImage::ImageLuma8(GrayImage::new(TARGET_WIDTH as u32, height)).to_luma8()),
        _ => None
    };
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
    return color_int
}
