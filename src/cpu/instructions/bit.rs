use crate::cpu::CPU;
use crate::system::System;
use std::sync::Arc;
use std::sync::Mutex;

pub fn bit(
    cpu: &mut CPU,
    instruction: u8,
    operand: u8,
    operand2: u8,
    system: &mut Arc<Mutex<System>>,
) -> u64 {
    let mut cycles: u64 = 2;

    match instruction {
        0x24 => {
            // Zero Page
            let value = cpu.get_mapped_byte(&mut system.clone(), operand as usize);
            cpu.status.zero = (value & cpu.a) == 0;
            cpu.status.overflow = (value & 0x40) != 0;
            cpu.status.negative = (value & 0x80) != 0;
            cpu.pc += 2;
            cycles = 3;
        }
        0x2C => {
            // Absolute
            let value = cpu.get_mapped_byte(
                &mut system.clone(),
                (operand as usize) | ((operand2 as usize) << 8),
            );
            cpu.status.zero = (value & cpu.a) == 0;
            cpu.status.overflow = (value & 0x40) != 0;
            cpu.status.negative = (value & 0x80) != 0;
            cpu.pc += 3;
            cycles = 4;
        }
        _ => {}
    }

    // Sleep for num of cycles
    return cycles;
}
