//! Basic Address data type.
//
numeric_wrapper!(Address, u16, |x| x & 0x0FFF);
