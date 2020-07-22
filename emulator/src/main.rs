//! Test-Emulator for the Chip-8.

use sdl2::render::WindowCanvas;

extern crate sdl2;
use sdl2::{event::Event, keyboard::Keycode, pixels::Color, rect::Rect};

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
    let cpu_hz: usize = 480;
    let frames_per_second: usize = 60;
    let cycles_per_frame = cpu_hz / frames_per_second;

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("chip - 8", 640, 320)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    // Background color
    canvas.set_draw_color(sdl2::pixels::Color::RGB(0, 15, 15));
    canvas.clear();
    canvas.present(); // Kind of like flushing the buffer?

    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut vm = get_vm();
    let rom = load_rom("tetris".into()).unwrap();
    vm.load_rom(&rom);

    'running: loop {
        let mut cycles = cycles_per_frame;

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                Event::KeyDown {
                    keycode: Some(keycode),
                    ..
                } => {
                    process_keycode(keycode, &mut vm);
                }
                Event::KeyUp { .. } => {
                    vm.keyboard.clear();
                }
                _ => {}
            }

            if cycles > 0 {
                vm.execute();
                cycles -= 1;
            }
        }


        if cycles > 0 {
            vm.execute_cycles(cycles)
        }

        // This can happen on the main thread.
        draw_vm(&mut canvas, vm.get_framebuffer());

        std::thread::sleep(std::time::Duration::new(
            0,
            1_000_000_000u32 / (frames_per_second as u32),
        ));
    }
}

/// Calls press_key on the vm for the Chip-8 key corresponding
/// to a SDL Keycode.
///
///### Arguments
///
///- **keycode**    : The SDL Keycode that is pressed.
///- **vm**         : The Chip8VirtualMachine
///
fn process_keycode(keycode: Keycode, vm: &mut VM) {
    match keycode {
        Keycode::Num1 => {
            vm.press_key(0x1);
        }
        Keycode::Num2 => {
            vm.press_key(0x2);
        }
        Keycode::Num3 => {
            vm.press_key(0x3);
        }
        Keycode::Num4 => {
            vm.press_key(0xC);
        }
        Keycode::Q => {
            vm.press_key(0x4);
        }
        Keycode::W => {
            vm.press_key(0x5);
        }
        Keycode::E => {
            vm.press_key(0x6);
        }
        Keycode::R => {
            vm.press_key(0xD);
        }
        Keycode::A => {
            vm.press_key(0x7);
        }
        Keycode::S => {
            vm.press_key(0x8);
        }
        Keycode::D => {
            vm.press_key(0x9);
        }
        Keycode::F => {
            vm.press_key(0xE);
        }
        Keycode::Z => {
            vm.press_key(0xA);
        }
        Keycode::X => {
            vm.press_key(0x0);
        }
        Keycode::C => {
            vm.press_key(0xB);
        }
        Keycode::V => {
            vm.press_key(0xF);
        }
        _ => {}
    }
}

/// Background color for the emulator.
///
static BACKGROUND_COLOR: Color = Color::RGB(0, 15, 15);

/// Foreground Color.
///
static FOREGROUND_COLOR: Color = Color::RGB(128, 0, 128);

/// Draws the framebuffer to a canvas.
///
///### Arguments
///
///- **canvas** : The canvas we are drawing to.
///- **buffer** : The FrameBuffer to be written.
///
fn draw_vm(canvas: &mut WindowCanvas, buffer: &dyn Chip8FrameBuffer) {
    // This handles drawing the background.
    canvas.set_draw_color(BACKGROUND_COLOR);
    canvas.clear();

    // Set the pixel color.
    canvas.set_draw_color(FOREGROUND_COLOR);

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
