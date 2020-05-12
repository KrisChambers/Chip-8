extern crate data;
mod register;

pub use register::*;

use data::{Address, Byte, Nibble};

/// Trait describing the main functionality of a VirtualMachine.
///
pub trait Chip8VirtualMachine {
    /// Executes cycles of the machine.
    ///
    ///###  Arguments
    ///
    ///- **n** : The numbers of cycles to be executed.
    ///
    fn execute_cycles(&mut self, n: usize);

    /// Executes a single cycle.
    ///
    fn execute(&mut self);

    /// Loads a rom into memory
    ///
    ///### Arguments
    ///
    ///- **data** : The bytes of the rom file.
    ///
    fn load_rom(&mut self, data: Vec<u8>);

    /// Returns the FrameBuffer.
    ///
    fn get_framebuffer(&self) -> &dyn Chip8FrameBuffer;

    /// Set the key to pressed.
    ///
    ///### Arguments
    /// 
    ///- **key** : A nibble representing the key being pressed.
    ///
    fn press_key(&mut self, key: Nibble);

    /// Releases pressed keys.
    ///
    fn release_keys(&mut self);
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

    /// Gets a slice of memory starting.
    ///
    ///### Arguments
    ///
    ///- **address**    : The starting address for the slice.
    ///- **nibble**     : The size of the slice.
    ///
    fn get_slice(&self, address: Address, nibble: Nibble) -> &[Byte];
}

pub trait Chip8FrameBuffer: std::ops::Deref<Target = [u64]> + std::fmt::Debug {
    /// Draws a sprite to this buffer.
    ///
    ///###  Arguments
    ///
    ///- **x**      : The x coordinate ofr where to start drawing.
    ///- **y**      : The y coordinate for where to start drawing.
    ///- **sprite** : A slice containing the sprite data.
    ///
    fn draw(&mut self, x: Byte, y: Byte, sprite: &[Byte]) -> bool;

    /// Clears the buffer.
    ///
    /// Clears the buffer by setting all pixels back
    /// to zero.
    ///
    fn clear(&mut self);
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

pub trait Chip8Keyboard {
    /// Presses the key.
    ///
    ///### Arguments
    ///
    ///- **nibble** : The nibble representing the key pressed.
    ///
    fn press(&mut self, key: Nibble);

    /// Releases the key.
    ///
    ///### Arguments
    ///
    ///- **key** : The nibble representing the key pressed.
    ///
    fn release(&mut self, key: Nibble);

    /// Returns a boolean indicating if the key is pressed.
    ///
    ///### Arguments
    ///
    ///- **key** : The key to check.
    ///
    fn is_pressed(&self, key: Nibble) -> bool;

    /// Resets the keyboard so nothing is pressed.
    ///
    fn clear(&mut self);
}
