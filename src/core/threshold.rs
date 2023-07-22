use std::fmt::{Debug, Display, Formatter};


pub struct Threshold {
    pub dark: f32,
    pub light: f32,
}

impl Threshold {

    pub fn is_empty(&self) -> bool {
        self.dark == self.light
    }

    pub fn is_max(&self) -> bool {
        self.dark == 0.0 && self.light == 1.0
    }

    pub fn size(&self) -> f32 {
        self.light - self.dark
    }

    pub fn contains(&self, other: f32) -> bool {
        self.dark < other && other < self.light
    }
}

impl Display for Threshold {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.dark, self.light)
    }
}

impl Debug for Threshold {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self, f)
    }
}

impl Clone for Threshold {
    fn clone(&self) -> Self {
        Threshold {
            dark: self.dark,
            light: self.light,
        }
    }
}
