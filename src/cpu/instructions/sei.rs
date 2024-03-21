use crate::cartridge::Cartridge;
use crate::cpu::CPU;
use std::sync::{Arc, Mutex};

pub fn sei(cpu: &mut CPU, instruction: u8, rom: Cartridge, ram: &Arc<Mutex<Vec<u8>>>) -> u64 {
    let mut cycles: u64 = 2;

    match instruction {
        0x78 => {
            // Implied
            cpu.status.interrupt_disable = true;
            cpu.pc += 1;
            cycles = 2;
        }
        _ => {}
    }

    // Sleep for num of cycles
    return cycles;
}
