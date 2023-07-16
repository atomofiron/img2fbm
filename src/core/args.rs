use std::fmt::Debug;
use std::path::PathBuf;
use clap::{CommandFactory, Parser};
use regex::Regex;
use crate::core::background::Background;
use crate::core::threshold::{RangeInc, THRESHOLD_RANGE_OP};

#[derive(Debug, Parser)]

#[command(name = "img2fbm")]
#[command(author = "Nesterov Y. <atomofiron@gmail.com>")]
#[command(version = "1.0")]
#[command(about = "Flipper bitmap files generator", long_about = None)]
#[command(arg_required_else_help = true)]
pub struct Cli {
    /// Path to png|jpg|gif file
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

    /// Generate the previews of result pictures
    #[arg(short, long)]
    pub preview: bool,

    /// Inverse output pixels
    #[arg(short, long)]
    pub inverse: bool,

    /// Set background pixels visible
    #[arg(short, long)]
    pub background: Option<Background>,
    // thread 'main' has overflowed its stack
    // fatal runtime error: stack overflow
    // caused by default_value_t = Background::Invisible

    /// Threshold value or range of pixel brightness as a percentage, like 20:80, 40:, :60, 50:50 or 50
    #[arg(
        short,
        long,
        value_name = "percentage[:percentage]",
        default_value_t = RangeInc(0.2..=0.8),
        value_parser = str_to_threshold,
    )]
    pub threshold: RangeInc,
}

fn str_to_threshold(value: &str) -> Result<RangeInc, String> {
    // seems like clap contains a bug
    let pattern = Regex::new(r":|\.\.=").unwrap();
    let parts = pattern.split(value).collect::<Vec<&str>>();
    let cause = || format!("'{value}' isn't a valid range");
    if parts.is_empty() || parts.len() > 2 {
        return Err(cause());
    }
    let first = *parts.first().unwrap();
    let first = if value.is_empty() { 0.0 } else {
        parse(first).map_err(|_| cause())?
    };
    if parts.len() == 1 {
        return Ok(RangeInc(first..=first))
    }
    let second = *parts.last().unwrap();
    let second = if second.is_empty() { 1.0 } else {
        parse(second).map_err(|_| cause())?
    };
    return Ok(RangeInc(first..=second));
}

fn parse(value: &str) -> Result<f32, ()> {
    let int = value.parse::<u8>();
    if let Ok(value) = int {
        return Ok(value as f32 / 100.0);
    }
    let float = value.parse::<f32>();
    if let Ok(value) = float {
        return Ok(value);
    }
    return Err(());
}
