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
use vm_state::VMState;

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
    pub memory: M,
    pub pc: PC,
    pub registers: R,
    pub framebuffer: FB,
    pub keyboard: KB,
    pub state: VMState,
    pub delay_timer: u8,
    pub sound_timer: u8,
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
            state: VMState::Initializing, // TODO: What should be the initial state?
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
        let right: u8 = self.memory.get(pc_addr + (1 as u16)).into();

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

    /// Returns a flag indicating if the vm is waiting for a key press.
    ///
    fn is_waiting(&self) -> bool {
        if let VMState::WaitingForKey(Some(n)) = self.state {
            !self.keyboard.is_pressed(n)
        } else {
            false
        }
    }

    /// Returns a flag indicating if the vm is paused.
    ///
    fn is_paused(&self) -> bool {
        self.state == VMState::Paused
    }

    /// Decrements delay and sound timers.
    ///
    fn decrement_timers(&mut self) {
        if self.delay_timer > 0 {
            self.delay_timer -= 1;
        }

        if self.sound_timer > 0 {
            self.sound_timer -= 1;
        }
    }

    /// Performs the instruction. Returns a flag indicating if the program counter
    /// was updated.
    ///
    ///### Arguments
    ///
    ///- **instruction** : The Instruction being executed.
    ///
    fn interpret_instruction(&mut self, instruction: Instruction) -> bool {
        use Instruction::*;

        match instruction {
            Sys(_addr) => {
                // This is purposely left unimplemented.
            }

            Invalid(instr) => println!("{:#X} is not a valid instruction", instr),

            Cls => {
                self.framebuffer.clear();
            }

            Return => {
                self.pc.rtrn();
            }

            Jump(addr) => {
                self.pc.set(addr);
                return true;
            }

            Call(addr) => {
                // See Jump comment.
                self.pc.to_subroutine(addr);
                return true;
            }

            SkipEqualByte(vx, byte) => {
                let contents = self.get_reg(vx);

                if contents == byte {
                    self.inc_pc();
                    self.inc_pc();
                    return true;
                }
            }

            SkipNotEqualByte(vx, byte) => {
                let contents = self.get_reg(vx);

                if contents != byte {
                    self.inc_pc();
                    self.inc_pc();
                    return true;
                }
            }

            SkipEqualReg(vx, vy) => {
                let (x, y) = self.get_regs(vx, vy);

                if x == y {
                    self.inc_pc();
                    self.inc_pc();
                    return true;
                }
            }

            SkipNotEqualReg(vx, vy) => {
                let (x, y) = self.get_regs(vx, vy);

                if x != y {
                    self.inc_pc();
                    self.inc_pc();
                    return true;
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

                // Check if there was a collision
                if self.framebuffer.draw(x, y, mem_slice) {
                    self.set_carry(1);
                } else {
                    self.set_carry(0);
                }
            }

            SkipPressed(vx) => {
                let key = self.registers.get_v(vx);
                let n = Nibble::new(key.get_raw());

                if self.keyboard.is_pressed(n) {
                    self.inc_pc();
                    self.inc_pc();
                    return true;
                };
            }

            SkipNotPressed(vx) => {
                let key = self.registers.get_v(vx);
                let n: Nibble = key.get_raw().into();

                if !self.keyboard.is_pressed(n) {
                    self.inc_pc();
                    self.inc_pc();
                    return true;
                }
            }

            LoadDelayTimer(vx) => {
                self.registers.set_v(vx, self.delay_timer.into());
            }

            WaitForKey(vx) => {
                let digit = self.registers.get_v(vx).get_raw();
                let nib = Nibble::from(digit);

                self.state = VMState::WaitingForKey(Some(nib));
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

                self.registers.set_i(i + (vx as u16) + (1 as u16));
            }

            CopyToRegisters(vx) => {
                let i = self.registers.get_i();

                for reg in Register::iter_to(vx) {
                    let value = self.memory.get(i + (reg as u16));

                    self.registers.set_v(reg, value);
                }

                self.registers.set_i(i + (vx as u16) + (1 as u16));
            }
        };

        false
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

    fn load_rom(&mut self, data: &[u8]) {
        self.state = VMState::LoadingROM;

        let start_addr = self.pc.current();

        for i in 0..data.len() {
            let addr = start_addr + (i as u16);
            let data: Byte = data[i].into();

            self.memory.set(addr, data);
        }

        self.state = VMState::Initializing;
    }

    fn press_key(&mut self, key: u8) {
        self.keyboard.press(Nibble::from(key));
    }

    fn execute(&mut self) {
        self.execute_cycles(1);
    }

    fn execute_cycles(&mut self, cycles: usize) {
        for _ in 0..cycles {
            self.decrement_timers();

            if self.is_waiting() || self.is_paused() {
                return;
            }

            let instruction = self.get_instr();

            self.state = VMState::Executing(instruction);

            if !self.interpret_instruction(instruction) {
                self.inc_pc();
            }
        }
    }

    fn release_keys(&mut self) {
        self.keyboard.clear();
    }
}
