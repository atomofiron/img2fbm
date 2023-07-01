use std::ops::Range;

pub trait StringUtil {
    fn substring(&self, range: Range<usize>) -> Self;
    fn index_of(&self, char: char) -> Option<usize>;
    fn signed_index_of(&self, char: char) -> i32;
    fn last_index_of(&self, char: char) -> Option<usize>;
    fn signed_last_index_of(&self, char: char) -> i32;
}

impl StringUtil for String {
    fn substring(&self, range: Range<usize>) -> Self {
        String::from(&self[range])
    }

    fn index_of(&self, char: char) -> Option<usize> {
        self.chars().position(|c| c == char)
    }

    fn signed_index_of(&self, char: char) -> i32 {
        self.index_of(char).map_or_else(|| -1, |i| i as i32)
    }

    fn last_index_of(&self, char: char) -> Option<usize> {
        self.chars().rev().position(|c| c == char).map(|i| self.len() - 1 - i)
    }

    fn signed_last_index_of(&self, char: char) -> i32 {
        self.last_index_of(char).map_or_else(|| -1, |i| i as i32)
    }
}
