//! Basic Address data type.
//
numeric_wrapper!(Address, u16, |x| x & 0x0FFF);

impl std::ops::AddAssign for Address {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
    }
}