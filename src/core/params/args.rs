use std::fmt::Debug;
use std::path::PathBuf;
use clap::Parser;
use crate::core::params::alignment::Alignment;
use crate::core::params::background::Background;
use crate::core::params::frame_cut::FrameCut;
use crate::core::params::values::Values;
use crate::core::params::scale_type::ScaleType;
use crate::core::params::threshold::Threshold;

#[derive(Debug, Parser)]

#[command(name = "img2fbm")]
#[command(author = "Nesterov Y. <atomofiron@gmail.com>")]
#[command(version = "1.0")]
#[command(about = "Flipper bitmap files generator", long_about = None)]
#[command(arg_required_else_help = true)]
pub struct Cli {
    /// Path to png|jpg|jpeg|gif file
    #[arg(value_name = "source")]
    pub source_path: PathBuf,

    /// Path to the 'dolphin' directory, if the GIF passed
    #[arg(value_name = "dolphin")]
    pub dolphin_path: Option<PathBuf>,

    /// Sets the height of output frame(s)
    #[arg(
        required = false,
        short = 'H',
        long,
        value_name = "1-64",
        value_parser = clap::value_parser!(u8).range(1..=64),
        default_value_t = 64,
    )]
    pub height: u8,

    /// Scale type
    #[arg(long = "st", value_name = "type", default_value = "fit")]
    pub scale_type: ScaleType,

    /// Applied alignment if the source picture has an aspect ratio different from the target
    #[arg(short, long, value_name = "side", default_value = "bottom")]
    pub alignment: Alignment,

    /// Generate the previews of result pictures
    #[arg(short, long)]
    pub preview: bool,

    /// Only preview, do not generate .bm and other Flipper Animation files
    #[arg(long = "op")]
    pub only_preview: bool,

    /// Preview scale ratio
    #[arg(long = "ps", default_value_t = 3, value_name = "multiplier")]
    pub preview_scale: u8,

    /// Inverse output pixels
    #[arg(short, long)]
    pub inverse: bool,

    /// Replace a dolphin/manifest.txt file with a new one.
    #[arg(short, long)]
    pub replace_manifest: bool,

    /// Set background pixels visible
    #[arg(short, long, value_name = "background", default_value = "invisible")]
    pub background: Background,
    // thread 'main' has overflowed its stack
    // fatal runtime error: stack overflow
    // caused by default_value_t = Background::Invisible

    /// Threshold value or range of pixel brightness as a percentage, such as 20:80, 40:, :60, 50:50 or 50
    #[arg(short, long, value_name = "percentage[:percentage]", value_parser = str_to_threshold, default_value = "20:80")]
    pub threshold: Threshold,

    /// Animation speed ratio
    #[arg(short, long, value_name = "speed", default_value_t = 1.0, value_parser = str_to_speed)]
    pub speed: f32,

    /// Drop some frames from the start and from the end. For example, 5:, :8 or 2:3, the last one drops 2 frames from start and 3 from the end.
    #[arg(short, long, value_name = "count[:count]", value_parser = str_to_frame_cut, default_value = "0:0")]
    pub cut: FrameCut,
}

fn str_to_threshold(value: &str) -> Result<Threshold, String> {
    let from_to = Values::<u8>::from::<u8>(value, 0, 100)?;
    if from_to.first > from_to.second {
        panic!("The first value must be greater than the second value")
    }
    let dark = from_to.first as f32 / 100.0;
    let light = from_to.second as f32 / 100.0;
    return Ok(Threshold { dark, light });
}

fn str_to_frame_cut(value: &str) -> Result<FrameCut, String> {
    let from_to = Values::<usize>::from::<usize>(value, 0, 0)?;
    return Ok(FrameCut { start: from_to.first, end: from_to.second });
}

fn str_to_speed(value: &str) -> Result<f32, String> {
    let value = value.parse::<f32>().map_err(|err| err.to_string())?;
    if value <= 0.0 {
        panic!("Invalid speed ratio must be greater than 0");
    }
    return Ok(value);
}
