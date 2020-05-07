//! Basic Address data type.
//

use super::byte::Byte;

numeric_wrapper!(Address, u16, |x| x & 0x0FFF);

use std::ops::{
    AddAssign,
    Add
};

impl AddAssign for Address {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
    }
}

impl Add<Byte> for Address {
    type Output = Self;
    fn add(self, rhs: Byte) -> Self {
        let rhs: u16 = rhs.get_raw().into();
        
        Self(self.0 + rhs)
    }
}
