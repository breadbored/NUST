use crate::cartridge::Cartridge;
use crate::cpu::CPU;
use crate::system::System;
use std::sync::Arc;
use std::sync::Mutex;

pub fn bne(cpu: &mut CPU, instruction: u8, operand: u8, system: &mut Arc<Mutex<System>>) -> u64 {
    let mut cycles: u64 = 2;

    match instruction {
        0xD0 => {
            // Relative
            if !cpu.status.zero {
                let offset = operand as i8;
                let new_addr = cpu.pc.wrapping_add(2).wrapping_add(offset as u16);
                cycles = if (cpu.pc & 0xFF00) != (new_addr & 0xFF00) {
                    4
                } else {
                    3
                };
                cpu.pc = new_addr;
            } else {
                cpu.pc += 2;
                cycles = 2;
            }
        }
        _ => {}
    }

    // Sleep for num of cycles
    return cycles;
}
