extern crate data;

use data::{Address, Byte};

/// Trait describing the main functionality of a VirtualMachine.
///
pub trait VirtualMachine {
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

/// Possible indexes for the Registers.
///
pub enum Register {
    V0 = 0x0,
    V1 = 0x1, // Todo: Add the other 15
}

/// Represents a collection of Registers.
///
pub trait RegisterBank {
    /// Returns the byte contained in the register.
    ///
    ///###  Arguments
    ///
    ///- **r** : The register.
    ///
    fn get_v(r: Register) -> Byte;

    /// Sets the byte for the register.
    ///
    ///###  Arguments
    ///
    ///- **r** : The register.
    ///- **b** : The byte to be stored.
    ///
    fn set_v(r: Register, b: Byte);

    /// Returns the contents of the address register.
    ///
    fn get_i() -> Address;

    /// Sets the value of the address register.
    ///
    fn set_i(a: Address);
}

/// Represents the accessible memory for the virtual machine.
///
pub trait Memory {
    /// Creates a new Memory object with a capacity of 4096kb.
    ///
    fn new() -> Self;

    /// Gets the byte stored in the register.
    ///
    ///###  Arguments
    ///
    ///- **address** : The address to the data.
    ///
    fn get(address: &Address) -> Byte;

    /// Sets the value at an address.
    ///
    /// Sets the memory at `address` to the byte `b`.
    ///
    ///###  Arguments
    ///
    ///- **address**    : The Address of the memory we setting.
    ///- **b**          : The byte.
    ///
    fn set(address: &Address);
}

pub trait FrameBuffer {
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
