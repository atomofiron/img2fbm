use std::fmt::{Debug, Display, Formatter};
use std::ops::RangeInclusive;
use std::path::PathBuf;
use clap::error::ErrorKind;
use clap::{CommandFactory, Parser};

const THRESHOLD_RANGE_OP: &str = "..=";

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
    pub height: u32,

    /// Generate the previews of result pictures
    #[arg(short, long)]
    pub preview: bool,

    /// Inverse output pixels
    #[arg(short, long)]
    pub inverse: bool,

    /// Inverse output pixels
    #[arg(short, long)]
    pub background: bool,

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

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum Background {
    White,
    Black,
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

pub struct RangeInc(RangeInclusive<u8>);

impl RangeInc {
    fn start(&self) -> u8 {
        *self.0.start()
    }
    fn end(&self) -> u8 {
        *self.0.end()
    }
}

impl Display for RangeInc {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}{}", self.start(), THRESHOLD_RANGE_OP, self.end())
    }
}

impl Debug for RangeInc {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self, f)
    }
}

impl Clone for RangeInc {
    fn clone(&self) -> Self {
        RangeInc(self.0.clone())
    }
}

fn verify_argument<T>(result: Result<T, String>) -> T {
    if result.is_err() {
        Cli::command()
            .error(ErrorKind::ArgumentConflict, result.err().unwrap())
            .exit();
    }
    return result.unwrap();
}
