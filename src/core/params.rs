use std::fmt::Display;
use std::ops::Deref;
use crate::core::args::Cli;
use crate::core::background::Background;
use crate::core::threshold::RangeInc;
use crate::ext::path_ext::{EXT_PNG, EXT_GIF, EXT_BM, EXT_PICTURE, PathExt};


pub const TARGET_WIDTH: u8 = 128;

pub struct Params {
    pub width: u8,
    pub height: u8,
    pub preview: bool,
    pub inverse: bool,
    pub background_visible: bool,
    pub threshold: RangeInc,

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
    pub fn from(cli: Cli) -> Params {
        let path_name = cli.path.get_path_name();
        let input_ext = cli.path.get_ext().to_lowercase();
        let preview_path_name = format!("{}_preview", cli.path.get_path_name());
        let preview_picture_path = format!("{preview_path_name}.{EXT_PNG}");
        let preview_gif_path = format!("{preview_path_name}.{EXT_GIF}");
        let picture_path_bm = format!("{path_name}.{EXT_BM}");
        let dolphin_path = cli.target.clone().map(|it| it.as_dir()).unwrap_or_else(|| cli.path.get_parent());
        let dolphin_anim_name = format!("{}_{TARGET_WIDTH}x{}", cli.path.get_name_no_ext(), cli.height);
        let dolphin_anim_path = format!("{dolphin_path}{dolphin_anim_name}/");
        let meta_path = format!("{dolphin_anim_path}meta.txt");
        let manifest_path = format!("{dolphin_path}manifest.txt");
        println!("src {}, path_name {path_name}\ninput_ext {input_ext}\npreview_path_name {preview_path_name}\npreview_picture_path {preview_picture_path}\npreview_gif_path {preview_gif_path}\npicture_path_bm {picture_path_bm}\ndolphin_path {dolphin_path}\ndolphin_anim_name {dolphin_anim_name}\ndolphin_anim_path {dolphin_anim_path}\nmeta_path {meta_path}\nmanifest_path {manifest_path}", cli.path.to_string());
        Params {
            width: TARGET_WIDTH,
            height: cli.height,
            preview: cli.preview,
            inverse: cli.inverse,
            background_visible: match cli.background {
                None => false,
                Some(Background::Visible) => true,
                Some(Background::Invisible) => false,
            },
            threshold: cli.threshold.clone(),
            path_src: cli.path.to_string(),
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
}

impl Params {
    pub fn path_bm<I>(&self, index: I) -> String where I: Display {
        format!("{}frame_{}.{EXT_BM}", self.dolphin_anim_path, index)
    }
}

/*impl From<Cli> for Params {
    fn from(cli: Cli) -> Params {}
}*/
