pub struct Opcode {
    n1: u8,
    n2: u8,
    n3: u8,
    n4: u8,
}
impl Opcode {
    pub fn addr(&self) -> u16 {
        u16::from_be_bytes([self.n2, (self.n3 << 4) + self.n4])
    }
    pub fn nibbles(&self) -> (u8, u8, u8, u8) {
        (self.n1, self.n2, self.n3, self.n4)
    }
    pub fn nibble(&self) -> u8 {
        self.n4
    }
    pub fn x(&self) -> usize {
        self.n2 as usize
    }
    pub fn y(&self) -> usize {
        self.n3 as usize
    }
    pub fn byte(&self) -> u8 {
        (self.n3 << 4) + self.n4
    }
}
impl From<u16> for Opcode {
    fn from(value: u16) -> Self {
        Self {
            n1: ((value & 0xF000) >> 12) as u8,
            n2: ((value & 0x0F00) >> 8) as u8,
            n3: ((value & 0x00F0) >> 4) as u8,
            n4: (value & 0x000F) as u8,
        }
    }
}
