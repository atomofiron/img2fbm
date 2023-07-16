use std::ops::AddAssign;
use std::slice::Iter;

pub trait Sum<T> {
    fn sum_of<R, F>(&self, init: R, block: F) -> R where F: Fn(&T) -> R, R: AddAssign;
    fn min_of<R, F>(&self, init: R, block: F) -> R where F: Fn(&T) -> R, R: Ord + Eq + Copy;
    fn max_of<R, F>(&self, init: R, block: F) -> R where F: Fn(&T) -> R, R: Ord;
}

impl<'a, T> Sum<T> for Iter<'a, T> {
    fn sum_of<R, F>(&self, init: R, block: F) -> R where F: Fn(&'a T) -> R, R: AddAssign {
        let mut sum: R = init;
        for it in self.to_owned() {
            sum += block(it);
        }
        return sum;
    }
    fn min_of<R, F>(&self, init: R, block: F) -> R where F: Fn(&'a T) -> R, R: Ord + Eq + Copy {
        let mut min: R = init;
        for it in self.to_owned() {
            let next = block(it);
            if min == init || next < min {
                min = next
            }
        }
        return min;
    }
    fn max_of<R, F>(&self, init: R, block: F) -> R where F: Fn(&'a T) -> R, R: Ord {
        let mut max: R = init;
        for it in self.to_owned() {
            let next = block(it);
            if next > max {
                max = next
            }
        }
        return max;
    }
}
