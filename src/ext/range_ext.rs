use std::ops::{Deref, Range};

pub fn for_each<F>(
    ordinates: Range<u32>,
    abscissa: Range<u32>,
    mut action: F,
) where F: FnMut(u32,u32) {
    for y in ordinates {
        for x in abscissa.clone() {
            action(x, y);
        }
    }
}

/*pub fn for_each<T,F>(
    ordinates: Range<T>,
    abscissa: Range<T>,
    action: F,
) where F: Fn(T,T), T: Copy + std::iter::Step {
    for y in ordinates {
        for x in abscissa {
            action(x, y);
        }
    }
}*/
