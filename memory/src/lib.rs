extern crate data;
extern crate model;

use model::{
    Memory
};

use data::{
    Byte,
    Address
};

pub struct Chip8Memory {
    store: [Byte; 4096],
}

impl Chip8Memory {
    /// Creates a new Chip8Memory.
    ///
    pub fn new() -> Self {
        Chip8Memory { store: [0.into(); 4096 ]}
    }
}

impl Memory for Chip8Memory {
    
    fn get(&self, addr: data::Address) -> data::Byte {
        let addr: u16 = addr.into();

        self.store[addr as usize]
    }

    fn set(&mut self, addr: data::Address, byte: Byte) {
        let addr: u16 = addr.into();

        self.store[addr as usize] = byte;
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn new_works() {
        let mut mem = Chip8Memory::new();

        mem.set(0u16.into(), 1.into());
    }

    #[test]
    fn get_works() {
        let mut mem = Chip8Memory::new();

        mem.store[0] = 1.into();

        assert_eq!(mem.get(0.into()), 1.into());
    }

    #[test]
    fn set_works() {
        let mut mem = Chip8Memory::new();

        mem.set(0.into(), 1.into());
        let byte = mem.get(0.into());

        assert_eq!(byte, 1u8.into());
    }
}
