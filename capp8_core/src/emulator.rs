use std::ops::BitXor;

use crate::{display::Display, instruction::Instruction};
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

pub struct Emulator {
    v: [u8; 16],
    i: u16,
    memory: [u8; 4096],
    program_counter: u16,
    stack: [u16; 16],
    stack_pointer: usize,
    delay: u8,
    sound: u8,
    display: Display,
}

impl Emulator {
    pub fn new() -> Self {
        Self {
            v: [0; 16],
            i: 0,
            program_counter: 0x200,
            memory: [0; 4096],
            stack_pointer: 0,
            stack: [0; 16],
            delay: 0,
            sound: 0,
            display: Display::new(),
        }
    }

    pub fn cycle(&mut self) {
        let opcode = self.fetch();
        let instruction = self.decode(opcode);
        self.execute(instruction);
    }
    pub fn fetch(&self) -> u16 {
        let msb = self.memory[self.program_counter as usize];
        let lsb = self.memory[(self.program_counter + 1) as usize];
        u16::from_be_bytes([msb, lsb])
    }
    pub fn decode(&self, opcode: u16) -> Instruction {
        let opcode = Opcode::from(opcode);
        let Opcode { n1, n2, n3, n4 } = opcode;
        match (n1, n2, n3, n4) {
            (0, 0, 0xE, 0) => Instruction::ClearScreen,

            (0, 0, 0xE, 0xE) => Instruction::Return,

            (0, _, _, _) => Instruction::Sys {
                addr: opcode.addr(),
            },

            (1, _, _, _) => Instruction::Jump {
                addr: opcode.addr(),
            },
            (2, _, _, _) => Instruction::Call {
                addr: opcode.addr(),
            },
            (3, _, _, _) => Instruction::SkipRegEqImm {
                reg: opcode.x(),
                imm: opcode.byte(),
            },

            (4, _, _, _) => Instruction::SkipRegNeqImm {
                reg: opcode.x(),
                imm: opcode.byte(),
            },
            (5, _, _, 0) => Instruction::SkipRegEqReg {
                reg_x: opcode.x(),
                reg_y: opcode.y(),
            },
            (6, _, _, _) => Instruction::StoreRegFromImm {
                reg: opcode.x(),
                imm: opcode.byte(),
            },
            (7, _, _, _) => Instruction::AddRegImm {
                reg: opcode.x(),
                imm: opcode.byte(),
            },
            (8, _, _, 0) => Instruction::StoreRegFromReg {
                reg_x: opcode.x(),
                reg_y: opcode.y(),
            },
            (8, _, _, 1) => Instruction::OrRegReg {
                reg_x: opcode.x(),
                reg_y: opcode.y(),
            },
            (8, _, _, 2) => Instruction::AndRegReg {
                reg_x: opcode.x(),
                reg_y: opcode.y(),
            },
            (8, _, _, 3) => Instruction::XorRegReg {
                reg_x: opcode.x(),
                reg_y: opcode.y(),
            },
            (8, _, _, 4) => Instruction::AddRegReg {
                reg_x: opcode.x(),
                reg_y: opcode.y(),
            },
            (8, _, _, 5) => Instruction::SubRegReg {
                reg_x: opcode.x(),
                reg_y: opcode.y(),
            },
            (8, _, _, 6) => Instruction::ShiftRight { reg: opcode.x() },
            (8, _, _, 7) => Instruction::SubnRegReg {
                reg_x: opcode.x(),
                reg_y: opcode.y(),
            },
            (8, _, _, 0xE) => Instruction::ShiftLeft { reg: opcode.x() },
            (9, _, _, 0) => Instruction::SkipRegNeqReg {
                reg_x: opcode.x(),
                reg_y: opcode.y(),
            },
            (0xA, _, _, _) => Instruction::SetI {
                addr: opcode.addr(),
            },
            (0xB, _, _, _) => Instruction::JumpWithOffset {
                addr: opcode.addr(),
            },
            (0xC, _, _, _) => Instruction::Rand {
                reg: opcode.x(),
                mask: opcode.byte(),
            },
            (0xD, _, _, _) => Instruction::Draw {
                reg_x: opcode.x(),
                reg_y: opcode.y(),
                height: opcode.nibble(),
            },
            (0xE, _, 9, 0xE) => Instruction::SkipIfKey { reg: opcode.x() },
            (0xE, _, 0xA, 1) => Instruction::SkipIfNotKey { reg: opcode.x() },
            (0xF, _, 0, 7) => Instruction::LoadDelayTimer { reg: opcode.x() },
            (0xF, _, 0, 0xA) => Instruction::WaitKeyPress { reg: opcode.x() },
            (0xF, _, 1, 5) => Instruction::SetDelayTimer { reg: opcode.x() },
            (0xF, _, 1, 8) => Instruction::SetSoundTimer { reg: opcode.x() },
            (0xF, _, 1, 0xE) => Instruction::AddI { reg: opcode.x() },
            (0xF, _, 2, 9) => Instruction::SetIToSprite { reg: opcode.x() },
            (0xF, _, 3, 3) => Instruction::StoreBCD { reg: opcode.x() },
            (0xF, _, 5, 5) => Instruction::StoreRegisters { reg: opcode.x() },
            (0xF, _, 6, 5) => Instruction::LoadRegisters { reg: opcode.x() },

            _ => unimplemented!(),
        }
    }
    pub fn execute(&mut self, instruction: Instruction) {
        match instruction {
            Instruction::Sys { addr: _ } => {
                // TODO: Should probably add a log and ignore maybe (?)
                unimplemented!("This should be ignored on most modern interpreters.")
            }
            Instruction::ClearScreen => {
                self.display.reset();
            }
            Instruction::Return => {
                self.program_counter = self.stack[self.stack_pointer];
                self.stack_pointer -= 1;
            }
            Instruction::Jump { addr } => self.program_counter = addr,
            Instruction::Call { addr } => {
                self.stack_pointer += 1;
                self.stack[self.stack_pointer] = self.program_counter;
                self.program_counter = addr;
            }
            Instruction::SkipRegEqImm { reg, imm } => {
                if self.v[reg] == imm {
                    self.program_counter += 2
                }
            }
            Instruction::SkipRegNeqImm { reg, imm } => {
                if self.v[reg] != imm {
                    self.program_counter += 2
                }
            }
            Instruction::SkipRegEqReg { reg_x, reg_y } => {
                if self.v[reg_x] == self.v[reg_y] {
                    self.program_counter += 2
                }
            }
            Instruction::StoreRegFromImm { reg, imm } => self.v[reg] = imm,
            Instruction::StoreRegFromReg { reg_x, reg_y } => self.v[reg_x] = self.v[reg_y],
            Instruction::AddRegImm { reg, imm } => self.v[reg] += imm,
            Instruction::AddRegReg { reg_x, reg_y } => {
                // TODO: Check. I think I should upcast into an u16 and then downcast
                let (v_x, carry) = self.v[reg_x].overflowing_add(self.v[reg_y]);

                self.v[reg_x] = v_x;
                self.v[0xF] = if carry { 1 } else { 0 };
            }
            Instruction::OrRegReg { reg_x, reg_y } => {
                self.v[reg_x] |= self.v[reg_y];
            }
            Instruction::AndRegReg { reg_x, reg_y } => {
                self.v[reg_x] &= self.v[reg_y];
            }
            Instruction::SubRegReg { reg_x, reg_y } => {
                // TODO: Check. I think they mean only subtract if Vx > Vy
                let (v_x, borrow) = self.v[reg_x].overflowing_sub(self.v[reg_y]);
                self.v[reg_x] = v_x;
                self.v[0xF] = if borrow { 1 } else { 0 };
            }
            Instruction::ShiftRight { reg } => {
                self.v[0xF] == self.v[reg] & 1;
                self.v[reg] >>= 2
            }
            Instruction::SubnRegReg { reg_x, reg_y } => {
                // TODO: Check. I think they mean only subtract if Vx > Vy
                let (v_x, borrow) = self.v[reg_x].overflowing_sub(self.v[reg_y]);
                self.v[reg_x] = v_x;
                self.v[0xF] = if borrow { 0 } else { 1 };
            } // _ => unimplemented!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn decode_sys() {
        let emu = Emulator::new();
        emu.decode(0xFFFF);
        emu.decode(0x000F);
        emu.decode(0x00F0);
        emu.decode(0xCE00);
        emu.decode(0x0E80);
        emu.decode(0x0E0A);
    }
}
