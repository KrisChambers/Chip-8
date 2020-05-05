//! Represents 4 bits of data.
//!

numeric_wrapper!(Nibble, u8, |x| x & 0x000F);
