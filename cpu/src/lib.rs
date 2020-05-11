extern crate data;
extern crate instruction;
extern crate model;
extern crate rand;

mod vm_state;

use data::{Address, Byte, Nibble};
use instruction::Instruction;
use model::{
    Chip8FrameBuffer, Chip8Keyboard, Chip8Memory, Chip8ProgramCounter, Chip8RegisterBank,
    Chip8VirtualMachine, Register,
};
use rand::Rng;

/// System fonts in byte form.
///
pub const FONTS: [u8; 80] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
    0x20, 0x60, 0x20, 0x20, 0x70, // 1
    0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
    0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
    0x90, 0x90, 0xF0, 0x10, 0x10, // 4
    0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
    0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
    0xF0, 0x10, 0x20, 0x40, 0x40, // 7
    0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
    0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
    0xF0, 0x90, 0xF0, 0x90, 0x90, // A
    0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
    0xF0, 0x80, 0x80, 0x80, 0xF0, // C
    0xE0, 0x90, 0x90, 0x90, 0xE0, // D
    0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
    0xF0, 0x80, 0xF0, 0x80, 0x80, // F
];

/// An implementation of the Chip8 virtual machine.
///
pub struct VirtualMachine<
    M: Chip8Memory,
    PC: Chip8ProgramCounter,
    R: Chip8RegisterBank,
    FB: Chip8FrameBuffer,
    KB: Chip8Keyboard,
> {
    memory: M,
    pc: PC,
    registers: R,
    framebuffer: FB,
    keyboard: KB,
    delay_timer: u8,
    sound_timer: u8,
}

impl<M, PC, R, FB, KB> VirtualMachine<M, PC, R, FB, KB>
where
    M: Chip8Memory,
    PC: Chip8ProgramCounter,
    R: Chip8RegisterBank,
    FB: Chip8FrameBuffer,
    KB: Chip8Keyboard,
{
    /// Constructs a new VirtualMachine
    ///
    pub fn new(memory: M, pc: PC, registers: R, framebuffer: FB, keyboard: KB) -> Self {
        let mut vm = VirtualMachine {
            memory,
            pc,
            registers,
            framebuffer,
            keyboard,
            delay_timer: 0,
            sound_timer: 0,
        };

        for i in 0..FONTS.len() {
            let byte = FONTS[i];
            let addr = Address::new(i as u16);

            vm.memory.set(addr, byte.into());
        }

        vm
    }

    /// Gets the next instruction.
    ///
    fn get_instr(&self) -> Instruction {
        let pc_addr = self.pc.current();
        let left: u8 = self.memory.get(pc_addr).into();
        let right: u8 = self.memory.get(pc_addr).into();

        Instruction::new((left as u16) << 8 | (right as u16))
    }

    /// Sets the carry register.
    ///
    /// Chip8 uses VF solely as a flag for carrying.
    ///
    ///### Arguments
    ///
    ///- **value** : The value being carried.
    ///
    fn set_carry<T: Into<Byte>>(&mut self, value: T) {
        self.set_reg(Register::VF, value.into());
    }

    /// Sets the value of the register.
    ///
    ///### Arguments
    ///
    ///- **r**      : The regiser to be set.
    ///- **value**  : The value to be set.
    ///
    fn set_reg(&mut self, r: Register, value: Byte) {
        self.registers.set_v(r, value);
    }

    /// Returns the contents of the register
    ///
    ///### Arguments
    ///
    ///- **r** : The register.
    ///
    fn get_reg(&self, r: Register) -> Byte {
        self.registers.get_v(r)
    }

    /// Returns a byte tuple.
    ///
    /// The bytes correspond to Vx and Vy respectively.
    ///
    fn get_regs(&mut self, rx: Register, ry: Register) -> (Byte, Byte) {
        (self.get_reg(rx), self.get_reg(ry))
    }

    /// Increments the current address by 2.
    ///
    fn inc_pc(&mut self) {
        self.pc.inc_by(2.into());
    }
}

