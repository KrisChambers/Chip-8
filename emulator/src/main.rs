extern crate termion;
extern crate framebuffer;

use termion::event::{Event, Key};
use termion::input::TermRead;
use termion::raw::IntoRawMode;

use framebuffer::FrameBuffer;

use std::io::{stdout, Write};

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
        ).unwrap();

        write!(stdout, "{}", termion::cursor::Goto(1, 2)).unwrap();

        count += 1;

        if let Some(Ok(Event::Key(Key::Char(c)))) = stdin.next() {
            match c {
                'q' => break,
                _ => {}
            };
        }

        write!(stdout, "{}", buffer_as_string(&buffer)).unwrap();
        stdout.flush().unwrap();
        std::thread::sleep(std::time::Duration::from_millis(10));
    }
}

fn buffer_as_string(buffer: &FrameBuffer) -> String {
    buffer
        .iter()
        .map(|line| format!("{:064b}", line))
        .fold(String::with_capacity(64 * 32), |acc, line| format!("{}{}\r\n", acc, line))
}

#[test]
fn line_test() {
    println!("{:06b}", 0);
}
