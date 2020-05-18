//! Test-Emulator for the Chip-8.

use sdl2::render::WindowCanvas;

extern crate sdl2;
use sdl2::{
    pixels::Color,
    event::Event,
    keyboard::Keycode,
    rect::Rect,
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
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("chip8", 640, 320)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    // Background color
    canvas.set_draw_color(sdl2::pixels::Color::RGB(0, 15, 15));
    canvas.clear();
    canvas.present();   // Kind of like flushing the buffer?

    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut i = 0;
    let mut vm = get_vm();
    let rom = load_rom("breakout".into()).unwrap();
    vm.load_rom(rom);

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                Event::KeyDown { keycode: Some(Keycode::Num1), ..} => {
                    vm.press_key(0x1.into());
                },
                Event::KeyDown { keycode: Some(Keycode::Num2), ..} => {
                    vm.press_key(0x2.into());
                },
                Event::KeyDown { keycode: Some(Keycode::Num3), ..} => {
                    vm.press_key(0x3.into());
                },
                Event::KeyDown { keycode: Some(Keycode::Num4), ..} => {
                    vm.press_key(0xC.into());
                },
                Event::KeyDown { keycode: Some(Keycode::Q), ..} => {
                    vm.press_key(0x4.into());
                },
                Event::KeyDown { keycode: Some(Keycode::W), ..} => {
                    vm.press_key(0x5.into());
                },
                Event::KeyDown { keycode: Some(Keycode::E), ..} => {
                    vm.press_key(0x6.into());
                },
                Event::KeyDown { keycode: Some(Keycode::R), ..} => {
                    vm.press_key(0xD.into());
                },
                Event::KeyDown { keycode: Some(Keycode::A), ..} => {
                    vm.press_key(0x7.into());
                },
                Event::KeyDown { keycode: Some(Keycode::S), ..} => {
                    vm.press_key(0x8.into());
                },
                Event::KeyDown { keycode: Some(Keycode::D), ..} => {
                    vm.press_key(0x9.into());
                },
                Event::KeyDown { keycode: Some(Keycode::F), ..} => {
                    vm.press_key(0xE.into());
                },
                Event::KeyDown { keycode: Some(Keycode::Z), ..} => {
                    vm.press_key(0xA.into());
                },
                Event::KeyDown { keycode: Some(Keycode::X), ..} => {
                    vm.press_key(0x0.into());
                },
                Event::KeyDown { keycode: Some(Keycode::C), ..} => {
                    vm.press_key(0xB.into());
                }
                Event::KeyDown { keycode: Some(Keycode::V), ..} => {
                    vm.press_key(0xF.into());
                },
                _ => { }
            }
        } // Put into another function.

        vm.execute();



        draw_vm(&mut canvas, vm.get_framebuffer());
        //canvas.set_draw_color(Color::RGB(10, 180, 10));
        canvas.fill_rect(Rect::new(0, 0, 10, 10)).unwrap();

        std::thread::sleep(std::time::Duration::new(0, 250_000_000u32 / 60));
    }

/*
fn log_state(stdout: &mut RawTerminal<StdoutLock>, vm: &VM, cycle_time: u128) {
    write!(stdout,
        "{}{}{:?}{:?} : {:?} : {:?} : {:?}",
        termion::cursor::Goto(1,1),
        termion::clear::CurrentLine,
        cycle_time,
        vm.state, vm.pc.current(), vm.keyboard, vm.delay_timer
    ).unwrap();
}
*/

/// Draws the framebuffer to a canvas.
///
///### Arguments
///
///- **canvas** : The canvas we are drawing to.
///- **buffer** : The FrameBuffer to be written.
///
fn draw_vm(canvas: &mut WindowCanvas, buffer: &dyn Chip8FrameBuffer) {

    // This handles drawing the background.
    canvas.set_draw_color(Color::RGB(0, 15, 15));
    canvas.clear();

    // Set the pixel color.
    canvas.set_draw_color(Color::RGB(0, 200, 0));

    let pixels: &[u64] = buffer;
    let len = pixels.len();

    for i in 0..len {
        let line = pixels[len - 1 - i];

        let y = i as i32;

        for x in 0..64 {
            let pixel_mask: u64 = 1 << (63 - x);

            if line & pixel_mask > 0 {
                canvas.fill_rect(Rect::new(10 * x, 10 * y, 10, 10)).unwrap();
            }
        }
    }

    canvas.present();
}