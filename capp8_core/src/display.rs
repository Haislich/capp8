#![allow(unused)]
use std::ops::{Index, IndexMut};

#[derive(Clone)]
pub struct Display {
    pixel: [bool; Display::HEIGHT * Display::WIDTH],
}
impl Display {
    pub const WIDTH: usize = 64;
    pub const HEIGHT: usize = 32;
    pub fn new() -> Self {
        Self::default()
    }
    pub fn reset(&mut self) {
        self.pixel = [false; Display::HEIGHT * Display::WIDTH];
    }
    pub fn draw_byte(&mut self, byte: u8, x: usize, y: usize) -> bool {
        let mut flip = false;
        for shift in (0usize..8usize).rev() {
            let new_pixel = ((byte >> shift) & 1) > 0;
            let idx = ((x + (7 - shift) % Self::HEIGHT), y % Self::HEIGHT);
            let old_pixel = self[idx];
            flip |= old_pixel;
            self[idx] ^= new_pixel;
        }
        flip
    }
}
impl Index<(usize, usize)> for Display {
    type Output = bool;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        &self.pixel[index.1 * Display::WIDTH + index.0]
    }
}

impl IndexMut<(usize, usize)> for Display {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        &mut self.pixel[index.1 * Display::WIDTH + index.0]
    }
}
impl std::fmt::Display for Display {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "\x1B[2J\x1B[H")?;
        // write!(f, "\x1B[H")?;

        for y in 0..Display::HEIGHT {
            for x in 0..Display::WIDTH {
                write!(f, "{}", if self[(x, y)] { "⬜" } else { "⬛" })?;
                // write!(f, "{}", if self[(x, y)] { "##" } else { "  " })?;
            }
            writeln!(f)?
        }
        Ok(())
    }
}
impl Default for Display {
    fn default() -> Self {
        Self {
            pixel: [false; Display::HEIGHT * Display::WIDTH],
        }
    }
}
#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_display() {
        let mut display = Display::new();
        display[(3, 2)] = true;
        display[(2, 3)] = true;
        println!("{}", display)
    }
}
