//! Represents a single byte.
//!

numeric_wrapper!(Byte, u8, |x| x & 0x00FF);

impl Byte {
    /// Returns the least significant bit.
    ///
    pub fn get_lsb(&self) -> u8 {
        self.0 & 0b00000001
    }

    /// Returns the most significant bit.
    ///
    pub fn get_msb(&self) -> u8 {
        (self.0 & 0b10000000) >> 7
    }

    /// Returns the BCD representation.
    ///
    /// # Example
    ///
    /// ```rust
    /// let (h, t, o) = data::Byte::new(123).get_bcd_rep();
    ///
    /// assert_eq!(h, 1);
    /// assert_eq!(t, 2);
    /// assert_eq!(o, 3);
    /// ```
    ///
    pub fn get_bcd_rep(&self) -> (u8, u8, u8) {
        let x = self.0;
        let hundreds = x.checked_div(100).unwrap_or(0);
        let x = x.checked_rem(100).unwrap_or(0);

        let tens = x.checked_div(10).unwrap_or(0);
        let ones = x.checked_rem(10).unwrap_or(0);

        (hundreds, tens, ones)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_lsb_return_1_for_odd_numbers() {
        let x = Byte::new(123);

        assert_eq!(x.get_lsb(), 1);
    }

    #[test]
    fn get_msb_returns_1_for_255() {
        let x = Byte::new(255);

        assert_eq!(x.get_msb(), 1);
    }

    #[test]
    fn get_bcd_rep_of_100_10_and_1() {
        let x = Byte::new(100);

        assert_eq!(x.get_bcd_rep(), (1, 0, 0));

        let x = Byte::new(10);

        assert_eq!(x.get_bcd_rep(), (0, 1, 0));

        let x = Byte::new(1);

        assert_eq!(x.get_bcd_rep(), (0, 0, 1));
    }
}
