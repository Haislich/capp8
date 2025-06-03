use capp8_core::emulator::Emulator;
fn main() {
    let mut emulator = Emulator::new("./capp8_core/examples/4-flags.ch8").unwrap();
    emulator.run();
}
