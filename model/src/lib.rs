extern crate data;
mod register;

pub use register::*;

use data::{
    Address,
    Byte
};

/// Trait describing the main functionality of a VirtualMachine.
///
pub trait Chip8VirtualMachine {
    /// Creates a new VirtualMachine.
    ///
    fn new() -> Self;

    /// Executes cycles of the machine.
    ///
    ///###  Arguments
    ///
    ///- **n** : The numbers of cycles to be executed.
    ///
    fn execute_cycles(&mut self, n: usize);

    /// Returns the FrameBuffer.
    ///
    fn get_framebuffer(&self) -> &[usize];
}

/// Represents a collection of Registers.
///
pub trait Chip8RegisterBank {
    /// Returns the byte contained in the register.
    ///
    ///###  Arguments
    ///
    ///- **r** : The register.
    ///
    fn get_v(&self, r: Register) -> Byte;

    /// Sets the byte for the register.
    ///
    ///###  Arguments
    ///
    ///- **r** : The register.
    ///- **b** : The byte to be stored.
    ///
    fn set_v(&mut self, r: Register, b: Byte);

    /// Returns the contents of the address register.
    ///
    fn get_i(&self) -> Address;

    /// Sets the value of the address register.
    ///
    ///###  Arguments
    ///
    ///- **a** : The address to be stored.
    ///
    fn set_i(&mut self, a: Address);
}

/// Represents the accessible memory for the virtual machine.
///
pub trait Chip8Memory {
    /// Gets the byte stored in the register.
    ///
    ///###  Arguments
    ///
    ///- **address** : The address to the data.
    ///
    fn get(&self, address: Address) -> Byte;

    /// Sets the value at an address.
    ///
    /// Sets the memory at `address` to the byte `b`.
    ///
    ///###  Arguments
    ///
    ///- **address**    : The Address of the memory we setting.
    ///- **b**          : The byte.
    ///
    fn set(&mut self, address: Address, byte: Byte);
}

pub trait Chip8FrameBuffer: std::ops::Deref<Target = [u64]> {
    /// Draws a sprite to this buffer.
    ///
    ///###  Arguments
    ///
    ///- **x**      : The x coordinate ofr where to start drawing.
    ///- **y**      : The y coordinate for where to start drawing.
    ///- **sprite** : A slice containing the sprite data.
    ///
    fn draw(&mut self, x: usize, y: usize, sprite: &[Byte]) -> bool;
}

pub trait Chip8ProgramCounter {
    /// Gets the address of the currently executing instruction.
    ///
    fn current(&self) -> Address;

    /// Increments the program counter by 1.
    ///
    fn inc(&mut self);

    /// Sets the program counter to the subroutine.
    ///
    ///### Arguments
    ///
    ///- **addr** : The address of the subroutine.
    ///
    fn to_subroutine(&mut self, addr: Address);

    /// Returns from the current routine.
    ///
    fn rtrn(&mut self);

    /// Increments the program counter by the provided amount.
    ///
    ///### Arguments
    ///
    ///- **amt** :  The amount to increase the counter by.
    ///
    fn inc_by(&mut self, amt: Byte);

    /// Returns the depth of the stack.
    ///
    fn depth(&self) -> usize;

    /// Sets the current executing instruction.
    ///
    fn set(&mut self, addr: Address);
}

