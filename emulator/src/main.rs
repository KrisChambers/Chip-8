extern crate termion;

use termion::event::{ Event, Key };
use termion::input::TermRead;
use termion::raw::IntoRawMode;

use std::{io::{ stdout, Write }};

fn main() {
    let mut stdin = termion::async_stdin().events();
    let stdout = stdout();
    let mut stdout = stdout.lock().into_raw_mode().unwrap();

    let mut count = 0;

    write!(stdout, "{}", termion::clear::All).unwrap();

    loop {
        write!(stdout, "{}{}", termion::cursor::Goto(1,1), count).unwrap();
        write!(stdout, "{}", termion::cursor::Goto(1,2)).unwrap();
        stdout.flush().unwrap();
        count += 1;
      
        if let Some(Ok(Event::Key(Key::Char(c)))) = stdin.next() {
            match c {
                'q' => break,
                _ => println!("{}", c)
            };
        }
  }
}
