use crate::cartridge::Cartridge;
use crate::cpu::CPU;
use std::sync::{Arc, Mutex};

pub fn pha(cpu: &mut CPU, instruction: u8, rom: Cartridge, ram: &Arc<Mutex<Vec<u8>>>) -> u64 {
    let mut cycles: u64 = 2;

    match instruction {
        0x48 => {
            // Implied
            cpu.push_stack(&ram.clone(), cpu.a);
            cpu.pc += 1;
            cycles = 3;
        }
        _ => {}
    }

    // Sleep for num of cycles
    return cycles;
}
