/**
* NES Picture Processing Unit (PPU) module
*/
mod screen;

use std::sync::{Arc, Mutex};

use crate::{cartridge::Cartridge, system::System};
pub use screen::{Color, Screen};

#[derive(Clone, Copy)]
pub struct PPU {
    pub ctrl: u8,     // $2000
    pub mask: u8,     // $2001
    pub status: u8,   // $2002
    pub oam_addr: u8, // $2003
    pub oam_data: u8, // $2004
    pub scroll: u8,   // $2005
    pub addr: u8,     // $2006
    pub data: u8,     // $2007

    pub scanline: u16,
}

#[derive(Clone, Copy)]
pub struct Sprite {
    pub y: u8,
    pub tile: u8,
    pub attr: u8,
    pub x: u8,
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

            scanline: 0,
        }
    }

    pub fn tick(&mut self, system: &mut Arc<Mutex<System>>, cycles: &mut u64) -> u64 {
        let vram = system.lock().unwrap().vram.clone();
        let ram = system.lock().unwrap().ram.clone();
        let cpu = system.lock().unwrap().cpu.clone();
        let mut cycle = *cycles;

        match self.scanline {
            0..=239 => self.render_scanline(cycle),
            240 => self.post_render(),
            241..=260 => self.vertical_blank(),
            _ => {}
        }

        cycle += 1;
        if cycle > 340 {
            cycle = 0;
            self.scanline += 1;
            if self.scanline > 261 {
                self.scanline = 0;
            }
        }

        return cycle;
    }

    pub fn render_scanline(&self, cycles: u64) {
        if cycles <= 0 {
            // Idle
        } else if cycles <= 256 {
            // Fetch 2 tiles per cycle, including tile and attribute
        } else if cycles <= 320 {
        } else if cycles <= 336 {
        } else if cycles <= 340 {
        }
    }

    pub fn post_render(&self) {
        // Post render
    }

    pub fn vertical_blank(&self) {
        // Vertical blank
    }

    pub fn fetch(&self, system: &mut Arc<Mutex<System>>, sprite_num: u8) -> Sprite {
        let oam = system.lock().unwrap().oam.clone();

        // Fetch 2 tiles per cycle, including tile and attribute
        let sprite_addr = sprite_num * 4;
        let y_coord = sprite_addr;
        let tile_num = sprite_addr + 1;
        let attribute = sprite_addr + 2;
        let x_coord = sprite_addr + 3;

        return Sprite {
            y: oam.lock().unwrap()[y_coord as usize],
            tile: oam.lock().unwrap()[tile_num as usize],
            attr: oam.lock().unwrap()[attribute as usize],
            x: oam.lock().unwrap()[x_coord as usize],
        };
    }

    pub fn get_screen(&self, rom: Cartridge) -> Screen {
        if rom.chr_rom.len() == 0 {
            return Screen::new();
        }

        let mut screen = Screen::new();
        screen.draw_entire_sprite_map(rom.clone(), 0, 0);

        return screen;
    }
}
