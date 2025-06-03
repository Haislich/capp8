use capp8_core::emulator::Emulator;
fn main() {
    let mut emulator = Emulator::new("./capp8_core/examples/1-chip8-logo.ch8").unwrap();
    emulator.run();
}
