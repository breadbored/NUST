use crate::cartridge::Cartridge;
use crate::cpu::CPU;
use std::sync::Arc;
use std::sync::Mutex;

pub fn tay(cpu: &mut CPU, instruction: u8, rom: Cartridge, ram: &Arc<Mutex<Vec<u8>>>) -> u64 {
    let mut cycles: u64 = 2;

    match instruction {
        0xA8 => {
            // Implied
            cpu.y = cpu.a;
            cpu.pc += 1;
            cpu.status.zero = cpu.y == 0;
            cpu.status.negative = cpu.y & 0x80 != 0;
            cycles = 2;
        }
        _ => {}
    }

    // Sleep for num of cycles
    return cycles;
}
