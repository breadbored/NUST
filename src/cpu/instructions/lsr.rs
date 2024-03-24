use crate::cartridge::Cartridge;
use crate::cpu::CPU;
use crate::system::System;
use std::sync::{Arc, Mutex};

pub fn lsr(
    cpu: &mut CPU,
    instruction: u8,
    operand: u8,
    operand2: u8,
    system: &mut Arc<Mutex<System>>,
) -> u64 {
    let mut cycles: u64 = 2;

    match instruction {
        0x4A => {
            // Accumulator
            cpu.status.carry = cpu.a & 0x01 != 0;
            cpu.a >>= 1;
            cpu.status.zero = cpu.a == 0;
            cpu.status.negative = cpu.a & 0x80 != 0;
            cpu.pc += 1;
            cycles = 2;
        }
        0x46 => {
            // Zero Page
            let addr = operand as usize;
            let value = cpu.get_mapped_byte(&mut system.clone(), addr);
            cpu.status.carry = value & 0x01 != 0;
            let value = value >> 1;
            cpu.status.zero = value == 0;
            cpu.status.negative = value & 0x80 != 0;
            cpu.pc += 2;
            cycles = 5;
        }
        0x56 => {
            // Zero Page, X
            let addr = (operand + cpu.x) as usize;
            let value = cpu.get_mapped_byte(&mut system.clone(), addr);
            cpu.status.carry = value & 0x01 != 0;
            let value = value >> 1;
            cpu.status.zero = value == 0;
            cpu.status.negative = value & 0x80 != 0;
            cpu.pc += 2;
            cycles = 6;
        }
        0x4E => {
            // Absolute
            let addr = operand as u16 | ((operand2 as u16) << 8);
            let value = cpu.get_mapped_byte(&mut system.clone(), addr as usize);
            cpu.status.carry = value & 0x01 != 0;
            let value = value >> 1;
            cpu.status.zero = value == 0;
            cpu.status.negative = value & 0x80 != 0;
            cpu.pc += 3;
            cycles = 6;
        }
        0x5E => {
            // Absolute, X
            let addr = (operand as u16 | ((operand2 as u16) << 8)).wrapping_add(cpu.x as u16);
            let value = cpu.get_mapped_byte(&mut system.clone(), addr as usize);
            cpu.status.carry = value & 0x01 != 0;
            let value = value >> 1;
            cpu.status.zero = value == 0;
            cpu.status.negative = value & 0x80 != 0;
            cpu.pc += 3;
            cycles = 7;
        }
        _ => {}
    }

    // Sleep for num of cycles
    return cycles;
}
