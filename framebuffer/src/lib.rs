extern crate model;
extern crate data;

mod utils;

use model::FrameBuffer;

/// Framebuffer implementation.
///
/// A Framebuffer contains all the data needed to draw a pixel to the screen.
/// Each pixel is represented by a bit. The height is assumed to be 64 pixels wide.
///
pub struct Chip8FrameBuffer {
    height: usize,
    pixels: Vec<u64>,
}

impl Chip8FrameBuffer {
    /// Creates a new FrameBuffer.
    ///
    ///
    ///### Arguments
    ///
    ///-** height **- : The height of the buffer.
    ///
    pub fn new(height: usize) -> Self {
        Chip8FrameBuffer {
            height,
            pixels: vec![0; height],
        }
    }
}

impl FrameBuffer for Chip8FrameBuffer {
    /// Draws a sprite to this buffer.
    ///
    ///###  Arguments
    ///
    ///-** x **-        : The x coordinate for where to start drawing.
    ///-** y **-        : The y coordinate for where to start drawing.
    ///-** sprite **-   : A slice containing the sprite data.
    ///
    fn draw(&mut self, x: usize, y: usize, sprite: &[data::Byte]) -> bool {
        let y_initial = y;

        let mut has_collision = false;

        for y in 0..sprite.len() as usize {
            let index = utils::compute_index(y_initial, y, self.height);

            let sprite_line = utils::get_sprite_line(sprite, x as u32, y);

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
}

use std::fmt;
use std::{ops::Deref, result::Result};

impl fmt::Debug for Chip8FrameBuffer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        self.pixels
            .iter()
            .map(|x| f.write_fmt(format_args!("{:064b}\r\n", x)))
            .fold(Ok(()), |acc, x| acc.and(x))
    }
}

impl Deref for Chip8FrameBuffer {
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
        let fb = Chip8FrameBuffer::new(32);

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

    #[test]
    fn draw_should_draw_a_simple_sprite() {
        let fb = {
            let mut fb = Chip8FrameBuffer::new(32);

            let collision = fb.draw(0, 0, &[
                0b10000001u8.into(),
                0b01000010.into(),
                0b00100100.into(),
                0b00011000.into()
            ]);

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
        let mut fb = Chip8FrameBuffer::new(32);

        fb.draw(0, 0, &[0b10000001.into(), 0b01000010.into(), 0b00100100.into(), 0b00011000.into()]);
        let collision = fb.draw(0, 0, &[0b10000001.into(), 0b01000010.into(), 0b00100100.into(), 0b00011000.into()]);

        assert!(collision);
    }
}
