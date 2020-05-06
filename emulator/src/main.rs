//! Test-Emulator for the Chip-8.

use std::io::{stdout, Write};

extern crate termion;
use termion::{
    event::{Event, Key},
    input::TermRead,
    raw::IntoRawMode,
};

extern crate cpu;
extern crate memory;
extern crate register_bank;
extern crate program_counter;
extern crate framebuffer;
extern crate model;

// We want all traits from the model in scope.
use model::*;

use cpu::VirtualMachine;
use memory::Memory;
use register_bank::_Chip8RegisterBank;
use program_counter::ProgramCounter;
use framebuffer::FrameBuffer;

fn get_vm () -> VirtualMachine<Memory, ProgramCounter, _Chip8RegisterBank, FrameBuffer> {
    VirtualMachine::new(
        Memory::new(),
        ProgramCounter::new(0x200u16.into()),
        _Chip8RegisterBank::new(),
        FrameBuffer::new(32)
    )
}

fn main() {
    let mut stdin = termion::async_stdin().events();
    let stdout = stdout();
    let mut stdout = stdout.lock().into_raw_mode().unwrap();

    let vm = get_vm();

    let mut count = 0;

    write!(stdout, "{}", termion::clear::All).unwrap();

    loop {
        write!(
            stdout,
            "{}{}{}",
            termion::clear::All,
            termion::cursor::Goto(1, 1),
            count
        )
        .unwrap();

        write!(stdout, "{}", termion::cursor::Goto(1, 2)).unwrap();

        count += 1;

        if let Some(Ok(Event::Key(Key::Char(c)))) = stdin.next() {
            match c {
                '`' => break,
                _ => {}
            };
        }

        write!(stdout, "{}", format!("{:?}", vm.get_framebuffer())).unwrap();
        stdout.flush().unwrap();
        std::thread::sleep(std::time::Duration::from_millis(10));
    }
}
