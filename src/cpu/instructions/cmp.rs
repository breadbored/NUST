use crate::cartridge::Cartridge;
use crate::cpu::CPU;
use crate::system::System;
use std::sync::Arc;
use std::sync::Mutex;

pub fn cmp(
    cpu: &mut CPU,
    instruction: u8,
    operand: u8,
    operand2: u8,
    system: &mut Arc<Mutex<System>>,
) -> u64 {
    let mut cycles: u64 = 2;

    match instruction {
        0xC9 => {
            // Immediate
            let value = cpu.a;
            let result = value.wrapping_sub(operand);
            cpu.status.carry = value >= operand;
            cpu.status.zero = value == operand;
            cpu.status.negative = (result & 0x80) != 0;
            cpu.pc += 2;
            cycles = 2;
        }
        0xC5 => {
            // Zero Page
            let value = cpu.get_mapped_byte(&mut system.clone(), operand as usize);
            let result = cpu.a.wrapping_sub(value);
            cpu.status.carry = cpu.a >= value;
            cpu.status.zero = cpu.a == value;
            cpu.status.negative = (result & 0x80) != 0;
            cpu.pc += 2;
            cycles = 3;
        }
        0xD5 => {
            // Zero Page, X
            let value = cpu.get_mapped_byte(&mut system.clone(), operand as usize + cpu.x as usize);
            let result = cpu.a.wrapping_sub(value);
            cpu.status.carry = cpu.a >= value;
            cpu.status.zero = cpu.a == value;
            cpu.status.negative = (result & 0x80) != 0;
            cpu.pc += 2;
            cycles = 4;
        }
        0xCD => {
            // Absolute
            let value = cpu.get_mapped_byte(
                &mut system.clone(),
                (operand as usize) | ((operand2 as usize) << 8),
            );
            let result = cpu.a.wrapping_sub(value);
            cpu.status.carry = cpu.a >= value;
            cpu.status.zero = cpu.a == value;
            cpu.status.negative = (result & 0x80) != 0;
            cpu.pc += 3;
            cycles = 4;
        }
        0xDD => {
            // Absolute, X
            let value = cpu.get_mapped_byte(
                &mut system.clone(),
                ((operand as usize) | ((operand2 as usize) << 8)).wrapping_add(cpu.x as usize),
            );
            let result = cpu.a.wrapping_sub(value);
            cpu.status.carry = cpu.a >= value;
            cpu.status.zero = cpu.a == value;
            cpu.status.negative = (result & 0x80) != 0;
            cpu.pc += 3;
            cycles = 4;
        }
        0xD9 => {
            // Absolute, Y
            let value = cpu.get_mapped_byte(
                &mut system.clone(),
                ((operand as usize) | ((operand2 as usize) << 8)).wrapping_add(cpu.y as usize),
            );
            let result = cpu.a.wrapping_sub(value);
            cpu.status.carry = cpu.a >= value;
            cpu.status.zero = cpu.a == value;
            cpu.status.negative = (result & 0x80) != 0;
            cpu.pc += 3;
            cycles = 4;
        }
        0xC1 => {
            // Indirect, X
            let value = cpu.get_mapped_byte(
                &mut system.clone(),
                cpu.get_indirect_address(&mut system.clone(), operand, cpu.x) as usize,
            );
            let result = cpu.a.wrapping_sub(value);
            cpu.status.carry = cpu.a >= value;
            cpu.status.zero = cpu.a == value;
            cpu.status.negative = (result & 0x80) != 0;
            cpu.pc += 2;
            cycles = 6;
        }
        0xD1 => {
            // Indirect, Y
            let value = cpu.get_mapped_byte(
                &mut system.clone(),
                cpu.get_indirect_address(&mut system.clone(), operand, cpu.y) as usize,
            );
            let result = cpu.a.wrapping_sub(value);
            cpu.status.carry = cpu.a >= value;
            cpu.status.zero = cpu.a == value;
            cpu.status.negative = (result & 0x80) != 0;
            cpu.pc += 2;
            cycles = 5;
        }
        _ => {}
    }

    // Sleep for num of cycles
    return cycles;
}
