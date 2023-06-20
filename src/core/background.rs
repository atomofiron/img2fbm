use std::fmt::{Debug, Formatter};
use clap::builder::PossibleValue;
use clap::ValueEnum;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Background {
    White,
    Black,
}

impl ValueEnum for Background {
    fn value_variants<'a>() -> &'a [Self] {
        &[Background::White, Background::Black]
    }

    fn to_possible_value<'a>(&self) -> Option<PossibleValue> {
        Some(match self {
            Background::White => PossibleValue::new("white").help("Keep transparent, white, unset, zero"),
            Background::Black => PossibleValue::new("black").help("Make visible, black, set, unit"),
        })
    }
}

impl std::fmt::Display for Background {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let str = self.to_possible_value()
            .expect("no values are skipped")
            .get_name();
        Debug::fmt(str, f)
    }
}

impl Debug for Background {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self, f)
    }
}

impl std::str::FromStr for Background {
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
