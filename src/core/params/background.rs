use std::fmt::{Display, Debug, Formatter};
use clap::builder::PossibleValue;
use clap::ValueEnum;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Background {
    Invisible, Start, End, Visible,
}

impl ValueEnum for Background {
    fn value_variants<'a>() -> &'a [Self] {
        &[Background::Invisible, Background::Start, Background::End, Background::Visible]
    }

    fn to_possible_value<'a>(&self) -> Option<PossibleValue> {
        Some(match self {
            Background::Invisible => PossibleValue::new("invisible").help("Keep transparent, white, unset, zero"),
            Background::Start => PossibleValue::new("start").help("Make visible on the left or top side"),
            Background::End => PossibleValue::new("end").help("Make visible on the right or bottom side"),
            Background::Visible => PossibleValue::new("visible").help("Make visible, black, set, unit"),
        })
    }
}

impl Display for Background {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Debug for Background {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self, f)
    }
}

impl std::str::FromStr for Background {
    type Err = String;

    fn from_str(s: &str) -> core::result::Result<Self, Self::Err> {
        for variant in Self::value_variants() {
            if variant.to_possible_value().unwrap().matches(s, false) {
                return Ok(*variant);
            }
        }
        Err(format!("invalid variant: {s}"))
    }
}
