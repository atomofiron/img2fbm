mod core;
mod ext;

use std::fs::{create_dir_all, File, OpenOptions};
use std::{env, fs, io};
use std::collections::hash_map::DefaultHasher;
use std::fmt::{Debug, Display, format, Formatter};
use std::hash::{Hash, Hasher};
use std::io::{Read, Write};
use std::num::{IntErrorKind, ParseIntError};
use std::ops::Shr;
use std::path::Path;
use clap::{Parser, Arg, ArgAction, ArgMatches, Command, CommandFactory};
use clap::error::ErrorKind;
use image::{ColorType, Delay, DynamicImage, Frame, GenericImage, GenericImageView, GrayImage, ImageFormat, Luma, Rgba};
use image::codecs::pnm::ArbitraryTuplType::Grayscale;
use crate::core::background::Background;
use crate::core::bitmap::Bitmap;
use crate::core::color::Color;
use crate::core::config::Cli;
use crate::core::img2bm::img2bm;

use image::codecs::gif::{GifDecoder, GifEncoder, Repeat};
use image::AnimationDecoder;
use crate::core::meta::{FrameData, get_manifest, get_meta};
use crate::ext::path_ext::PathExt;


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

    cli.path.file_name().expect("invalid input file path");

    let make_background_visible = match cli.background {
        None => false,
        Some(Background::Visible) => true,
        Some(Background::Invisible) => false,
    };

    let dolphin_path = cli.target
        .map(|it| it.as_dir())
        .unwrap_or_else(|| cli.path.get_parent());
    let path_src = cli.path.to_string();
    let path_name = cli.path.get_path_name();
    let input_ext = cli.path.get_ext().to_lowercase();
    let preview_path_name = format!("{}_preview", cli.path.get_path_name());

    if EXT_PICTURE.contains(&input_ext.as_str()) {
        let image = image::open(path_src).unwrap().to_rgba8();
        let bitmap = img2bm(&image, cli.height, cli.inverse, make_background_visible, &cli.threshold);

        let path_bm = format!("{path_name}.{EXT_BM}");
        let mut file_dst = File::create(path_bm).unwrap();
        file_dst.write_all(bitmap.bytes.as_slice()).unwrap();

        if cli.preview {
            let preview = bm2preview(&bitmap);
            let preview_path = format!("{preview_path_name}.{EXT_PNG}");
            save_preview(&preview, preview_path.as_str());
        }
    } else if input_ext.as_str() == EXT_GIF {
        let mut preview_frames = Vec::<GrayImage>::new();
        let dir_name = format!("{}_{TARGET_WIDTH}x{}", cli.path.get_name_no_ext(), cli.height);
        let dir_path = format!("{dolphin_path}{dir_name}/");
        create_dir_all(dir_path.as_str()).unwrap();
        let file = File::open(path_src).unwrap();
        let mut decoder = GifDecoder::new(file).unwrap();
        let mut hashes = Vec::<u64>::new();
        let mut data = Vec::<FrameData>::new();
        let mut min_duration = -1f32;
        for frame in decoder.into_frames().map(|it| it.unwrap()) {
            // todo use rayon
            let image = frame.buffer().to_owned();
            let bitmap = img2bm(&image, cli.height, cli.inverse, make_background_visible, &cli.threshold);

            let mut hasher = DefaultHasher::new();
            bitmap.hash(&mut hasher);
            let hash = hasher.finish();
            let index = hashes.iter().position(|&it| it == hash).unwrap_or_else(|| {
                let index = hashes.len();
                let path_dst = format!("{dir_path}frame_{}.{EXT_BM}", index);
                let mut file_dst = File::create(path_dst).unwrap();
                file_dst.write_all(bitmap.bytes.as_slice()).unwrap();
                hashes.push(hash);
                if cli.preview {
                    preview_frames.push(bm2preview(&bitmap));
                }
                index
            });
            let f_data = FrameData::from(index, &frame.delay());
            if min_duration < 0.0 || f_data.duration < min_duration {
                min_duration = f_data.duration;
            }
            data.push(f_data);
        }
        for f_data in data.iter_mut() {
            f_data.duration = (f_data.duration / min_duration).round() * min_duration;
        }

        let meta = get_meta(cli.height, &data);
        fs::write(format!("{dir_path}meta.txt"), meta).unwrap();
        let manifest_path = format!("{dolphin_path}manifest.txt");
        let manifest_path = Path::new(manifest_path.as_str().clone());
        let with_header = !manifest_path.exists();
        let manifest_part = get_manifest(with_header, dir_name);
        let mut manifest_file = OpenOptions::new()
            .write(true).append(true).create(true)
            .open(manifest_path)
            .unwrap();
        manifest_file.write(manifest_part.as_bytes()).unwrap();

        if cli.preview {
            let mut frames = Vec::<Frame>::new();
            for fd in data {
                let image = preview_frames.get(fd.index).unwrap();
                let dynamic = DynamicImage::from(image.clone());
                let delay = Delay::from_numer_denom_ms(fd.duration as u32, 1);
                let frame = Frame::from_parts(dynamic.to_rgba8(), 0, 0, delay);
                frames.push(frame);
            }
            let preview_path = format!("{preview_path_name}.{EXT_GIF}");
            let preview_file = File::create(preview_path).unwrap();
            let mut encoder = GifEncoder::new(preview_file);
            encoder.set_repeat(Repeat::Infinite).unwrap();
            encoder.encode_frames(frames.into_iter()).unwrap();
        }
    } else {
        panic!("invalid file format")
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
