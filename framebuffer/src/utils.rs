/// Computes an index based on initial y, current y, and a height.
///
/// The height is used to wrap around.
///
pub fn compute_index<YInitial: Into<usize>, YCurrent: Into<usize>>(
    y_initial: YInitial,
    y_current: YCurrent,
    height: usize,
) -> usize {
    (height - 1)
        .wrapping_sub(y_initial.into() + y_current.into())
        .checked_rem(height)
        .unwrap_or(0)
}

/// Gets the sprite_line from the memory slice.
///
/// The line is returned in the right position as a u64 to be
/// written to the display.
///
pub fn get_sprite_line<X: Into<u32>, Y: Into<usize>>(mem_slice: &[u8], x: X, y: Y) -> u64 {
    let line = mem_slice[y.into()] as u64;

    let line = line.rotate_right(8).rotate_right(x.into());

    line
}
