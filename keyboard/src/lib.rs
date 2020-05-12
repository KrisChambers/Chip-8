extern crate data;
extern crate model;

use data::Nibble;
use model::Chip8Keyboard;

pub struct Keyboard {
    keys: u16,
}

impl Keyboard {
    pub fn new() -> Self {
        Keyboard { keys: 0 }
    }
}

impl Chip8Keyboard for Keyboard {
    fn press(&mut self, key: Nibble) {
        self.keys = get_key_code(key);
    }

    fn release(&mut self, key: Nibble) {
        self.keys &= !get_key_code(key);
    }

    fn is_pressed(&self, key: Nibble) -> bool {
        self.keys == get_key_code(key)
    }

    fn clear(&mut self) {
        self.keys = 0;
    }
}

/// Creates the key code for a key.
///
fn get_key_code(key: Nibble) -> u16 {
    2u16.pow(key.get_raw() as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! key_test {
        (
            $([$($pressed: expr,)+] => $state: expr),+
        ) => {
            $(
                let mut kb = Keyboard::new();

                $(
                    kb.press($pressed.into());
                )+

                assert_eq!(kb.keys, $state);
            )+
        }
    }

    #[test]
    fn name() {
        assert_eq!(get_key_code(0.into()), 1);
        assert_eq!(get_key_code(1.into()), 2);
    }

    #[test]
    fn press_single_key() {
        for key in 0..16 as u8 {
            key_test!([key,] => get_key_code(key.into()));
        }
    }

    #[test]
    fn press_multiple() {
        key_test!([0, 1, 2, ] => get_key_code(0.into()) | get_key_code(1.into()) | get_key_code(2.into()));
    }
}
