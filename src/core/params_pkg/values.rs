use std::str::FromStr;

pub struct Values<T> where T: FromStr + Copy {
    pub first: T,
    pub second: T,
}

impl<T> Values<T> where T: FromStr + Copy {
    pub fn from<V>(value: &str, default_first: V, default_second: V) -> Result<Values<V>, String> where V: FromStr + Copy {
        let parts = value.split(':').collect::<Vec<&str>>();
        let cause = || format!("'{value}' isn't a valid values");
        if parts.is_empty() || parts.len() > 2 {
            return Err(cause());
        }
        let first = *parts.first().unwrap();
        let first = if first.is_empty() { default_first } else {
            first.parse::<V>().map_err(|_| cause())?
        };
        if parts.len() == 1 {
            return Ok(Values { first, second: first })
        }
        let second = *parts.last().unwrap();
        let second = if second.is_empty() { default_second } else {
            second.parse::<V>().map_err(|_| cause())?
        };
        return Ok(Values { first, second });
    }
}