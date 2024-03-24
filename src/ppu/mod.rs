/**
* NES Picture Processing Unit (PPU) module
*/
mod screen;

use crate::cartridge::Cartridge;
pub use screen::{Color, Screen};

pub struct PPU {
    // TODO: Implement PPU
}

impl PPU {
    pub fn new() -> PPU {
        PPU {}
    }

    pub fn tick(&self) {}

    pub fn fetch(&self) {}

    pub fn get_screen(&self, rom: Cartridge) -> Screen {
        if rom.chr_rom.len() == 0 {
            return Screen::new();
        }

        let mut screen = Screen::new();
        screen.draw_entire_sprite_map(rom.clone(), 0, 0);

        return screen;
    }
}
