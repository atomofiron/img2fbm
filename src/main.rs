mod core;
mod ext;

use std::fs::{create_dir_all, File, OpenOptions};
use std::fs;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::io::{BufReader, Write};
use std::path::Path;
use image::{AnimationDecoder, ColorType, Delay, DynamicImage, Frame, GrayImage, ImageFormat, Luma};
use crate::core::bitmap::Bitmap;
use crate::core::img2bm::img2bm;

use image::codecs::gif::{GifDecoder, GifEncoder, Repeat};
use indicatif::{ProgressBar, ProgressStyle};
use crate::core::meta::{FrameData, get_manifest, get_meta};
use crate::core::params::params::{FileType, Params};


fn main() {
    let params = Params::parse();
    match params.file_type {
        FileType::Picture => from_picture(&params),
        FileType::Gif => from_gif(&params),
    }
}

fn from_picture(params: &Params) {
    let image = image::open(params.path_src.clone()).unwrap().to_rgba8();
    let bitmap = img2bm(&image, &params);

    if !params.only_preview {
        let mut file_dst = File::create(params.picture_path_bm.clone()).unwrap();
        file_dst.write_all(bitmap.bytes.as_slice()).unwrap();
    }
    if params.preview {
        let preview = bm2preview(&bitmap, params.preview_scale);
        save_preview(&preview, params.preview_picture_path.as_str());
    }
}

fn new_progress(length: usize, prefix: &str) -> ProgressBar {
    let mut progressbar = ProgressBar::new(length as u64);
    progressbar.set_prefix(String::from(prefix));
    progressbar.set_message("done");
    let style = ProgressStyle::with_template("{spinner:.white} {prefix} {msg} [{bar:.white/white}] {pos}/{len}")
        .unwrap()
        .progress_chars("#>-")
        .tick_chars("-\\|/#");
    progressbar.set_style(style);
    return progressbar;
}

fn from_gif(params: &Params) {
    let mut preview_frames = Vec::<GrayImage>::new();
    if !params.only_preview {
        create_dir_all(params.dolphin_anim_path.as_str()).unwrap();
    }
    let file = File::open(params.path_src.clone()).unwrap();
    let reader = BufReader::new(file);
    let mut decoder = GifDecoder::new(reader).unwrap();
    let mut hashes = Vec::<u64>::new();
    let mut data = Vec::<FrameData>::new();
    let mut min_duration = -1f32;

    let frames = decoder.into_frames().collect_frames().unwrap();
    let min_index = params.cut.start;
    let max_index = frames.len() - 1 - params.cut.end;
    let bar = new_progress(max_index + 1 - min_index, "Converting...");
    let frames_iter = frames.into_iter()
        .enumerate()
        .filter(|&(i, _)| { i >= min_index && i <= max_index })
        .map(|(_, it)| it);
    for frame in frames_iter {
        // todo use rayon
        let image = frame.buffer().to_owned();
        let bitmap = img2bm(&image, &params);

        let mut hasher = DefaultHasher::new();
        bitmap.hash(&mut hasher);
        let hash = hasher.finish();
        let index = hashes.iter().position(|&it| it == hash).unwrap_or_else(|| {
            let index = hashes.len();
            if !params.only_preview {
                let mut file_dst = File::create(params.path_bm(index)).unwrap();
                file_dst.write_all(bitmap.bytes.as_slice()).unwrap();
            }
            hashes.push(hash);
            if params.preview {
                preview_frames.push(bm2preview(&bitmap, params.preview_scale));
            }
            index
        });
        let f_data = FrameData::from(index, &frame.delay());
        if min_duration < 0.0 || f_data.duration < min_duration {
            min_duration = f_data.duration;
        }
        min_duration /= params.speed;
        data.push(f_data);
        bar.inc(1);
    }
    bar.finish();

    for f_data in data.iter_mut() {
        f_data.duration = (f_data.duration / min_duration).round() * min_duration;
    }
    if !params.only_preview {
        let meta = get_meta(params.height, &data);
        fs::write(params.meta_path.clone(), meta).unwrap();
        if params.with_manifest {
            write_manifest(&params);
        }
    }
    if params.preview {
        bm2preview_gif(&params, &data, &preview_frames)
    }
}

fn write_manifest(params: &Params) {
    let manifest_path = Path::new(params.manifest_path.as_str().clone());
    let with_header = params.replace_manifest || !manifest_path.exists();
    let manifest_part = get_manifest(with_header, params.dolphin_anim_name.clone());
    let mut manifest_file = OpenOptions::new()
        .create(true).write(true).append(!params.replace_manifest)
        .open(manifest_path)
        .unwrap();
    manifest_file.write(manifest_part.as_bytes()).unwrap();
}

fn bm2preview_gif(params: &Params, data: &Vec::<FrameData>, preview_frames: &Vec::<GrayImage>) {
    let bar = new_progress(preview_frames.len(), "Generating preview...");
    let mut frames = Vec::<Frame>::new();
    for fd in data {
        let image = preview_frames.get(fd.index).unwrap();
        let dynamic = DynamicImage::from(image.clone());
        let duration = (fd.duration / params.speed) as u32;
        let delay = Delay::from_numer_denom_ms(duration, 1);
        let frame = Frame::from_parts(dynamic.to_rgba8(), 0, 0, delay);
        frames.push(frame);
        bar.inc(1);
    }
    let preview_file = File::create(params.preview_gif_path.clone()).unwrap();
    let mut encoder = GifEncoder::new(preview_file);
    encoder.set_repeat(Repeat::Infinite).unwrap();
    encoder.encode_frames(frames.into_iter()).unwrap();
    bar.finish();
}

fn bm2preview(bitmap: &Bitmap, scale: u32) -> GrayImage {
    let width = bitmap.width as u32;
    let height = bitmap.height as u32;
    let mut image = GrayImage::new(width * scale, height * scale);
    for y in 0..height {
        for x in 0..width {
            // +1 because of the first byte is extra 0x00
            let bit = bitmap.get(x, y);
            if !bit {
                for x in (x * scale)..(x * scale + scale) {
                    for y in (y * scale)..(y * scale + scale) {
                        image.put_pixel(x, y, Luma([255u8]));
                    }
                }
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
