use crate::cartridge::Cartridge;
use crate::cpu::CPU;
use crate::system::System;
use std::sync::Arc;
use std::sync::Mutex;

pub fn tax(cpu: &mut CPU, instruction: u8, system: &mut Arc<Mutex<System>>) -> u64 {
    let mut cycles: u64 = 2;

    match instruction {
        0xAA => {
            // Implied
            cpu.x = cpu.a;
            cpu.pc += 1;
            cpu.status.zero = cpu.x == 0;
            cpu.status.negative = cpu.x & 0x80 != 0;
            cycles = 2;
        }
        _ => {}
    }

    // Sleep for num of cycles
    return cycles;
}
