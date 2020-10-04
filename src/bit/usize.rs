/// 对于usize值的扩展方法
pub trait BitUtil<T> {
    /// 获取指定长度的位，其他位置0
    ///
    /// #Example
    /// ```
    /// use fantasy_util::bit::usize::BitUtil;
    ///
    /// let a = 0b1001_1001;
    /// let i:u32 = a.extract(3,4); // i = 0b0000_0011;
    /// ```
    fn extract(self, off: u8, len: u8) -> T;

    /// 获取指定位的bool值
    ///
    /// #Example
    /// ```
    /// use fantasy_util::bit::usize::BitUtil;
    ///
    /// let a:u32 = 0b1001_1001 as u32;
    /// let b:bool = a.get_bit_bool(0); // b = true;
    /// ```
    fn get_bit_bool(self, bit: u8) -> bool { self.get_bit(bit) != 0 }

    /// 获取指定位
    ///
    /// #Example
    /// ```
    /// use fantasy_util::bit::usize::BitUtil;
    ///
    /// let a:u32 = 0b1001_1001 as u32;
    /// let b:u8 = a.get_bit(0); // b = 1;
    /// ```
    fn get_bit(self, bit: u8) -> u8;
}

impl BitUtil<u8> for u8 {
    fn extract(self, off: u8, len: u8) -> u8 { (self >> off) & ((1u8 << len) - 1) }


    fn get_bit(self, bit: u8) -> u8 { ((self >> bit) as u8) & 1 }
}

impl BitUtil<u16> for u16 {
    fn extract(self, off: u8, len: u8) -> u16 { (self >> off) & ((1u16 << len) - 1) }


    fn get_bit(self, bit: u8) -> u8 { ((self >> bit) as u8) & 1 }
}


impl BitUtil<u32> for u32 {
    fn extract(self, off: u8, len: u8) -> u32 { (self >> off) & ((1u32 << len) - 1) }

    fn get_bit(self, bit: u8) -> u8 { ((self >> bit) as u8) & 1 }
}

impl BitUtil<u64> for u64 {
    fn extract(self, off: u8, len: u8) -> u64 { (self >> off) & ((1u64 << len) - 1) }

    fn get_bit_bool(self, bit: u8) -> bool { self.get_bit(bit) != 0 }

    fn get_bit(self, bit: u8) -> u8 { ((self >> bit) as u8) & 1 }
}


impl BitUtil<u128> for u128 {
    fn extract(self, off: u8, len: u8) -> u128 { (self >> off) & ((1u128 << len) - 1) }

    fn get_bit_bool(self, bit: u8) -> bool { self.get_bit(bit) != 0 }

    fn get_bit(self, bit: u8) -> u8 { ((self >> bit) as u8) & 1 }
}


#[cfg(test)]
mod tests {
    use crate::bit::usize::BitUtil;

    #[test]
    fn it_works() {
        let a: u32 = 0b1001_1001;
        let i = a.extract(3, 4);
        assert_eq!(0b0011, a.extract(3, 4));
    }
}