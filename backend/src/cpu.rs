struct Cpu {
    // Chip-8 has 16 general purpose 8-bit registers, usually referred to as Vx, where x is a hexadecimal digit (0 through F).
    v: [u8; 16],
    // There is also a 16-bit register called I. This register is generally used to store memory addresses, so only the lowest (rightmost) 12 bits are usually used.
    i: u16,
    memory: [u8; 4096],
    delay: u8,
    timer: u8,
    // The program counter (PC) should be 16-bit, and is used to store the currently executing address
    pc: u16,
    // The stack pointer (SP) can be 8-bit, it is used to point to the topmost level of the stack.
    sp: u8,
    // The stack is an array of 16 16-bit values, used to store the address that the interpreter shoud return to when finished with a subroutine.
    stack: [u16; 16],
}
