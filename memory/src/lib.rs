extern crate data;
extern crate model;

use model::Chip8Memory;

use data::{ Address, Byte, Nibble };

pub struct Memory {
    store: [Byte; 4096],
}

impl Memory {
    /// Creates a new Chip8Memory.
    ///
    pub fn new() -> Self {
        Memory {
            store: [0.into(); 4096],
        }
    }
}

impl Chip8Memory for Memory {
    fn get(&self, addr: Address) -> Byte {
        let addr = addr
            .get_raw() as usize;

        self.store[addr]
    }

    fn set(&mut self, addr: Address, byte: Byte) {
        let addr = addr
            .get_raw() as usize;

        self.store[addr] = byte;
    }

    fn get_slice(&self, address: Address, length: Nibble) -> &[Byte] {
        let start: usize = address.get_raw().into();
        let length: usize = length.get_raw().into();
        let end = start + length;

        &self.store[start .. end]
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn new_works() {
        let mut mem = Memory::new();

        mem.set(0u16.into(), 1.into());
    }

    #[test]
    fn get_works() {
        let mut mem = Memory::new();

        mem.store[0] = 1.into();

        assert_eq!(mem.get(0.into()), 1.into());
    }

    #[test]
    fn set_works() {
        let mut mem = Memory::new();

        mem.set(0.into(), 1.into());
        let byte = mem.get(0.into());

        assert_eq!(byte, 1.into());
    }
}
