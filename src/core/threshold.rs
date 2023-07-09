use std::fmt::{Debug, Display, Formatter};
use std::ops::RangeInclusive;


pub const THRESHOLD_RANGE_OP: &str = "..=";

pub struct RangeInc(pub RangeInclusive<u8>);

impl RangeInc {

    pub fn start(&self) -> u8 {
        *self.0.start()
    }

    pub fn end(&self) -> u8 {
        *self.0.end()
    }

    pub fn is_max(&self) -> bool {
        *self.0.start() == 0 && *self.0.end() == 100
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
