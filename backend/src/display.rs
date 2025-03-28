#![allow(unused)]
use std::ops::{Index, IndexMut};
const HEIGHT: usize = 32;
const WIDTH: usize = 64;
const FONTS: [u8; 80] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
    0x20, 0x60, 0x20, 0x20, 0x70, // 1
    0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
    0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
    0x90, 0x90, 0xF0, 0x10, 0x10, // 4
    0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
    0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
    0xF0, 0x10, 0x20, 0x40, 0x40, // 7
    0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
    0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
    0xF0, 0x90, 0xF0, 0x90, 0x90, // A
    0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
    0xF0, 0x80, 0x80, 0x80, 0xF0, // C
    0xE0, 0x90, 0x90, 0x90, 0xE0, // D
    0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
    0xF0, 0x80, 0xF0, 0x80, 0x80, // F
];

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct FontDigit(usize);
#[derive(Debug)]
pub struct FontDigitError(usize);
impl std::fmt::Display for FontDigitError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Invalid font digit value: {}", self.0)
    }
}
impl std::error::Error for FontDigitError {}
impl FontDigit {
    pub fn new(val: usize) -> Result<Self, FontDigitError> {
        if val <= 0xF {
            Ok(FontDigit(val))
        } else {
            Err(FontDigitError(val))
        }
    }

    pub fn get(self) -> usize {
        self.0
    }
}
fn xor(digit1: FontDigit, digit2: FontDigit) -> [u8; 5] {
    let digit1 = digit1.get() * 5;
    let digit2 = digit2.get() * 5;
    [
        FONTS[digit1] ^ FONTS[digit2],
        FONTS[digit1 + 1] ^ FONTS[digit2 + 1],
        FONTS[digit1 + 2] ^ FONTS[digit2 + 2],
        FONTS[digit1 + 3] ^ FONTS[digit2 + 3],
        FONTS[digit1 + 4] ^ FONTS[digit2 + 4],
    ]
}
pub struct Display {
    pixel: [bool; HEIGHT * WIDTH],
}
impl Display {
    pub fn new() -> Self {
        Self {
            pixel: [false; HEIGHT * WIDTH],
        }
    }
    pub fn reset(&mut self) {
        self.pixel = [false; HEIGHT * WIDTH];
    }
}
impl<T> Index<(T, T)> for Display
where
    T: Into<usize>,
{
    type Output = bool;

    fn index(&self, index: (T, T)) -> &Self::Output {
        &self.pixel[index.0.into() * WIDTH + index.1.into()]
    }
}
impl<T> IndexMut<(T, T)> for Display
where
    T: Into<usize>,
{
    fn index_mut(&mut self, index: (T, T)) -> &mut Self::Output {
        &mut self.pixel[index.0.into() * WIDTH + index.1.into()]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn print_font(font: &[u8]) {
        for i in 0..5 {
            // Only the leftmost bits are used actually
            (4usize..8usize).rev().into_iter().for_each(|shift| {
                print!(
                    "{}",
                    if ((font[i] >> shift) & 1) > 0 {
                        "⬜"
                    } else {
                        "⬛"
                    }
                )
            });
            println!("")
        }
    }
    #[test]
    fn testone() {
        let digit1 = FontDigit::new(17).unwrap();
        let digit2 = FontDigit::new(2).unwrap();

        let elem = xor(digit1, digit2);

        print_font(&[
            FONTS[digit1.get() * 5],
            FONTS[digit1.get() * 5 + 1],
            FONTS[digit1.get() * 5 + 2],
            FONTS[digit1.get() * 5 + 3],
            FONTS[digit1.get() * 5 + 4],
        ]);
        println!();
        print_font(&[
            FONTS[digit2.get() * 5],
            FONTS[digit2.get() * 5 + 1],
            FONTS[digit2.get() * 5 + 2],
            FONTS[digit2.get() * 5 + 3],
            FONTS[digit2.get() * 5 + 4],
        ]);
        println!();
        print_font(&elem);
    }
    #[test]
    fn test_display() {
        let mut display = Display::new();
        display[(16usize, 32usize)] = true;
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                print!("{}", if display[(y, x)] { "⬜" } else { "⬛" });
            }
            println!("")
        }
    }
}
