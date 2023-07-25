use std::fmt::{Debug, Display, Formatter};
use clap::builder::PossibleValue;
use clap::ValueEnum;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum ScaleType {
    Fill, Fit
}

impl ValueEnum for ScaleType {
    fn value_variants<'a>() -> &'a [Self] {
        &[ScaleType::Fill, ScaleType::Fit]
    }

    fn to_possible_value<'a>(&self) -> Option<PossibleValue> {
        Some(match self {
            ScaleType::Fill => PossibleValue::new("fill").help("Scale to fill animation bounds"),
            ScaleType::Fit => PossibleValue::new("fit").help("Scale to fit in animation bounds"),
        })
    }
}

impl Display for ScaleType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Debug for ScaleType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self, f)
    }
}

impl std::str::FromStr for ScaleType {
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
