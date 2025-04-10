/// The full set of CHIP-8 instruction.
///
/// http://devernay.free.fr/hacks/chip8/C8TECH10.HTM#0.0
pub enum Instruction {
    /// 0nnn - SYS addr  
    /// Call a machine code routine at address `addr`. (Ignored on most modern interpreters.)
    Sys { addr: u16 },

    /// 00E0 - CLS  
    /// Clear the display.
    ClearScreen,

    /// 00EE - RET  
    /// Return from a subroutine.
    Return,

    /// 1nnn - JP addr  
    /// Jump to address `addr`.
    Jump { addr: u16 },

    /// 2nnn - CALL addr  
    /// Call subroutine at address `addr`.
    Call { addr: u16 },

    /// 3xkk - SE Vx, byte  
    /// Skip the next instruction if Vx == `imm`.
    SkipRegEqImm { reg: usize, imm: u8 },

    /// 4xkk - SNE Vx, byte  
    /// Skip the next instruction if Vx != `imm`.
    SkipRegNeqImm { reg: usize, imm: u8 },

    /// 5xy0 - SE Vx, Vy  
    /// Skip the next instruction if Vx == Vy.
    SkipRegEqReg { reg_x: usize, reg_y: usize },

    /// 6xkk - LD Vx, byte  
    /// Set Vx = `imm`.
    StoreRegFromImm { reg: usize, imm: u8 },

    /// 8xy0 - LD Vx, Vy  
    /// Set Vx = Vy.
    StoreRegFromReg { reg_x: usize, reg_y: usize },

    /// 7xkk - ADD Vx, byte  
    /// Set Vx = Vx + `imm`.
    AddRegImm { reg: usize, imm: u8 },

    /// 8xy4 - ADD Vx, Vy  
    /// Set Vx = Vx + Vy. Set VF = carry.
    AddRegReg { reg_x: usize, reg_y: usize },

    /// 8xy1 - OR Vx, Vy  
    /// Set Vx = Vx OR Vy.
    OrRegReg { reg_x: usize, reg_y: usize },

    /// 8xy2 - AND Vx, Vy  
    /// Set Vx = Vx AND Vy.
    AndRegReg { reg_x: usize, reg_y: usize },

    /// 8xy3 - XOR Vx, Vy  
    /// Set Vx = Vx XOR Vy.
    XorRegReg { reg_x: usize, reg_y: usize },

    /// 8xy5 - SUB Vx, Vy  
    /// Set Vx = Vx - Vy. Set VF = NOT borrow.
    SubRegReg { reg_x: usize, reg_y: usize },

    /// 8xy7 - SUBN Vx, Vy  
    /// Set Vx = Vy - Vx. Set VF = NOT borrow.
    SubnRegReg { reg_x: usize, reg_y: usize },

    /// 8xy6 - SHR Vx {, Vy}  
    /// Set Vx = Vx >> 1. Store LSB in VF.
    ShiftRight { reg: usize },

    /// 8xyE - SHL Vx {, Vy}  
    /// Set Vx = Vx << 1. Store MSB in VF.
    ShiftLeft { reg: usize },

    /// 9xy0 - SNE Vx, Vy  
    /// Skip the next instruction if Vx != Vy.
    SkipRegNeqReg { reg_x: usize, reg_y: usize },

    /// Annn - LD I, addr  
    /// Set I = `addr`.
    SetI { addr: u16 },

    /// Bnnn - JP V0, addr  
    /// Jump to address `addr + V0`.
    JumpWithOffset { addr: u16 },

    /// Cxkk - RND Vx, byte  
    /// Set Vx = random byte AND `mask`.
    Rand { reg: usize, mask: u8 },

    /// Dxyn - DRW Vx, Vy, nibble  
    /// Display n-byte sprite starting at memory[I] at (Vx, Vy). Set VF = collision.
    Draw {
        reg_x: usize,
        reg_y: usize,
        height: u8,
    },

    /// Ex9E - SKP Vx  
    /// Skip the next instruction if key with the value of Vx is pressed.
    SkipIfKey { reg: usize },

    /// ExA1 - SKNP Vx  
    /// Skip the next instruction if key with the value of Vx is not pressed.
    SkipIfNotKey { reg: usize },

    /// Fx07 - LD Vx, DT  
    /// Set Vx = delay timer value.
    LoadDelayTimer { reg: usize },

    /// Fx0A - LD Vx, K  
    /// Wait for a key press, store the value of the key in Vx.
    WaitKeyPress { reg: usize },

    /// Fx15 - LD DT, Vx  
    /// Set delay timer = Vx.
    SetDelayTimer { reg: usize },

    /// Fx18 - LD ST, Vx  
    /// Set sound timer = Vx.
    SetSoundTimer { reg: usize },

    /// Fx1E - ADD I, Vx  
    /// Set I = I + Vx.
    AddI { reg: usize },

    /// Fx29 - LD F, Vx  
    /// Set I = location of sprite for digit Vx.
    SetIToSprite { reg: usize },

    /// Fx33 - LD B, Vx  
    /// Store BCD representation of Vx in memory at I, I+1, I+2.
    StoreBCD { reg: usize },

    /// Fx55 - LD [I], Vx  
    /// Store registers V0 through Vx in memory starting at address I.
    StoreRegisters { reg: usize },

    /// Fx65 - LD Vx, [I]  
    /// Read registers V0 through Vx from memory starting at address I.
    LoadRegisters { reg: usize },
}
