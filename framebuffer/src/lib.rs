mod utils;

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
    /// ### Arguments
    ///
    /// * **height** - The height of the buffer.
    ///
    pub fn new(height: usize) -> Self {
        FrameBuffer {
            height,
            pixels: vec![0; height],
        }
    }

    /// Draws a sprite to the buffer. Returns a flag indicating if there was a collision.
    ///
    /// ### Arguments
    ///
    /// * **x** - The x coordinate to start drawing at.
    /// * **y** - The y coordinate to start drawing at.
    /// * **sprite** - The slice containing the sprite to be drawn.
    ///
    pub fn draw(&mut self, x: usize, y: usize, sprite: &[u8]) -> bool {
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

impl fmt::Debug for FrameBuffer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        self.pixels
            .iter()
            .map(|x| f.write_fmt(format_args!("{:064b}\n", x)))
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

    #[test]
    fn draw_should_draw_a_simple_sprite() {
        let fb = {
            let mut fb = FrameBuffer::new(32);

            let collision = fb.draw(0, 0, &[0b10000001, 0b01000010, 0b00100100, 0b00011000]);

            assert!(!collision);

            fb
        };

        assert_eq!(fb[31], 0b1000000100000000000000000000000000000000000000000000000000000000);
        assert_eq!(fb[30], 0b0100001000000000000000000000000000000000000000000000000000000000);
        assert_eq!(fb[29], 0b0010010000000000000000000000000000000000000000000000000000000000);
        assert_eq!(fb[28], 0b0001100000000000000000000000000000000000000000000000000000000000);
    }

    #[test]
    fn draw_should_return_true_if_collision() {
        let mut fb = FrameBuffer::new(32);

        fb.draw(0, 0, &[0b10000001, 0b01000010, 0b00100100, 0b00011000]);
        let collision = fb.draw(0, 0, &[0b10000001, 0b01000010, 0b00100100, 0b00011000]);

        assert!(collision);
    }
}
