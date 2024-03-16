mod cartridge;
mod cpu;

use cartridge::{get_rom, Cartridge};
use std::{
    process::exit,
    sync::{Arc, Mutex},
    time::{Duration, SystemTime},
};

fn main() {
    let rom: Cartridge = get_rom();
    let ram: Arc<Mutex<Vec<u8>>> = Arc::new(Mutex::new(vec![0; 0x800]));
    let vram: Arc<Mutex<Vec<u8>>> = Arc::new(Mutex::new(vec![0; 0x800]));

    // CPU Thread
    let cpu_rom = rom.clone();
    let cpu_ram = ram.clone();
    let cpu_vram = vram.clone();
    let cpu_thread = std::thread::spawn(move || {
        const CPU_CYCLES: u128 = 559; // 1.79 MHz

        let mut cpu = cpu::CPU::new();

        let mut last_cycle = get_time();
        loop {
            // CPU runs at 1.79 MHz
            let check_cpu_time = get_time();
            if check_cpu_time - last_cycle >= CPU_CYCLES {
                cpu.tick(cpu_rom.clone(), &cpu_ram, &cpu_vram);
            }

            // Check if we need to sleep
            let current_time = get_time();
            if current_time - last_cycle < CPU_CYCLES {
                std::thread::sleep(Duration::from_nanos(
                    (CPU_CYCLES - (current_time - last_cycle)) as u64,
                ));
            }

            last_cycle = get_time();
        }
    });

    // PPU Thread
    let ppu_vram = vram.clone();
    let ppu_thread = std::thread::spawn(|| {
        const PPU_CYCLES: u128 = 186; // 5.37 MHz
        let mut last_cycle = get_time();
        loop {
            // PPU runs at 5.37 MHz
            let check_ppu_time = get_time();
            if check_ppu_time - last_cycle >= PPU_CYCLES {
                // PPU.tick(&vram);
            }

            // Check if we need to sleep
            let current_time = get_time();
            if current_time - last_cycle < PPU_CYCLES {
                std::thread::sleep(Duration::from_nanos(
                    (PPU_CYCLES - (current_time - last_cycle)) as u64,
                ));
            }

            last_cycle = get_time();
        }
    });

    // APU Thread
    let apu_rom = rom.clone();
    let apu_ram = ram.clone();
    let apu_vram = vram.clone();
    let apu_thread = std::thread::spawn(|| {
        const APU_CYCLES: u128 = 559; // 1.79 MHz
        let mut last_cycle = get_time();
        loop {
            // APU runs at 1.79 MHz
            let check_apu_time = get_time();
            if check_apu_time - last_cycle >= APU_CYCLES {
                // APU.tick();
            }

            // Check if we need to sleep
            let current_time = get_time();
            if current_time - last_cycle < APU_CYCLES {
                std::thread::sleep(Duration::from_nanos(
                    (APU_CYCLES - (current_time - last_cycle)) as u64,
                ));
            }

            last_cycle = get_time();
        }
    });

    cpu_thread.join().unwrap();
    ppu_thread.join().unwrap();
    apu_thread.join().unwrap();
}

fn get_time() -> u128 {
    return SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_nanos();
}
