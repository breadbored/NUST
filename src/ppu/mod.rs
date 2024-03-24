/**
* NES Picture Processing Unit (PPU) module
*/
mod screen;

use crate::cartridge::Cartridge;
pub use screen::{Color, Screen};

#[derive(Clone, Copy)]
pub struct PPU {
    ctrl: u8,     // $2000
    mask: u8,     // $2001
    status: u8,   // $2002
    oam_addr: u8, // $2003
    oam_data: u8, // $2004
    scroll: u8,   // $2005
    addr: u8,     // $2006
    data: u8,     // $2007
}

impl PPU {
    pub fn new() -> PPU {
        PPU {
            ctrl: 0,
            mask: 0,
            status: 0,
            oam_addr: 0,
            oam_data: 0,
            scroll: 0,
            addr: 0,
            data: 0,
        }
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
