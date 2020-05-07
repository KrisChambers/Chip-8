//! Test-Emulator for the Chip-8.

use std::io::{stdout, Write};

extern crate termion;
use termion::{
    event::{Event, Key},
    input::TermRead,
    raw::IntoRawMode,
};

extern crate cpu;
extern crate framebuffer;
extern crate keyboard;
extern crate memory;
extern crate model;
extern crate program_counter;
extern crate register_bank;

// We want all traits from the model in scope.
use model::*;

use cpu::VirtualMachine;
use framebuffer::FrameBuffer;
use keyboard::Keyboard;
use memory::Memory;
use program_counter::ProgramCounter;
use register_bank::RegisterBank;

/// Creates a new VirtualMachine.
///
fn get_vm() -> VirtualMachine<Memory, ProgramCounter, RegisterBank, FrameBuffer, Keyboard> {
    VirtualMachine::new(
        Memory::new(),
        ProgramCounter::new(0x200u16.into()),
        RegisterBank::new(),
        FrameBuffer::new(32),
        Keyboard::new(),
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

        draw_vm(&mut stdout, vm.get_framebuffer());

        stdout.flush().unwrap();
        std::thread::sleep(std::time::Duration::from_millis(10));
    }
}

/// Draws the framebuffer to the screen.
///
///### Arguments
///
///- **stdout** : The writer we want to write to.
///- **buffer** : The FrameBuffer to be written.
///
fn draw_vm<Writer: Write>(stdout: &mut Writer, buffer: &dyn Chip8FrameBuffer) {
    write!(stdout, "{}", format!("{:?}", buffer)).unwrap()
}
