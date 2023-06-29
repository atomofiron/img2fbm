use std::fmt::Debug;
use std::path::PathBuf;
use clap::{CommandFactory, Parser};
use crate::core::background::Background;
use crate::core::threshold::{RangeInc, THRESHOLD_RANGE_OP};

#[derive(Debug, Parser)]

#[command(name = "img2fbm")]
#[command(author = "Nesterov Y. <atomofiron@gmail.com>")]
#[command(version = "1.0")]
#[command(about = "Flipper bitmap files generator", long_about = None)]
pub struct Cli {
    /// Path to png|jpg|gif file
    pub path: PathBuf,

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
        default_value_t = RangeInc(40..=60),
        value_parser = str_to_threshold,
    )]
    pub threshold: RangeInc,
}

fn str_to_threshold(value: &str) -> Result<RangeInc, String> {
    let parts = value.split(':').collect::<Vec<&str>>();
    // seems like clap contains a bug
    let parts2 = value.split(THRESHOLD_RANGE_OP).collect::<Vec<&str>>();
    let parts = if parts.len() > parts2.len() { parts } else { parts2 };
    let mapper = || format!("'{value}' isn't a valid range");
    if parts.len() > 2 {
        return Err(mapper())
    }
    let first = parts[0];
    let first = if first.is_empty() { "0" } else { first };
    let second = *parts.get(1).unwrap_or(&first);
    let second = if second.is_empty() { "100" } else { second };
    let mapper = |_| mapper();
    let first: u8 = first.parse().map_err(mapper)?;
    let second: u8 = second.parse().map_err(mapper)?;
    Ok(RangeInc(first..=second))
}
