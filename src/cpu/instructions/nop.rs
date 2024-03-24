use crate::cpu::CPU;
use crate::system::System;

pub fn nop(cpu: &mut CPU, instruction: u8) -> u64 {
    // println!("NOP");
    cpu.pc += 1;

    let mut cycles: u64 = 2;

    match instruction {
        0x1A | 0x3A | 0x5A | 0x7A | 0xDA | 0xFA | 0x80 | 0x82 | 0x89 | 0xC2 | 0xE2 => {
            // Implied
            // 2 cycles
            cycles = 2;
        }
        0x04 | 0x44 | 0x64 => {
            // Zero Page
            // 3 cycles
            cycles = 3;
        }
        0x14 | 0x34 | 0x54 | 0x74 | 0xD4 | 0xF4 | 0x0C | 0x1C | 0x3C | 0x5C | 0x7C | 0xDC
        | 0xFC => {
            // Absolute & Zero Page, X
            // 4 cycles
            cycles = 4;
        }
        _ => {}
    }

    // Sleep for num of cycles
    return cycles;
}
