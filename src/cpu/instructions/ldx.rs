use crate::cartridge::Cartridge;
use crate::cpu::CPU;
use crate::system::System;
use std::sync::Arc;
use std::sync::Mutex;

pub fn ldx(
    cpu: &mut CPU,
    instruction: u8,
    operand: u8,
    operand2: u8,
    system: &mut Arc<Mutex<System>>,
) -> u64 {
    let mut cycles: u64 = 2;

    match instruction {
        0xA2 => {
            // Immediate
            cpu.x = operand;
            cpu.pc += 2;
            cycles = 2;
        }
        0xA6 => {
            // Zero Page
            cpu.x = cpu.get_mapped_byte(&mut system.clone(), operand as usize);
            cpu.pc += 2;
            cycles = 3;
        }
        0xB6 => {
            // Zero Page, Y
            cpu.x = cpu.get_mapped_byte(&mut system.clone(), operand as usize + cpu.y as usize);
            cpu.pc += 2;
            cycles = 4;
        }
        0xAE => {
            // Absolute
            cpu.x = cpu.get_mapped_byte(
                &mut system.clone(),
                (operand as usize) | ((operand2 as usize) << 8),
            );
            cpu.pc += 3;
            cycles = 4;
        }
        0xBE => {
            // Absolute, Y
            cpu.x = cpu.get_mapped_byte(
                &mut system.clone(),
                (operand as usize) | ((operand2 as usize) << 8) + cpu.y as usize,
            );
            cpu.pc += 3;
            cycles = 4;
        }
        _ => {}
    }

    // Sleep for num of cycles
    return cycles;
}
