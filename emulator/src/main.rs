//! Test-Emulator for the Chip-8.

use std::io::{stdout, Write};

extern crate termion;
use termion::{
    event::{Event, Key},
    input::TermRead,
    raw::IntoRawMode,
};

extern crate framebuffer;
use framebuffer::FrameBuffer;

fn main() {
    let mut stdin = termion::async_stdin().events();
    let stdout = stdout();
    let mut stdout = stdout.lock().into_raw_mode().unwrap();

    let buffer = FrameBuffer::new(32);

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
                'q' => break,
                _ => {}
            };
        }

        write!(stdout, "{}", format!("{:?}", buffer)).unwrap();
        stdout.flush().unwrap();
        std::thread::sleep(std::time::Duration::from_millis(10));
    }
}
