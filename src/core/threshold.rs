use std::fmt::{Debug, Display, Formatter};
use std::ops::RangeInclusive;


pub struct RangeInc(pub RangeInclusive<f32>);

impl RangeInc {

    pub fn start(&self) -> f32 {
        *self.0.start()
    }

    pub fn end(&self) -> f32 {
        *self.0.end()
    }

    pub fn is_max(&self) -> bool {
        self.start() == 0.0 && self.end() == 1.0
    }

    pub fn size(&self) -> f32 {
        self.end() - self.start()
    }
}

impl Display for RangeInc {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.start(), self.end())
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
