use crate::cartridge::Cartridge;
use crate::cpu::CPU;
use crate::system::System;
use std::sync::Arc;
use std::sync::Mutex;

pub fn lda(
    cpu: &mut CPU,
    instruction: u8,
    operand: u8,
    operand2: u8,
    system: &mut Arc<Mutex<System>>,
) -> u64 {
    let mut cycles: u64 = 2;

    match instruction {
        0xA1 => {
            // (Indirect, X)
            cpu.a = cpu.get_mapped_byte(
                &mut system.clone(),
                (operand as usize) | ((operand2 as usize) << 8) + cpu.x as usize,
            );
            cpu.pc += 2;
            cycles = 6;
        }
        0xB1 => {
            // (Indirect), Y
            cpu.a = cpu.get_mapped_byte(
                &mut system.clone(),
                (operand as usize) | ((operand2 as usize) << 8),
            ) + cpu.y;
            cpu.pc += 2;
            cycles = 5;
        }
        0xA5 => {
            // Zero Page
            cpu.a = cpu.get_mapped_byte(&mut system.clone(), operand as usize);
            cpu.pc += 2;
            cycles = 3;
        }
        0xB5 => {
            // Zero Page, X
            cpu.a = cpu.get_mapped_byte(&mut system.clone(), operand as usize + cpu.x as usize);
            cpu.pc += 2;
            cycles = 4;
        }
        0xA9 => {
            // Immediate
            cpu.a = operand;
            cpu.pc += 2;
            cycles = 2;
        }
        0xB9 => {
            // Absolute, Y
            cpu.a = cpu.get_mapped_byte(
                &mut system.clone(),
                (operand as usize) | ((operand2 as usize) << 8) + cpu.y as usize,
            );
            cpu.pc += 3;
            cycles = 4;
        }
        0xAD => {
            // Absolute
            cpu.a = cpu.get_mapped_byte(
                &mut system.clone(),
                (operand as usize) | ((operand2 as usize) << 8),
            );
            cpu.pc += 3;
            cycles = 4;
        }
        0xBD => {
            // Absolute, X
            cpu.a = cpu.get_mapped_byte(
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
