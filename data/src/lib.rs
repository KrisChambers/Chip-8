#[macro_use]
mod data_macros;

mod byte;
mod nibble;

pub use byte::Byte;
pub use nibble::Nibble;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
