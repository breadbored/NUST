use crate::cartridge::Cartridge;
use crate::cpu::CPU;
use crate::system::System;
use std::sync::Arc;
use std::sync::Mutex;

pub fn cpx(
    cpu: &mut CPU,
    instruction: u8,
    operand: u8,
    operand2: u8,
    system: &mut Arc<Mutex<System>>,
) -> u64 {
    let mut cycles: u64 = 2;

    match instruction {
        0xE0 => {
            // Immediate
            let value = cpu.x.wrapping_sub(operand);
            cpu.status.carry = cpu.x >= operand;
            cpu.status.zero = value == 0;
            cpu.status.negative = (value & 0x80) == 0x80;
            cpu.pc += 2;
            cycles = 2;
        }
        0xE4 => {
            // Zero Page
            let addr = operand as usize;
            let value = cpu
                .get_mapped_byte(&mut system.clone(), addr)
                .wrapping_sub(cpu.x);
            cpu.status.carry = cpu.x >= value;
            cpu.status.zero = value == 0;
            cpu.status.negative = (value & 0x80) == 0x80;
            cpu.pc += 2;
            cycles = 3;
        }
        _ => {}
    }

    // Sleep for num of cycles
    return cycles;
}
