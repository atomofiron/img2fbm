use std::fmt::Debug;
use std::path::PathBuf;
use clap::{CommandFactory, Parser};
use regex::Regex;
use crate::core::background::Background;
use crate::core::frame_cut::FrameCut;
use crate::core::values::Values;
use crate::core::scale_type::ScaleType;
use crate::core::threshold::Threshold;

#[derive(Debug, Parser)]

#[command(name = "img2fbm")]
#[command(author = "Nesterov Y. <atomofiron@gmail.com>")]
#[command(version = "1.0")]
#[command(about = "Flipper bitmap files generator", long_about = None)]
#[command(arg_required_else_help = true)]
pub struct Cli {
    /// Path to png|jpg|jpeg|gif file
    pub path: PathBuf,

    /// Target path to the 'dolphin' directory, if the gif passed
    pub target: Option<PathBuf>,

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

    /// Scale type [default: fit-bottom]
    #[arg(long = "st", value_name = "type")]
    pub scale_type: Option<ScaleType>,
    // thread 'main' has overflowed its stack
    // fatal runtime error: stack overflow
    // caused by default_value_t = ScaleType::FitBottom

    /// Generate the previews of result pictures
    #[arg(short, long)]
    pub preview: bool,

    /// Only preview, do not generate .bm and other Flipper Animation files
    #[arg(long)]
    pub op: bool,

    /// Preview scale ratio
    #[arg(long, default_value_t = 3)]
    pub ps: u8,

    /// Inverse output pixels
    #[arg(short, long)]
    pub inverse: bool,

    /// Replace dolphin/manifest.txt file with a new one.
    #[arg(short, long)]
    pub replace_manifest: bool,

    /// Set background pixels visible [default: invisible]
    #[arg(short, long, value_name = "background")]
    pub background: Option<Background>,
    // thread 'main' has overflowed its stack
    // fatal runtime error: stack overflow
    // caused by default_value_t = Background::Invisible

    /// Threshold value or range of pixel brightness as a percentage, such as 20:80, 40:, :60, 50:50 or 50 [default: 20:80]
    #[arg(short, long, value_name = "percentage[:percentage]", value_parser = str_to_threshold)]
    pub threshold: Option<Threshold>,

    /// Animation speed ratio
    #[arg(short, long, value_name = "speed", default_value_t = 1.0)]
    pub speed: f32,

    /// Drop some frames from the start and from the end. For example 5:, :8 or 2:3, the last one drops 2 frames from start and 3 from the end. [default: 0:0]
    #[arg(short, long, value_name = "count[:count]", value_parser = str_to_frame_cut)]
    pub cut: Option<FrameCut>,
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
    let from_to = Values::<u32>::from::<u32>(value, 0, 0)?;
    return Ok(FrameCut { start: from_to.first, end: from_to.second });
}
