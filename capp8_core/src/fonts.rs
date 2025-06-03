use std::{fmt::Debug, ops::BitXor};

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Font([u8; 5], [bool; 20]);

impl Font {
    pub const FONTS: [u8; 80] = [
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

    pub fn sprite(&self) -> &[bool; 20] {
        &self.1
    }
}
impl TryFrom<usize> for Font {
    type Error = FontIndexError;

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        let value = value * 5;
        if value + 4 < 80 {
            let font = [
                Font::FONTS[value],
                Font::FONTS[value + 1],
                Font::FONTS[value + 2],
                Font::FONTS[value + 3],
                Font::FONTS[value + 4],
            ];
            let mut sprite = [false; 20];

            let mut cnt = 0;
            for i in 0..5 {
                // Only the leftmost bits are used actually
                for shift in (4usize..8usize).rev() {
                    sprite[i * 4 + (7 - shift)] = ((font[i] >> shift) & 1) > 0;
                    cnt += 1;
                }
            }
            println!("{cnt}");

            Ok(Font(font, sprite))
        } else {
            Err(FontIndexError(value))
        }
    }
}
impl From<[u8; 5]> for Font {
    fn from(value: [u8; 5]) -> Self {
        Font(value, [false; 20])
    }
}
impl BitXor<Font> for Font {
    type Output = Font;
    fn bitxor(self, rhs: Font) -> Self::Output {
        Font::from([
            self.0[0] ^ rhs.0[0],
            self.0[1] ^ rhs.0[1],
            self.0[2] ^ rhs.0[2],
            self.0[3] ^ rhs.0[3],
            self.0[4] ^ rhs.0[4],
        ])
    }
}
impl BitXor<[u8; 5]> for Font {
    type Output = Font;
    fn bitxor(self, rhs: [u8; 5]) -> Self::Output {
        let rhs: Font = rhs.into();
        self ^ rhs
    }
}
impl BitXor<Font> for [u8; 5] {
    type Output = Font;
    fn bitxor(self, rhs: Font) -> Self::Output {
        rhs ^ self
    }
}
impl std::fmt::Display for Font {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in 0..5 {
            // Only the leftmost bits are used actually
            for shift in (4usize..8usize).rev() {
                write!(
                    f,
                    "{}",
                    if ((self.0[i] >> shift) & 1) > 0 {
                        "⬜"
                    } else {
                        "⬛"
                    }
                )?;
            }
            writeln!(f)?
        }
        Ok(())
    }
}

#[derive(Debug)]
pub struct FontIndexError(usize);
impl std::fmt::Display for FontIndexError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Invalid font index: {}. Expected 0-15.", self.0)
    }
}
impl std::error::Error for FontIndexError {}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_fonts() {
        for value in 0..=15 {
            println!("{}", Font::try_from(value).unwrap())
        }
    }
}
