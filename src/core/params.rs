use crate::core::args::Cli;
use crate::core::background::Background;
use crate::core::threshold::RangeInc;
use crate::ext::path_ext::PathExt;

pub struct Params {
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

impl From<Cli> for Params {
    fn from(cli: Cli) -> Params {
        Params {
            height: cli.height,
            preview: cli.preview,
            inverse: cli.inverse,
            background_visible: match cli.background {
                None => false,
                Some(Background::Visible) => true,
                Some(Background::Invisible) => false,
            },
            threshold: cli.threshold,
            path_src: cli.path.to_string(),
            path_name: cli.path.get_path_name(),
            input_ext: cli.path.get_ext().to_lowercase(),
            preview_path_name: format!("{}_preview", cli.path.get_path_name()),
            preview_picture_path: format!("{preview_path_name}.{EXT_PNG}"),
            preview_gif_path: format!("{preview_path_name}.{EXT_GIF}"),
            picture_path_bm: format!("{path_name}.{EXT_BM}"),
            dolphin_path: cli.targe.map(|it| it.as_dir()).unwrap_or_else(|| cli.path.get_parent()),
            dolphin_anim_name: format!("{}_{TARGET_WIDTH}x{}", cli.path.get_name_no_ext(), cli.height),
            dolphin_anim_path: format!("{dolphin_path}{dir_name}/"),
            meta_path: format!("{dolphin_anim_path}meta.txt"),
            manifest_path: format!("{dolphin_path}manifest.txt"),
        }
    }
}
