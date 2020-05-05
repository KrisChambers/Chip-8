extern crate data;
extern crate model;

use model::{Register, RegisterBank};

use data::{Address, Byte};

pub struct Chip8RegisterBank {
    store: [Byte; 16],
    i: Address,
}

impl Chip8RegisterBank {
    pub fn new() -> Self {
        Chip8RegisterBank {
            store: [0.into(); 16],
            i: 0.into(),
        }
    }
}

impl RegisterBank for Chip8RegisterBank {
    fn get_v(&self, r: Register) -> Byte {
        self.store[r as usize]
    }
    fn set_v(&mut self, r: Register, b: Byte) {
        self.store[r as usize] = b;
    }
    fn get_i(&self) -> Address {
        self.i
    }
    fn set_i(&mut self, addr: Address) {
        self.i = addr;
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn set_v_works() {
        let mut rb = Chip8RegisterBank::new();

        rb.set_v(Register::V0, 1.into());

        assert_eq!(rb.store[0], 1.into());
    }
}
