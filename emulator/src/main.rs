//! Test-Emulator for the Chip-8.

use termion::raw::RawTerminal;
use std::io::{stdout, Write, StdoutLock};

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
extern crate rom_library;

// We want all traits from the model in scope.
use model::*;

use cpu::VirtualMachine;
use framebuffer::FrameBuffer;
use keyboard::Keyboard;
use memory::Memory;
use program_counter::ProgramCounter;
use register_bank::RegisterBank;
use rom_library::load_rom;

type VM = VirtualMachine<Memory, ProgramCounter, RegisterBank, FrameBuffer, Keyboard>;

/// Creates a new VirtualMachine.
///
fn get_vm() -> VM {
    VM::new(
        Memory::new(),
        ProgramCounter::new(0x200u16.into()),
        RegisterBank::new(),
        FrameBuffer::new(32),
        Keyboard::new(),
    )
}

fn main() {
    let mut events = termion::async_stdin().events();
    let stdout = stdout();
    let mut stdout = stdout.lock().into_raw_mode().unwrap();

    let mut vm = get_vm();

    let rom = load_rom("test_opcode".into()).unwrap();

    vm.load_rom(rom);

    write!(stdout, "{}", termion::clear::All).unwrap();

    loop {
        clear(&mut stdout);

        write!(stdout, "{}", termion::cursor::Goto(1, 2)).unwrap();

        if let Some(Ok(Event::Key(Key::Char(c)))) = events.next() {
            if c == '`' { clear(&mut stdout); break; }

            if let Some(key) = get_chip8_key(c) {
                vm.press_key(key.into());
            }
        }

        vm.execute();

        draw_vm(&mut stdout, vm.get_framebuffer());
        log_state(&mut stdout, &vm);

        stdout.flush().unwrap();
        std::thread::sleep(std::time::Duration::from_millis(100));
    }
}

fn clear(stdout: &mut RawTerminal<StdoutLock>) {
    write!(
        stdout,
        "{}",
        termion::clear::All,
    )
    .unwrap();
}

fn log_state(stdout: &mut RawTerminal<StdoutLock>, vm: &VM) {
    write!(stdout,
        "{}{:?} : {:?}",
        termion::cursor::Goto(1,1), vm.state, vm.pc.current()
    ).unwrap();
}

/// Draws the framebuffer to the screen.
///
///### Arguments
///
///- **stdout** : The writer we want to write to.
///- **buffer** : The FrameBuffer to be written.
///
fn draw_vm<Writer: Write>(stdout: &mut Writer, buffer: &dyn Chip8FrameBuffer) {
    let pixels: &[u64] = buffer;
    let len = pixels.len();

    for i in 0 .. len {
        let pixel = pixels[len - 1 - i];
        let y: u16 = (i as u16) + 2;

        let line = format!("{:064b}", pixel)
            .replace("0", " ")
            .replace("1", "X");

        write!(stdout,
            "{}{}",
            termion::cursor::Goto(1, y),
            line
        ).unwrap();

    }
}

fn get_chip8_key(input: char) -> Option<u8> {
    match input {
        '1' => Some(0x1),
        '2' => Some(0x2),
        '3' => Some(0x3),
        '4' => Some(0xC),
        'q' => Some(0x4),
        'w' => Some(0x5),
        'e' => Some(0x6),
        'r' => Some(0xD),
        'a' => Some(0x7),
        's' => Some(0x8),
        'd' => Some(0x9),
        'f' => Some(0xE),
        'z' => Some(0xA),
        'x' => Some(0x0),
        'c' => Some(0xB),
        'v' => Some(0xF),
        _   => None
    }
}

fn get_actual_key(input: u8) -> char {
    match input {
        0x1 => '1',
        0x2 => '2',
        0x3 => '3',
        0xC => '4',
        0x4 => 'q',
        0x5 => 'w',
        0x6 => 'e',
        0xD => 'r',
        0x7 => 'a',
        0x8 => 's',
        0x9 => 'd',
        0xE => 'f',
        0xA => 'z',
        0x0 => 'x',
        0xB => 'c',
        0xF => 'v',
        _ => { ' ' }
    }
}