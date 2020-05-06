extern crate model;

use model::Chip8VirtualMachine;

pub struct VirtualMachine {}

impl VirtualMachine {
    fn new() -> Self {
        VirtualMachine {}
    }
}

impl Chip8VirtualMachine for VirtualMachine {
    fn execute_cycles(&mut self, cycles: usize) {
        for cycle in 0..cycles {}
    }

    fn get_framebuffer(&self) -> &[u64] {
        Default::default()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
