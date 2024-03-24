use crate::cartridge::Cartridge;
use crate::cpu::CPU;
use crate::system::System;
use std::sync::Arc;
use std::sync::Mutex;

pub fn dey(cpu: &mut CPU, instruction: u8, system: &mut Arc<Mutex<System>>) -> u64 {
    let mut cycles: u64 = 2;

    match instruction {
        0xCA => {
            // Implied
            let value = cpu.y.wrapping_sub(1);
            cpu.status.carry = cpu.y > value;
            cpu.status.zero = value == 0;
            cpu.y = value;
            cpu.pc += 1;
            cycles = 2;
        }
        _ => {}
    }

    // Sleep for num of cycles
    return cycles;
}
