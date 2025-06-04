#![allow(dead_code)]
#![allow(unused)]

use std::{
    fs::{File, OpenOptions},
    io::Read,
    path::{Path, PathBuf},
    thread,
    time::Duration,
};

use crate::{
    display::Display, fonts::FONTS, frontend::Frontend, instruction::Instruction, opcode::Opcode,
};
use rand::{Rng, rngs::ThreadRng};

pub struct Emulator {
    v: [u8; 16],
    i: u16,
    memory: [u8; 4096],
    program_counter: u16,
    stack: [u16; 16],
    stack_pointer: usize,
    delay_timer: u8,
    sound_timer: u8,
    display: Display,
    keypad: [bool; 16],
    rng: ThreadRng,
    timer_accum: Duration,
}

impl Emulator {
    pub fn new<P: AsRef<Path>>(rom_path: &P) -> Result<Self, std::io::Error> {
        let mut memory = [0; 4096];
        memory[0x50..=0x9F].copy_from_slice(&FONTS[..]);
        let mut file = OpenOptions::new().read(true).open(rom_path)?;
        let mut buf: Vec<u8> = Vec::new();
        let file_size = file.read_to_end(&mut buf)?;
        memory[0x200..0x200 + file_size].copy_from_slice(buf.as_slice());
        Ok(Self {
            v: [0; 16],
            i: 0,
            program_counter: 0x200,
            memory,
            stack_pointer: 0,
            stack: [0; 16],
            delay_timer: 0,
            sound_timer: 0,
            display: Display::new(),
            keypad: [false; 16],
            rng: rand::rng(),
            timer_accum: Duration::new(0, 0),
        })
    }
    pub fn display(&self) -> &Display {
        &self.display
    }
    pub fn set_keypad(&mut self, keypad: [bool; 16]) {
        self.keypad = keypad;
    }
    pub fn step(&mut self, dt: Duration) {
        let opcode = self.fetch();
        self.program_counter += 2;
        let instruction = self.decode(opcode);
        self.execute(instruction);
        // 2. accumulate elapsed time
        self.timer_accum += dt; // dt comes from the main loop

        // 3. tick timers every 16-17 ms no matter how many opcodes we executed
        while self.timer_accum >= Duration::from_micros(16_667) {
            self.timer_accum -= Duration::from_micros(16_667);
            if self.delay_timer > 0 {
                self.delay_timer -= 1;
            }
            if self.sound_timer > 0 {
                self.sound_timer -= 1;
            }
        }
    }

    /// Read the instruction that PC is currently pointing at from memory.
    fn fetch(&self) -> Opcode {
        let msb = self.memory[self.program_counter as usize];
        let lsb = self.memory[(self.program_counter + 1) as usize];
        Opcode::from(u16::from_be_bytes([msb, lsb]))
    }
    fn decode(&self, opcode: Opcode) -> Instruction {
        match opcode.nibbles() {
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
                nibble: opcode.nibble(),
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
    fn execute(&mut self, instruction: Instruction) {
        // println!("{:?}", self.keypad);
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
            Instruction::AddRegImm { reg, imm } => self.v[reg] = self.v[reg].wrapping_add(imm),
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
            Instruction::XorRegReg { reg_x, reg_y } => self.v[reg_x] ^= self.v[reg_y],
            Instruction::SubRegReg { reg_x, reg_y } => {
                // TODO: Check. I think they mean only subtract if Vx > Vy
                let (v_x, borrow) = self.v[reg_x].overflowing_sub(self.v[reg_y]);
                self.v[reg_x] = v_x;
                self.v[0xF] = if borrow { 1 } else { 0 };
            }
            Instruction::ShiftRight { reg } => {
                self.v[0xF] == self.v[reg] & 1;
                self.v[reg] >>= 1
            }
            Instruction::SubnRegReg { reg_x, reg_y } => {
                // TODO: Check. I think they mean only subtract if Vx > Vy
                let (v_x, borrow) = self.v[reg_y].overflowing_sub(self.v[reg_x]);
                self.v[reg_x] = v_x;
                self.v[0xF] = if borrow { 1 } else { 0 };
            }
            Instruction::ShiftLeft { reg } => {
                self.v[0xF] == self.v[reg] & 0x80;
                self.v[reg] <<= 1
            }
            Instruction::SkipRegNeqReg { reg_x, reg_y } => {
                if self.v[reg_x] != self.v[reg_y] {
                    self.program_counter += 2
                }
            }
            Instruction::SetI { addr } => self.i = addr,
            Instruction::JumpWithOffset { addr } => {
                self.program_counter = (self.v[0] as u16) + addr;
            }
            Instruction::Rand { reg, mask } => {
                self.v[reg] = self.rng.random_range(0..255) & mask;
            }

            Instruction::Draw {
                reg_x,
                reg_y,
                nibble,
            } => {
                // We're assuming a specific size of the screen
                let v_x = self.v[reg_x] & 0x3F;
                let v_y = self.v[reg_y] & 0x1F;
                let rows = nibble as usize;
                let mut flip = false;
                for row in 0..rows {
                    flip |= self.display.draw_byte(
                        self.memory[self.i as usize + row],
                        v_x as usize,
                        v_y as usize + row,
                    )
                }
                self.v[0xF] = if flip { 1 } else { 0 };
            }
            Instruction::SkipIfKey { reg } => {
                if self.keypad[self.v[reg] as usize] {
                    self.program_counter += 2
                }
            }
            Instruction::SkipIfNotKey { reg } => {
                if !self.keypad[self.v[reg] as usize] {
                    self.program_counter += 2
                }
            }
            Instruction::LoadDelayTimer { reg } => self.v[reg] = self.delay_timer,
            Instruction::WaitKeyPress { reg } => {
                // 'wait: loop {
                // println! {"Iaaaa we're here"};
                let mut key_pressed = false;
                for (idx, key) in self.keypad.iter().enumerate() {
                    if *key {
                        key_pressed = true;
                        self.v[reg] = idx as u8;
                        break;
                    }
                }
                // TODO: Find a better solution
                // super hacky, rewind the program ccounter by two
                if !key_pressed {
                    self.program_counter -= 2;
                }
            }
            Instruction::SetDelayTimer { reg } => self.delay_timer = self.v[reg],
            Instruction::SetSoundTimer { reg } => self.sound_timer = self.v[reg],
            Instruction::AddI { reg } => self.i += self.v[reg] as u16,
            Instruction::SetIToSprite { reg } => {
                // TODO: Check for errors.
                self.i = self.v[reg] as u16;
            }
            Instruction::StoreBCD { reg } => {
                let v_x = self.v[reg];
                self.memory[self.i as usize] = (v_x >> 2) & 1;
                self.memory[self.i as usize] = (v_x >> 1) & 1;
                self.memory[self.i as usize] = (v_x) & 1
            }
            Instruction::StoreRegisters { reg } => {
                for offset in 0..=reg {
                    self.memory[(self.i as usize) + offset] = self.v[offset]
                }
            }
            Instruction::LoadRegisters { reg } => {
                for offset in 0..=reg {
                    self.v[offset] = self.memory[(self.i as usize) + offset]
                }
            }
        }
    }
}
