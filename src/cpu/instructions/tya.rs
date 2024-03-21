use crate::cartridge::Cartridge;
use crate::cpu::CPU;
use std::sync::Arc;
use std::sync::Mutex;

pub fn tya(cpu: &mut CPU, instruction: u8, rom: Cartridge, ram: &Arc<Mutex<Vec<u8>>>) -> u64 {
    let mut cycles: u64 = 2;

    match instruction {
        0x98 => {
            // Implied
            cpu.a = cpu.y;
            cpu.pc += 1;
            cycles = 2;
        }
        _ => {}
    }

    // Sleep for num of cycles
    return cycles;
}
