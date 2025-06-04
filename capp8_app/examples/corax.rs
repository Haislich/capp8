use capp8_core::frontend::Frontend;
use capp8_desktop::gui::DesktopFrontend;

fn main() {
    let mut frontend = DesktopFrontend::new("./capp8_app/examples/3-corax+.ch8", 640, 320);
    frontend.run();
}
