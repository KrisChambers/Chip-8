extern crate data;
extern crate model;

use data::Nibble;
use model::Chip8Keyboard;

#[derive(Debug)]
pub struct Keyboard {
    pressed: Option<Nibble>,
}

impl Keyboard {
    pub fn new() -> Self {
        Keyboard { pressed: None }
    }
}

impl Chip8Keyboard for Keyboard {
    fn press(&mut self, key: Nibble) {
        self.pressed = Some(key);
    }

    fn release(&mut self, _key: Nibble) {
        self.clear();
    }

    fn is_pressed(&self, key: Nibble) -> bool {
        if let Some(n) = self.pressed {
            n == key
        } else {
            false
        }
    }

    fn clear(&mut self) {
        self.pressed = None;
    }
}

/*
/// Creates the key code for a key.
///
fn get_key_code(key: Nibble) -> u16 {
    2u16.pow(key.get_raw() as u32)
}
*/
