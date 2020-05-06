extern crate data;
extern crate model;
extern crate instruction;
extern crate rand;

use model::{
    Chip8VirtualMachine,
    Chip8Memory,
    Chip8ProgramCounter,
    Chip8RegisterBank,
    Chip8FrameBuffer,
    Register
};
use instruction::Instruction;
use data::Byte;
use rand::Rng;

/// An implementation of the Chip8 virtual machine.
///
pub struct VirtualMachine<
    M   : Chip8Memory,
    PC  : Chip8ProgramCounter,
    R   : Chip8RegisterBank,
    FB  : Chip8FrameBuffer
> {
    memory: M,
    pc: PC,
    registers: R,
    framebuffer: FB
}

impl<M, PC, R, FB> VirtualMachine<M, PC, R, FB>
    where
        M   : Chip8Memory,
        PC  : Chip8ProgramCounter,
        R   : Chip8RegisterBank,
        FB  : Chip8FrameBuffer
{
    /// Constructs a new VirtualMachine
    ///
    pub fn new(
        memory: M,
        pc: PC,
        registers: R,
        framebuffer: FB
    ) -> Self
    where M : Chip8Memory,
          PC: Chip8ProgramCounter,
          R : Chip8RegisterBank,
          FB: Chip8FrameBuffer
    {
        VirtualMachine {
            memory,
            pc,
            registers,
            framebuffer
        }
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

impl<M, PC, R, FB> Chip8VirtualMachine for VirtualMachine<M, PC, R, FB>
where
    M : Chip8Memory,
    PC: Chip8ProgramCounter,
    R : Chip8RegisterBank,
    FB: Chip8FrameBuffer

{
    fn execute(&mut self) {
        self.execute_cycles(1);
    }

    fn get_framebuffer(&self) -> &dyn Chip8FrameBuffer {
        &self.framebuffer
    }

    fn load_rom(&mut self, _rom_data: Vec<u8>) { unimplemented!() }

    fn execute_cycles(&mut self, cycles: usize) {
        use Instruction::*;

        let mut pcupdated = false;

        for _cycle in 0..cycles {
            let instruction = self.get_instr();

            match instruction {
                Sys(_addr) => {
                    // This is purposely left unimplemented.
                },
                Invalid(instr) => panic!("{} is not a valid instruction", instr),
                Cls => {
                    self.framebuffer.clear();
                },
                Return => {
                    self.pc.rtrn();
                },
                Jump(addr) => {
                    // We are decrementing by 2 to take into consideration that
                    // the program counter is updated later.
                    // Kinda hacky solution for now.
                    self.pc.set(addr);
                    pcupdated = true;
                },
                Call(addr) => {
                    // See Jump comment.
                    self.pc.to_subroutine(addr);
                    pcupdated = true;
                },
                SkipEqualByte(vx, byte) => {
                    let contents = self.get_reg(vx);
    
                    if contents == byte {
                        self.inc_pc();
                        pcupdated = true;
                    }
                },
                SkipNotEqualByte(vx, byte) => {
                    let contents = self.get_reg(vx);
    
                    if contents != byte {
                        self.inc_pc();
                        pcupdated = true;
                    }
                },
                SkipEqualReg(vx, vy) => {
                    let (x, y) = self.get_regs(vx, vy);
    
                    if x == y {
                        self.inc_pc();
                        pcupdated = true;
                    }
                },
                SkipNotEqualReg(vx, vy) => {
                    let (x, y) = self.get_regs(vx, vy);
    
                    if x != y {
                        self.inc_pc();
                        pcupdated = true;
                    }
                },
                Load(vx, byte) => {
                    self.set_reg(vx, byte);
                },
                LoadFromReg(vx, vy) => {
                    self.set_reg(vx, self.get_reg(vy))
                },
                Or(vx, vy) => {
                    let (x, y) = self.get_regs(vx, vy);
    
                    self.set_reg(vx, x | y)
                },
                And(vx, vy) => {
                    let (x, y) = self.get_regs(vx, vy);
    
                    self.set_reg(vx, x & y);
                },
                XOr(vx, vy) => {
                    let (x, y) = self.get_regs(vx, vy);
    
                    self.set_reg(vx, x ^ y);
                },
                AddReg(vx, vy) => {
                    let (x, y) = self.get_regs(vx, vy);
    
                    self.set_reg(vx, x.wrapping_add(y));
                },
                SubReg(vx, vy) => {
                    let (x, y) = self.get_regs(vx, vy);
    
                    if x > y {
                        self.set_carry(1);
                    }
    
                    self.set_reg(vx, x.wrapping_sub(y));
                },
                ShiftRight(vx) => {
                    let x = self.get_reg(vx);
    
                    self.set_carry(x.get_lsb());
    
                    self.set_reg(vx, x >> 1);
                },
                ReverseSub(vx, vy) => {
                    let (x, y) = self.get_regs(vx, vy);
    
                    if y > x { self.set_carry(1); }
                    else { self.set_carry(0); }
    
                    self.set_reg(vx, y - x);
                },
                ShiftLeft(vx) => {
                    let x = self.get_reg(vx);
    
                    self.set_carry(x.get_msb());
    
                    self.set_reg(vx, x << 1);
                },
                LoadInstr(addr) => {
                    self.registers.set_i(addr);
                },
                Add(vx, byte) => {
                    let contents = self.get_reg(vx);
                    self.set_reg(vx, contents.wrapping_add(byte));
                },
                JumpPlus(addr) => {
                    let summand = self.get_reg(Register::V0);
                    let addr = addr + (summand.get_raw() as u16);

                    // NOTE: Need to double check the spec here.
                    // Not sure if we are jumping based on the current pc
                    // or what. Doesn't look to be the case though.
                    self.pc.set(addr);
                },
                Rand(vx, byte) => {
                    let mut rng = rand::thread_rng();
    
                    let n: Byte = {
                        let b: u8 = rng.gen();

                        b.into()
                    };
    
                    self.set_reg(vx, n & byte);
                },
                Draw(vx, vy, nibble) => {
                    let (x, y) = self.get_regs(vx, vy);
    
                    let i = self.registers.get_i();
                    let mem_slice = self.memory.get_slice(i, nibble);
    
                    self.framebuffer.draw(x, y, mem_slice);
                },
                SkipPressed(vx) => {
                    unimplemented!()
                },
                SkipNotPressed(vx) => {
                    unimplemented!()
                },
                LoadDelayTimer(vx) => {
                    unimplemented!()
                },
                WaitForKey(vx) => {
                    unimplemented!()
                },
                SetDelayTimer(vx) => {
                    unimplemented!()
                },
                SetSoundTimer(vx) => {
                    unimplemented!()
                },
                
                IncrementAddress(vx) => {
                    unimplemented!()
                },
                LoadSpriteAddress(vx) => {
                    unimplemented!()
                },
                LoadBCD(vx) => {
                    let x = self.registers.get_v(vx);
                    let i = self.registers.get_i();
                    let (h, t, o) = x.get_bcd_rep();
    
                    self.memory.set(i, h.into());
                    self.memory.set(i + 1u16, t.into());
                    self.memory.set(i + 2u16, o.into());
                },
                CopyToRam(vx) =>{
                    // copy from V0 to Vx to memory starting at I.
                    let i = self.registers.get_i();
                    
                    for reg in Register::iter_to(vx) {
                        let value = self.registers.get_v(reg);
                        let addr = i + (reg as u16);
    
                        self.memory.set(addr, value);
                    }
                },
                CopyToRegisters(vx) => {
                    let i = self.registers.get_i();
    
                    for reg in Register::iter_to(vx) {
                        let value = self.memory.get(i + (reg as u16));
    
                        self.registers.set_v(reg, value);
                    }
                }
            };

            if !pcupdated { self.pc.inc_by(2.into())}
        }
    }
}
