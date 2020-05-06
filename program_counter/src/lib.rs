extern crate data;
extern crate model;

use data::{Address, Byte};

use model::Chip8ProgramCounter;

const STACK_SIZE: usize = 16;

/// An implementation of a program counter
///
/// A ProgramCounter maintains the pointer to the current instruction.
///
pub struct ProgramCounter {
    items: [Address; STACK_SIZE],
    pointer: usize,
}

impl ProgramCounter {
    /// Creates a new ProgramCounter.
    ///
    ///### Arguments
    ///
    ///- **initial** : The initial address.
    ///
    pub fn new(initial: Address) -> Self {
        let mut this = ProgramCounter {
            items: [Address::new(0); STACK_SIZE],
            pointer: 0,
        };

        this.items[this.pointer] = initial;

        this
    }
}

impl Chip8ProgramCounter for ProgramCounter {
    fn current(&self) -> Address {
        self.items[self.pointer]
    }

    fn inc(&mut self) {
        self.items[self.pointer] += 1.into();
    }

    fn to_subroutine(&mut self, new_ptr: Address) {
        self.pointer += 1;
        self.items[self.pointer] = new_ptr;
    }

    fn rtrn(&mut self) {
        if self.pointer == 0 {
            panic!("Popping the CallStack will empty the stack.")
        }

        self.pointer -= 1;
    }

    fn inc_by(&mut self, amt: Byte) {
        let amt: u16 = amt.get_raw().into();
        self.items[self.pointer] += amt.into();
    }

    fn depth(&self) -> usize {
        self.pointer + 1
    }

    fn set(&mut self, new_pc: Address) {
        self.items[self.pointer] = new_pc;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn new_has_current_set_to_provided_value() {
        let pc = ProgramCounter::new(0x200.into());

        assert_eq!(pc.current(), 0x200.into());
    }

    #[test]
    fn increment_increases_current_by_1() {
        let mut pc = ProgramCounter::new(0x200.into());

        pc.inc();

        let new_pc = 0x200 + 1;
        assert_eq!(pc.current(), new_pc.into());
    }

    #[test]
    fn inc_by_increases_current_by_the_provided_amount() {
        let mut pc = ProgramCounter::new(0x200.into());

        pc.inc_by(5.into());

        assert_eq!(pc.current(), (0x200 + 5).into());
    }

    #[test]
    #[should_panic]
    fn returning_when_only_1_address_panics() {
        let mut pc = ProgramCounter::new(0x200.into());

        pc.rtrn();
    }
}
