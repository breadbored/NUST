use crate::cartridge::Cartridge;
use crate::cpu::CPU;
use crate::ppu::PPU;
use std::sync::{Arc, Mutex};

pub struct System {
    pub cpu: Arc<Mutex<CPU>>,
    pub ppu: Arc<Mutex<PPU>>,
    // pub apu: Arc<Mutex<APU>>,
    pub rom: Cartridge,
    pub ram: Arc<Mutex<Vec<u8>>>,
    pub vram: Arc<Mutex<Vec<u8>>>,
    pub oam: Arc<Mutex<Vec<u8>>>,
}

impl System {
    pub fn new(rom: Cartridge) -> System {
        System {
            cpu: Arc::new(Mutex::new(CPU::new())),
            ppu: Arc::new(Mutex::new(PPU::new())),
            // apu: Arc::new(Mutex::new(APU::new())),
            rom,
            ram: Arc::new(Mutex::new(vec![0; 0x800])),
            vram: Arc::new(Mutex::new(vec![0; 0x800])),
            oam: Arc::new(Mutex::new(vec![0; 0x100])),
        }
    }
}
