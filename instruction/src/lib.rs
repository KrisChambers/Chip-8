//! ##Chip-8 Instructions
//!
//! Each instruction is 16bits long or 4 nibbles.
//! Each instruction has atleast 1 nibble used for identifying the instruction.
//!
//! If a memory location is referenced it takes up the remaining 3 nibbles,
//! If only 1 register is referenced nibble 2 references Vx.
//! If 2 registers are referenced nibble 3 references Vy.
//!
//! If loading a value of bytes, nibble 3 and 4 make up the byte.
//!
//! Otherwise nibble 4 is used to identify the instruction.
//!
//! 


extern crate data;
extern crate model;

mod utils;


use data::{
    Byte,
    Address,
    Nibble
};
use model::{
    Register
};

/// First instruction register argument.
type Vx = Register;
/// Second instruction register argument.
type Vy = Register;

/// A Raw instruction is of the form 0xnnnn.
type RawInstruction = u16;

/// An instruction to be executed by the vm.
///
#[derive(Debug, PartialEq)]
pub enum Instruction {
    // Represents an invalid instruction
    Invalid(RawInstruction),

    /// Jumps to a machine code routine specified by addr.
    /// Note: this is not implemented.
    Sys(Address),                  // 0nnn - Sys addr

    /// Clears the display
    Cls,                        // 00E0 - CLS

    /// Return from a subroutine
    Return,                     // 00EE - RET

    /// Jumps to location nnn.
    Jump(Address),                 // 1nnn - JP addr

    /// Calls a subroutine at addr
    Call(Address),                 // 2nnn - CALL addr

    /// Skips the next instruction if Vx = Byte
    SkipEqualByte(Vx, Byte),         // 3xkk - SE Vx, byte

    /// Skips the next instruction if Vx != Byte
    SkipNotEqualByte(Vx, Byte),     // 4xkk - SNE Vx, byte

    /// Skips the next isntruction if Vx == Vy
    SkipEqualReg(Vx, Vy),           // 5xy0 - SE Vx, Vy

    /// Loads the value Byte into Register Vx.
    Load(Vx, Byte),                 // 6xkk - LD Vx, byte

    /// Sets the value of Vx to Vx + Byte
    Add(Vx, Byte),                  // 7xkk - ADD Vx, byte

    /// Stores the value of register Vy in register Vx.
    LoadFromReg(Vx, Vy),            // 8xy0 - LD Vx, Vy

    /// Stores the bitwise OR of Vx and Vy in Vx.
    Or(Vx, Vy),                     // 8xy1 - OR Vx, Vy

    /// Stores the bitwise And of Vx and Vy in Vx.
    And(Vx, Vy),                    // 8xy2 - AND Vx, Vy

    /// Stores the bitwise XOR of Vx and Vy in Vx.
    XOr(Vx, Vy),                    // 8xy3 - XOR Vx, Vy

    /// Stores the sum of the contents of Vx and Vy
    /// in Vx and sets VF to 1 if there result overflows.
    AddReg(Vx, Vy),                 // 8xy4 - ADD Vx, Vy

    /// Sets the register Vx to the difference of Vy from Vx.
    /// If Vx > Vy then VF is set to 1
    /// otherwise 0
    SubReg(Vx, Vy),                 // 8xy5 - SUB Vx, Vy

    /// Sets the register Vx to the result of shifting Vx right 1.
    /// If the LSB of Vx is 1, VF = 1.
    ShiftRight(Vx),                 // 8xy6 - SHR Vx {, Vy}

    /// Sets the register Vx to the difference of Vx from Vy
    /// If Vy > Vx then VF is set to 1,
    /// otherwise 0
    ReverseSub(Vx, Vy),             // 8xy7 - SUBN Vx, Vy

    /// Sets the register Vx to the result of shfit the contents of Vx left 1.
    ShiftLeft(Vx),                  // 8xy8 - SHL Vx {, Vy}

    /// Skip the next instruction if the contents of Vx and Vy are not equal.
    SkipNotEqualReg(Vx, Vy),        // 9xy0 - SNE Vx, Vy

    /// Stores the provided address in the address register.
    LoadInstr(Address),                // Annn - LD I, Addr

    /// Jump to the location determined by V0 + Addr.
    JumpPlus(Address),                 // Bnnn - JP V0, addr

