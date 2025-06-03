use std::io::Write;

pub trait UnitUtil {
    fn flush(self: Self);
}

impl UnitUtil for () {

    fn flush(self: Self) {
        std::io::stdout().flush().unwrap();
    }
}