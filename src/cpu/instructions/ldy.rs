use crate::cartridge::Cartridge;
use crate::cpu::CPU;
use crate::system::System;
use std::sync::{Arc, Mutex};

pub fn ldy(
    cpu: &mut CPU,
    instruction: u8,
    operand: u8,
    operand2: u8,
    system: &mut Arc<Mutex<System>>,
) -> u64 {
    let mut cycles: u64 = 2;

    match instruction {
        0xA0 => {
            // Immediate
            cpu.y = operand;
            cpu.pc += 2;
            cycles = 2;
        }
        0xA4 => {
            // Zero Page
            cpu.y = cpu.get_mapped_byte(&mut system.clone(), operand as usize);
            cpu.pc += 2;
            cycles = 3;
        }
        0xB4 => {
            // Zero Page, X
            cpu.y = cpu.get_mapped_byte(&mut system.clone(), operand as usize + cpu.x as usize);
            cpu.pc += 2;
            cycles = 4;
        }
        0xAC => {
            // Absolute
            cpu.y = cpu.get_mapped_byte(
                &mut system.clone(),
                (operand as usize) | ((operand2 as usize) << 8),
            );
            cpu.pc += 3;
            cycles = 4;
        }
        0xBC => {
            // Absolute, X
            cpu.y = cpu.get_mapped_byte(
                &mut system.clone(),
                (operand as usize) | ((operand2 as usize) << 8) + cpu.x as usize,
            );
            cpu.pc += 3;
            cycles = 4;
        }
        _ => {}
    }

    // Sleep for num of cycles
    return cycles;
}