    /// Sets Vx to the bitwise and of a random number between 0 and 255
    /// and Byte.
    Rand(Vx, Byte),                 // Cxkk - RNX Vx, byte

    /// Draws an n-byte sprite starting at memory location I at position
    /// (Vx, Vy). Sets VF to 1 if there is a collision.
    Draw(Vx, Vy, Nibble),           // Dxyn - DRW Vx, Vy, nibble

    /// Skips the next instruction if the key with value Vx is pressed.
    SkipPressed(Vx),                // Ex9E - SKP Vx

    /// Skip the next instruction if the key with value Vx is not pressed.
    SkipNotPressed(Vx),             // ExA1 - SKNP Vx

    /// Loads the value of the delay timer into Vx.
    LoadDelayTimer(Vx),              // Fx07 - LD Vx, DT

    /// Wait for a key press and store the value of the key in Vx.
    WaitForKey(Vx),                 // Fx0A - LD Vx, K

    /// Sets the delay timer to the value contained in register Vx.
    SetDelayTimer(Vx),              // Fx15 - LD DT, Vx

    /// Sets the sound timer to the value container in register Vx.
    SetSoundTimer(Vx),              // Fx18 - LD ST, Vx

    /// Increments the address register by the contents of Vx
    IncrementAddress(Vx),           // Fx1E - ADD I, Vx

    /// Sets the address register to the location of the sprite
    /// corresponding to the value of Vx.
    LoadSpriteAddress(Vx),          // Fx29 - LD F, Vx

    /// Loads the BCD represtation of Vx in memory locations I, I + 1, I + 2
    LoadBCD(Vx),                    // Fx33 - LD B, Vx

    /// Copies the contents of Registers V0 to Vx to Memory
    /// starting at I
    CopyToRam(Vx),                  // Fx55 - LD [I], Vx

    /// Copies values from memory starting at I
    /// into registers V0 through Vx.
    CopyToRegisters(Vx),            // Fx65 - LD Vx, [I]
}

impl Instruction {
    /// Creates a new instruction from a raw 16 bit instruction.
    pub fn new (instr: RawInstruction) -> Instruction {

        use Instruction::*;

        let (
            addr,
            vx, vy,
            byte,
            nibble
        ) = utils::parse_raw_instruction(instr);

        match instr {
            0x00E0 => Cls,
            0x00EE => Return,

            _ => match instr & 0xF000 {
                0x0000 => Sys(addr),
                0x1000 => Jump(addr),
                0x2000 => Call(addr),
                0x3000 => SkipEqualByte(vx, byte),
                0x4000 => SkipNotEqualByte(vx, byte),
                0x5000 => SkipEqualReg(vx, vy),
                0x6000 => Load(vx, byte),
                0x7000 => Add(vx, byte),
                0x8000 => match instr & 0x000F {
                    0x0000 => LoadFromReg(vx, vy),
                    0x0001 => Or(vx, vy),
                    0x0002 => And(vx, vy),
                    0x0003 => XOr(vx, vy),
                    0x0004 => AddReg(vx, vy),
                    0x0005 => SubReg(vx, vy),
                    0x0006 => ShiftRight(vx),
                    0x0007 => ReverseSub(vx, vy),
                    0x000E => ShiftLeft(vx),
                    _ => Invalid(instr)
                },
                0x9000 => SkipNotEqualReg(vx, vy),
                0xA000 => LoadInstr(addr),
                0xB000 => JumpPlus(addr),
                0xC000 => Rand(vx, byte),
                0xD000 => Draw(vx, vy, nibble),
                0xE000 => match instr & 0x00FF {
                    0x009E => SkipPressed(vx),
                    0x00A1 => SkipNotPressed(vx),
                    _ => Invalid(instr)
                },
                0xF000 => match instr & 0x00FF {
                    0x0007 => LoadDelayTimer(vx),
                    0x000A => WaitForKey(vx),
                    0x0015 => SetDelayTimer(vx),
                    0x0018 => SetSoundTimer(vx),
                    0x001E => IncrementAddress(vx),
                    0x0029 => LoadSpriteAddress(vx),
                    0x0033 => LoadBCD(vx),
                    0x0055 => CopyToRam(vx),
                    0x0065 => CopyToRegisters(vx),
                    _ => Invalid(instr)
                },
                _ => Invalid(instr)
            }
        }
    }
}
