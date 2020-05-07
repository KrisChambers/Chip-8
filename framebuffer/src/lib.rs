extern crate data;
extern crate model;

mod utils;

use data::Byte;
use model::Chip8FrameBuffer;

/// Framebuffer implementation.
///
/// A Framebuffer contains all the data needed to draw a pixel to the screen.
/// Each pixel is represented by a bit. The height is assumed to be 64 pixels wide.
///
pub struct FrameBuffer {
    height: usize,
    pixels: Vec<u64>,
}

impl FrameBuffer {
    /// Creates a new FrameBuffer.
    ///
    ///
    ///### Arguments
    ///
    ///-** height **- : The height of the buffer.
    ///
    pub fn new(height: usize) -> Self {
        FrameBuffer {
            height,
            pixels: vec![0; height],
        }
    }
}

impl Chip8FrameBuffer for FrameBuffer {
    /// Draws a sprite to this buffer.
    ///
    ///###  Arguments
    ///
    ///- **x**        : The x coordinate for where to start drawing.
    ///- **y**        : The y coordinate for where to start drawing.
    ///- **sprite**   : A slice containing the sprite data.
    ///
    fn draw(&mut self, x: Byte, y: Byte, sprite: &[Byte]) -> bool {
        let y_initial = y;

        let mut has_collision = false;

        for y in 0..sprite.len() as usize {
            let index = utils::compute_index(y_initial.get_raw(), y, self.height);

            let sprite_line = utils::get_sprite_line(sprite, x.get_raw(), y);

            self.pixels[index] = {
                let cur = self.pixels[index];
                let new = cur ^ sprite_line;

                if (new & cur != cur) | (sprite_line & new != sprite_line) {
                    has_collision = true
                }

                new
            }
        }

        has_collision
    }

    /// Clears the frame buffer.
    ///
    fn clear(&mut self) {
        let height = self.height;
        self.pixels = vec![0; height];
    }
}

use std::fmt;
use std::{ops::Deref, result::Result};

impl fmt::Debug for FrameBuffer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        self.pixels
            .iter()
            .map(|x| f.write_fmt(format_args!("{:064b}\r\n", x)))
            .fold(Ok(()), |acc, x| acc.and(x))
    }
}

impl Deref for FrameBuffer {
    type Target = [u64];

    fn deref(&self) -> &Self::Target {
        &self.pixels
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn new_should_have_pixels_length_as_height() {
        let fb = FrameBuffer::new(32);

        assert_eq!(fb.pixels.len(), 32);
    }

    #[test]
    fn a_xor_b_and_a_is_a_iff_no_bits_shared() {
        let a = 0b1101;
        let b = 0b0010;

        assert_eq!((a ^ b) & a, a);

        let c = 0b1010;

        assert_ne!((a ^ c) & a, a);
    }

    fn get_sprite() -> [Byte; 4] {
        [
            0b10000001u8.into(),
            0b01000010u8.into(),
            0b00100100u8.into(),
            0b00011000u8.into(),
        ]
    }

    #[test]
    fn draw_should_draw_a_simple_sprite() {
        let fb = {
            let mut fb = FrameBuffer::new(32);

            let collision = fb.draw(0.into(), 0.into(), &get_sprite()[0..]);

            assert!(!collision);

            fb
        };

        assert_eq!(
            fb[31],
            0b1000000100000000000000000000000000000000000000000000000000000000
        );
        assert_eq!(
            fb[30],
            0b0100001000000000000000000000000000000000000000000000000000000000
        );
        assert_eq!(
            fb[29],
            0b0010010000000000000000000000000000000000000000000000000000000000
        );
        assert_eq!(
            fb[28],
            0b0001100000000000000000000000000000000000000000000000000000000000
        );
    }

    #[test]
    fn draw_should_return_true_if_collision() {
        let mut fb = FrameBuffer::new(32);
        let sprite = get_sprite();

        fb.draw(0.into(), 0.into(), &sprite[0..]);
        let collision = fb.draw(0.into(), 0.into(), &sprite[0..]);

        assert!(collision);
    }
}
