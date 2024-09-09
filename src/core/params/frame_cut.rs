use std::fmt::{Debug, Display, Formatter};


pub struct FrameCut {
    pub start: usize,
    pub end: usize,
}

impl Display for FrameCut {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.start, self.end)
    }
}

impl Debug for FrameCut {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self, f)
    }
}

impl Clone for FrameCut {
    fn clone(&self) -> Self {
        FrameCut {
            start: self.start,
            end: self.end,
        }
    }
}
