use std::fmt::{Debug, Display, Formatter};
use std::ops::RangeInclusive;


pub const THRESHOLD_RANGE_OP: &str = "..=";

pub struct RangeInc(pub RangeInclusive<u8>);

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
