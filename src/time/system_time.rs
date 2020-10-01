use std::time::{SystemTime, UNIX_EPOCH};

pub struct SystemLocalTime {}

/// 为了美观获取当前时间的包装实现
impl SystemLocalTime {
    ///获取当前的unix纳秒时间
    /// # Example
    /// ```
    /// use fantasy_util::time::system_time::SystemLocalTime;
    /// let nanos:u128 = SystemLocalTime::unix_nanos();
    /// ```
    pub fn unix_nanos() -> u128 {
        SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos()
    }

    ///获取当前的unix微秒时间
    /// # Example
    /// ```
    /// use fantasy_util::time::system_time::SystemLocalTime;
    /// let micros:u128 = SystemLocalTime::unix_micros();
    /// ```
    pub fn unix_micros() -> u128 {
        SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_micros()
    }

    ///获取当前的unix毫秒时间
    /// # Example
    /// ```
    /// use fantasy_util::time::system_time::SystemLocalTime;
    /// let mills:u64 = SystemLocalTime::unix_mills();
    /// ```
    pub fn unix_mills() -> u64 {
        SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as u64
    }

    ///获取当前的unix秒时间
    /// # Example
    /// ```
    /// use fantasy_util::time::system_time::SystemLocalTime;
    /// let mills:u64 = SystemLocalTime::unix_secs();
    /// ```
    pub fn unix_secs() -> u64 {
        SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs()
    }
}


#[cfg(test)]
mod tests {
    use crate::time::system_time::SystemLocalTime;

    #[test]
    fn it_works() {
        println!("{:X}", u128::max_value());
        println!("{:X}", SystemLocalTime::unix_nanos());
        println!("{:X}", SystemLocalTime::unix_micros());
        println!("{:X}", SystemLocalTime::unix_mills());
        println!("{:X}", SystemLocalTime::unix_secs());
        assert_eq!(2 + 2, 4);
    }
}
