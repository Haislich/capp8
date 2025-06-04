use std::ffi::OsStr;
use std::path::Path;

use capp8_core::emulator::Emulator;
use capp8_core::frontend::Frontend;
use raylib::prelude::*;
use raylib::{RaylibHandle, RaylibThread, ffi::KeyboardKey};

pub struct DesktopFrontend {
    emulator: Emulator,
    raylib_handle: RaylibHandle,
    raylib_thread: RaylibThread,
}
impl DesktopFrontend {
    pub fn new<P: AsRef<Path>>(rom_path: P, width: i32, height: i32) -> Self {
        let emulator =
            Emulator::new(&rom_path).expect("Could not find the specified rom {rom_path}");
        let title = rom_path
            .as_ref()
            .file_stem()
            .and_then(OsStr::to_str)
            .unwrap_or("Capp8");
        let (raylib_handle, raylib_thread) = raylib::init()
            .size(width, height)
            .title(title)
            .resizable()
            .log_level(TraceLogLevel::LOG_NONE)
            .build();

        Self {
            emulator,
            raylib_handle,
            raylib_thread,
        }
    }
}
impl Frontend for DesktopFrontend {
    fn poll_keys(&mut self) {
        self.emulator.set_keypad([
            self.raylib_handle.is_key_down(KeyboardKey::KEY_X),
            self.raylib_handle.is_key_down(KeyboardKey::KEY_ONE),
            self.raylib_handle.is_key_down(KeyboardKey::KEY_TWO),
            self.raylib_handle.is_key_down(KeyboardKey::KEY_THREE),
            self.raylib_handle.is_key_down(KeyboardKey::KEY_Q),
            self.raylib_handle.is_key_down(KeyboardKey::KEY_W),
            self.raylib_handle.is_key_down(KeyboardKey::KEY_E),
            self.raylib_handle.is_key_down(KeyboardKey::KEY_A),
            self.raylib_handle.is_key_down(KeyboardKey::KEY_S),
            self.raylib_handle.is_key_down(KeyboardKey::KEY_D),
            self.raylib_handle.is_key_down(KeyboardKey::KEY_Z),
            self.raylib_handle.is_key_down(KeyboardKey::KEY_C),
            self.raylib_handle.is_key_down(KeyboardKey::KEY_FOUR),
            self.raylib_handle.is_key_down(KeyboardKey::KEY_R),
            self.raylib_handle.is_key_down(KeyboardKey::KEY_F),
            self.raylib_handle.is_key_down(KeyboardKey::KEY_V),
        ]);
        // println!()
    }

    fn render_display(&mut self) {
        let x_scale = self.raylib_handle.get_screen_width() / 64;
        let y_scale = self.raylib_handle.get_screen_height() / 32;
        // println!("{} {}", x_scale, y_scale);
        let mut d = self.raylib_handle.begin_drawing(&self.raylib_thread);
        for x in 0..64usize {
            for y in 0..32usize {
                d.draw_rectangle(
                    x as i32 * x_scale,
                    y as i32 * y_scale,
                    x_scale,
                    y_scale,
                    if self.emulator.display()[(x, y)] {
                        Color::WHITE
                    } else {
                        Color::BLACK
                    },
                );
            }
        }
    }

    fn play_sound(&self) {}

    fn step(&mut self) {
        self.poll_keys();
        self.emulator.step();
        self.render_display();
        self.play_sound();
    }

    fn run(&mut self) {
        while !self.raylib_handle.window_should_close() {
            self.step();
        }
    }
}
