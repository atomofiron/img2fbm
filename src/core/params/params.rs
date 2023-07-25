use std::fmt::Display;
use clap::Parser;
use crate::core::params::args::Cli;
use crate::core::params::background::Background;
use crate::core::params::frame_cut::FrameCut;
use crate::core::params::scale_type::ScaleType;
use crate::core::params::threshold::Threshold;
use crate::ext::path_ext::{PathExt, EXT_PNG, EXT_GIF, EXT_BM, EXT_PICTURE};


const TARGET_WIDTH: u8 = 128;

pub enum FileType {
    Picture, Gif
}

pub struct Params {
    pub file_type: FileType,
    pub width: u8,
    pub height: u8,
    pub preview: bool,
    pub only_preview: bool,
    pub preview_scale: u32,
    pub inverse: bool,
    pub background: Background,
    pub threshold: Threshold,
    pub cut: FrameCut,
    pub scale_type: ScaleType,
    pub speed: f32,
    pub with_manifest: bool,
    pub replace_manifest: bool,

    pub path_src: String,
    pub path_name: String,
    pub input_ext: String,
    pub preview_path_name: String,
    pub preview_picture_path: String,
    pub preview_gif_path: String,
    pub picture_path_bm: String,
    pub dolphin_path: String,
    pub dolphin_anim_name: String,
    pub dolphin_anim_path: String,
    pub meta_path: String,
    pub manifest_path: String,
}

impl Params {

    pub fn parse() -> Params {
        let cli = Cli::parse();
        cli.source_path.extension().expect("invalid input file");
        cli.source_path.file_name().expect("invalid input file path");
        let input_ext = cli.source_path.get_ext().to_lowercase();
        let file_type = match () {
            _ if EXT_PICTURE.contains(&&*input_ext) => FileType::Picture,
            _ if input_ext == EXT_GIF => FileType::Gif,
            _ => panic!("invalid input file format"),
        };
        let path_name = cli.source_path.get_path_name();
        let preview_path_name = format!("{}_preview", cli.source_path.get_path_name());
        let preview_picture_path = format!("{preview_path_name}.{EXT_PNG}");
        let preview_gif_path = format!("{preview_path_name}.{EXT_GIF}");
        let picture_path_bm = format!("{path_name}.{EXT_BM}");
        let dolphin_path = cli.dolphin_path.clone()
            .map(|it| it.as_dir())
            .unwrap_or_else(|| cli.source_path.get_parent());
        let dolphin_anim_name = format!("{}_{TARGET_WIDTH}x{}", cli.source_path.get_name_no_ext(), cli.height);
        let dolphin_anim_path = format!("{dolphin_path}{dolphin_anim_name}/");
        let meta_path = format!("{dolphin_anim_path}meta.txt");
        let manifest_path = format!("{dolphin_path}manifest.txt");
        Params {
            file_type,
            width: TARGET_WIDTH,
            height: cli.height,
            preview: cli.preview || cli.only_preview,
            only_preview: cli.only_preview,
            preview_scale: cli.preview_scale as u32,
            inverse: cli.inverse,
            background: cli.background,
            threshold: cli.threshold,
            cut: cli.cut,
            scale_type: cli.scale_type,
            speed: cli.speed,
            with_manifest: cli.dolphin_path.is_some(),
            replace_manifest: cli.replace_manifest,

            path_src: cli.source_path.to_string(),
            path_name,
            input_ext,
            preview_path_name,
            preview_picture_path,
            preview_gif_path,
            picture_path_bm,
            dolphin_path,
            dolphin_anim_name,
            dolphin_anim_path,
            meta_path,
            manifest_path,
        }
    }

    pub fn path_bm<I>(&self, index: I) -> String where I: Display {
        format!("{}frame_{}.{EXT_BM}", self.dolphin_anim_path, index)
    }
}
