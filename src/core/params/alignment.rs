use std::fmt::{Debug, Display, Formatter};
use clap::builder::PossibleValue;
use clap::ValueEnum;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Alignment {
    Left, Top, Right, Bottom
}

impl ValueEnum for Alignment {
    fn value_variants<'a>() -> &'a [Self] {
        &[Alignment::Left, Alignment::Top, Alignment::Right, Alignment::Bottom]
    }

    fn to_possible_value<'a>(&self) -> Option<PossibleValue> {
        Some(match self {
            Alignment::Left => PossibleValue::new("left").help("Align source picture to left"),
            Alignment::Top => PossibleValue::new("top").help("Align source picture to top"),
            Alignment::Right => PossibleValue::new("right").help("Align source picture to right"),
            Alignment::Bottom => PossibleValue::new("bottom").help("Align source picture to bottom"),
        })
    }
}

impl Display for Alignment {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Debug for Alignment {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self, f)
    }
}

impl std::str::FromStr for Alignment {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        for variant in Self::value_variants() {
            if variant.to_possible_value().unwrap().matches(s, false) {
                return Ok(*variant);
            }
        }
        Err(format!("invalid variant: {s}"))
    }
}
