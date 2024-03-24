use crate::cartridge::Cartridge;
use crate::cpu::CPU;
use crate::system::System;
use std::sync::Arc;
use std::sync::Mutex;

pub fn sty(
    cpu: &mut CPU,
    instruction: u8,
    operand: u8,
    operand2: u8,
    system: &mut Arc<Mutex<System>>,
) -> u64 {
    let mut cycles: u64 = 2;

    match instruction {
        0x84 => {
            // Zero Page
            cpu.set_mapped_byte(&mut system.clone(), operand as usize, cpu.y);
            cpu.pc += 2;
            cycles = 3;
        }
        0x94 => {
            // Zero Page, X
            cpu.set_mapped_byte(
                &mut system.clone(),
                operand as usize + cpu.x as usize,
                cpu.y,
            );
            cpu.pc += 2;
            cycles = 4;
        }
        0x8C => {
            // Absolute
            cpu.set_mapped_byte(
                &mut system.clone(),
                (operand as usize) | ((operand2 as usize) << 8),
                cpu.y,
            );
            cpu.pc += 3;
            cycles = 4;
        }
        _ => {}
    }

    // Sleep for num of cycles
    return cycles;
}