impl<M, PC, R, FB, KB> Chip8VirtualMachine for VirtualMachine<M, PC, R, FB, KB>
where
    M: Chip8Memory,
    PC: Chip8ProgramCounter,
    R: Chip8RegisterBank,
    FB: Chip8FrameBuffer,
    KB: Chip8Keyboard,
{
    fn get_framebuffer(&self) -> &dyn Chip8FrameBuffer {
        &self.framebuffer
    }

    fn load_rom(&mut self, data: Vec<u8>) {
        let start_addr = self.pc.current();

        for i in 0..data.len() {
            let addr = start_addr + (i as u16);
            let data: Byte = data[i].into();

            self.memory.set(addr, data);
        }
    }

    fn press_key(&mut self, key: Nibble) {
        self.keyboard.press(key);
    }

    fn execute(&mut self) {
        self.execute_cycles(1);
    }

    fn execute_cycles(&mut self, cycles: usize) {
        use Instruction::*;

        let mut pcupdated = false;

        for _cycle in 0..cycles {
            let instruction = self.get_instr();

            match instruction {
                Sys(_addr) => {
                    // This is purposely left unimplemented.
                }

                Invalid(instr) => panic!("{} is not a valid instruction", instr),

                Cls => {
                    self.framebuffer.clear();
                }

                Return => {
                    self.pc.rtrn();
                }

                Jump(addr) => {
                    // We are decrementing by 2 to take into consideration that
                    // the program counter is updated later.
                    // Kinda hacky solution for now.
                    self.pc.set(addr);
                    pcupdated = true;
                }

                Call(addr) => {
                    // See Jump comment.
                    self.pc.to_subroutine(addr);
                    pcupdated = true;
                }

                SkipEqualByte(vx, byte) => {
                    let contents = self.get_reg(vx);

                    if contents == byte {
                        self.inc_pc();
                        pcupdated = true;
                    }
                }

                SkipNotEqualByte(vx, byte) => {
                    let contents = self.get_reg(vx);

                    if contents != byte {
                        self.inc_pc();
                        pcupdated = true;
                    }
                }

                SkipEqualReg(vx, vy) => {
                    let (x, y) = self.get_regs(vx, vy);

                    if x == y {
                        self.inc_pc();
                        pcupdated = true;
                    }
                }

                SkipNotEqualReg(vx, vy) => {
                    let (x, y) = self.get_regs(vx, vy);

                    if x != y {
                        self.inc_pc();
                        pcupdated = true;
                    }
                }

                Load(vx, byte) => {
                    self.set_reg(vx, byte);
                }

                LoadFromReg(vx, vy) => self.set_reg(vx, self.get_reg(vy)),

                Or(vx, vy) => {
                    let (x, y) = self.get_regs(vx, vy);

                    self.set_reg(vx, x | y)
                }

                And(vx, vy) => {
                    let (x, y) = self.get_regs(vx, vy);

                    self.set_reg(vx, x & y);
                }

                XOr(vx, vy) => {
                    let (x, y) = self.get_regs(vx, vy);

                    self.set_reg(vx, x ^ y);
                }

                AddReg(vx, vy) => {
                    let (x, y) = self.get_regs(vx, vy);

                    self.set_reg(vx, x.wrapping_add(y));
                }

                SubReg(vx, vy) => {
                    let (x, y) = self.get_regs(vx, vy);

                    if x > y {
                        self.set_carry(1);
                    }

                    self.set_reg(vx, x.wrapping_sub(y));
                }

                ShiftRight(vx) => {
                    let x = self.get_reg(vx);

                    self.set_carry(x.get_lsb());

                    self.set_reg(vx, x >> 1);
                }

                ReverseSub(vx, vy) => {
                    let (x, y) = self.get_regs(vx, vy);

                    if y > x {
                        self.set_carry(1);
                    } else {
                        self.set_carry(0);
                    }

                    self.set_reg(vx, y - x);
                }

                ShiftLeft(vx) => {
                    let x = self.get_reg(vx);

                    self.set_carry(x.get_msb());

                    self.set_reg(vx, x << 1);
                }

                LoadInstr(addr) => {
                    self.registers.set_i(addr);
                }

                Add(vx, byte) => {
                    let contents = self.get_reg(vx);
                    self.set_reg(vx, contents.wrapping_add(byte));
                }

                JumpPlus(addr) => {
                    let summand = self.get_reg(Register::V0);
                    let addr = addr + (summand.get_raw() as u16);

                    // NOTE: Need to double check the spec here.
                    // Not sure if we are jumping based on the current pc
                    // or what. Doesn't look to be the case though.
                    self.pc.set(addr);
                }

                Rand(vx, byte) => {
                    let mut rng = rand::thread_rng();

                    let n: Byte = {
                        let b: u8 = rng.gen();

                        b.into()
                    };

                    self.set_reg(vx, n & byte);
                }

                Draw(vx, vy, nibble) => {
                    let (x, y) = self.get_regs(vx, vy);

                    let i = self.registers.get_i();
                    let mem_slice = self.memory.get_slice(i, nibble);

                    self.framebuffer.draw(x, y, mem_slice);
                }

                SkipPressed(vx) => {
                    let key = self.registers.get_v(vx);
                    let n = Nibble::new(key.get_raw());

                    if self.keyboard.is_pressed(n) {
                        self.inc_pc();
                        self.inc_pc();
                        pcupdated = true;
                    };
                }

                SkipNotPressed(vx) => {
                    let key = self.registers.get_v(vx);
                    let n: Nibble = key.get_raw().into();

                    if !self.keyboard.is_pressed(n) {
                        self.inc_pc();
                        self.inc_pc();
                        pcupdated = true;
                    }
                }

                LoadDelayTimer(vx) => {
                    self.registers.set_v(vx, self.delay_timer.into());
                }

                WaitForKey(vx) => {
                    /* TODO:
                       An Idea. When this instruction is hit. We need to set a flag in the vm.
                       One of the variants would be WaitingForKey(Option<Nibble>).

                       In the execute_cycle function we then have the following bit of code.

                           if let WaitingForKey(Some(n)) = self.state {
                               if self.keyboard.is_pressed(n) {
                                   .. continue with execution
                               } else {
                                   .. continue waiting.
                                   return
                               }
                           }

                       Things to note:

                           1. After hitting the instruction the program counter will be set to the
                           next instruction that will be executed when the key is hit.

                           2. If multiple cycles are being executed at once, we only need to check if the
                           key is pressed once.

                           3. We need to move the actual instruction interpretation into a different
                           function.

                        Todo:
                           Implement:
                                pub enum VMState {
                                    Initializing,
                                    LoadingROM,
                                    LoadingFont,
                                    Executing
                                    Paused,
                                    WaitingForKey(Option<Nibble>),
                                }

                            Move the cycle execution into a function in impl VirtualMachine.
                                So execute_cycle should look something like:
                                    1. waiting?
                                    2. update state to execute
                                    3. run function that handles the execution



                    */
                    unimplemented!()
                }

                SetDelayTimer(vx) => {
                    self.delay_timer = self.registers.get_v(vx).get_raw();
                }

                SetSoundTimer(vx) => {
                    self.sound_timer = self.registers.get_v(vx).get_raw();
                }

                IncrementAddress(vx) => {
                    let i = self.registers.get_i();
                    let b = self.registers.get_v(vx);

                    self.registers.set_i(i + b);
                }

                LoadSpriteAddress(vx) => {
                    // We are loading the fonts starting at index 0 in the memory.
                    // (See the new function above.)
                    // Each font is 8 bits wide (1 byte) and 5 bits high.
                    // So 0 is at 0, 1 is at 5, 2 at 10. ie. digit * 5

                    // Firstly we only support 16 possible digits.
                    // So we pass it through Nibble to make sure it is in this range.
                    let byte = self.registers.get_v(vx);
                    let digit: u16 = Nibble::new(byte.into()).get_raw().into();

                    let address = Address::new(digit * 5);

                    self.registers.set_i(address)
                }

                LoadBCD(vx) => {
                    let x = self.registers.get_v(vx);
                    let i = self.registers.get_i();
                    let (h, t, o) = x.get_bcd_rep();

                    self.memory.set(i, h.into());
                    self.memory.set(i + 1u16, t.into());
                    self.memory.set(i + 2u16, o.into());
                }

                CopyToRam(vx) => {
                    // copy from V0 to Vx to memory starting at I.
                    let i = self.registers.get_i();

                    for reg in Register::iter_to(vx) {
                        let value = self.registers.get_v(reg);
                        let addr = i + (reg as u16);

                        self.memory.set(addr, value);
                    }
                }

                CopyToRegisters(vx) => {
                    let i = self.registers.get_i();

                    for reg in Register::iter_to(vx) {
                        let value = self.memory.get(i + (reg as u16));

                        self.registers.set_v(reg, value);
                    }
                }
            };

            if !pcupdated {
                self.pc.inc_by(2.into())
            }
        }
    }
}
