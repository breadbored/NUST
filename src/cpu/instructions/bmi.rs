use crate::cpu::CPU;
use crate::system::System;
use std::sync::Arc;
use std::sync::Mutex;

pub fn bmi(cpu: &mut CPU, instruction: u8, operand: u8, system: &mut Arc<Mutex<System>>) -> u64 {
    let mut cycles: u64 = 2;

    match instruction {
        0x30 => {
            // Relative
            if cpu.status.negative {
                let jump: i16 = operand as i8 as i16;
                if (cpu.pc & 0xFF00) != ((cpu.pc.wrapping_add(jump as u16)) & 0xFF00) {
                    cycles += 1;
                }
                cpu.pc = cpu.pc.wrapping_add(jump as u16);
                cycles += 1;
            }
            cpu.pc += 2;
        }
        _ => {}
    }

    // Sleep for num of cycles
    return cycles;
}
