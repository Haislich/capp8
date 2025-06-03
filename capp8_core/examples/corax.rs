use capp8_core::emulator::Emulator;
fn main() {
    let mut emulator = Emulator::new("./capp8_core/examples/3-corax+.ch8").unwrap();
    emulator.run();
}
