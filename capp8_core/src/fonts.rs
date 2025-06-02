use std::{fmt::Debug, ops::BitXor};

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Font([u8; 5]);

impl Font {
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
}
impl TryFrom<usize> for Font {
    type Error = FontIndexError;

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        if value % 4 == 0 && value + 4 < 80 {
            Ok(Font([
                Font::FONTS[value],
                Font::FONTS[value + 1],
                Font::FONTS[value + 2],
                Font::FONTS[value + 3],
                Font::FONTS[value + 4],
            ]))
        } else {
            Err(FontIndexError(value))
        }
    }
}
impl From<[u8; 5]> for Font {
    fn from(value: [u8; 5]) -> Self {
        Font(value)
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
        let rhs: Font = rhs.into();
        rhs ^ self
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
