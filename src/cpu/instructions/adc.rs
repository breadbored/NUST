use crate::cpu::CPU;
use crate::system::System;
use std::sync::Arc;
use std::sync::Mutex;

pub fn adc(
    cpu: &mut CPU,
    instruction: u8,
    operand: u8,
    operand2: u8,
    system: &mut Arc<Mutex<System>>,
) -> u64 {
    let mut cycles: u64 = 2;

    match instruction {
        0x69 => {
            // Immediate
            let value = operand;
            let result = cpu.a as u16 + value as u16 + (cpu.status.carry as u8) as u16;
            cpu.status.carry = result > 0xFF;
            cpu.status.overflow = ((cpu.a ^ value) & 0x80) != 0;
            cpu.status.zero = result & 0xFF == 0;
            cpu.a = result as u8;
            cpu.pc += 2;
            cycles = 2;
        }
        0x65 => {
            // Zero Page
            let value = cpu.get_mapped_byte(&mut system.clone(), operand as usize);
            let result = cpu.a as u16 + value as u16 + (cpu.status.carry as u8) as u16;
            cpu.status.carry = result > 0xFF;
            cpu.status.overflow = ((cpu.a ^ value) & 0x80) != 0;
            cpu.status.zero = result & 0xFF == 0;
            cpu.a = result as u8;
            cpu.pc += 2;
            cycles = 3;
        }
        0x75 => {
            // Zero Page, X
            let value = cpu.get_mapped_byte(&mut system.clone(), operand as usize + cpu.x as usize);
            let result = cpu.a as u16 + value as u16 + (cpu.status.carry as u8) as u16;
            cpu.status.carry = result > 0xFF;
            cpu.status.overflow = ((cpu.a ^ value) & 0x80) != 0;
            cpu.status.zero = result & 0xFF == 0;
            cpu.a = result as u8;
            cpu.pc += 2;
            cycles = 4;
        }
        0x6D => {
            // Absolute
            let value = cpu.get_mapped_byte(
                &mut system.clone(),
                (operand as usize) | ((operand2 as usize) << 8),
            );
            let result = cpu.a as u16 + value as u16 + (cpu.status.carry as u8) as u16;
            cpu.status.carry = result > 0xFF;
            cpu.status.overflow = ((cpu.a ^ value) & 0x80) != 0;
            cpu.status.zero = result & 0xFF == 0;
            cpu.a = result as u8;
            cpu.pc += 3;
            cycles = 4;
        }
        0x7D => {
            // Absolute, X
            let value = cpu.get_mapped_byte(
                &mut system.clone(),
                (operand as usize) | ((operand2 as usize) << 8) + cpu.x as usize,
            );
            let result = cpu.a as u16 + value as u16 + (cpu.status.carry as u8) as u16;
            cpu.status.carry = result > 0xFF;
            cpu.status.overflow = ((cpu.a ^ value) & 0x80) != 0;
            cpu.status.zero = result & 0xFF == 0;
            cpu.a = result as u8;
            cpu.pc += 3;
            cycles = 4;
        }
        0x79 => {
            // Absolute, Y
            let value = cpu.get_mapped_byte(
                &mut system.clone(),
                (operand as usize) | ((operand2 as usize) << 8) + cpu.y as usize,
            );
            let result = cpu.a as u16 + value as u16 + (cpu.status.carry as u8) as u16;
            cpu.status.carry = result > 0xFF;
            cpu.status.overflow = ((cpu.a ^ value) & 0x80) != 0;
            cpu.status.zero = result & 0xFF == 0;
            cpu.a = result as u8;
            cpu.pc += 3;
            cycles = 4;
        }
        0x61 => {
            // (Indirect, X)
            let value = cpu.get_mapped_byte(
                &mut system.clone(),
                cpu.get_mapped_byte(&mut system.clone(), operand as usize + cpu.x as usize)
                    as usize
                    | (cpu
                        .get_mapped_byte(&mut system.clone(), operand as usize + cpu.x as usize + 1)
                        as usize)
                        << 8,
            );
            let result = cpu.a as u16 + value as u16 + (cpu.status.carry as u8) as u16;
            cpu.status.carry = result > 0xFF;
            cpu.status.overflow = ((cpu.a ^ value) & 0x80) != 0;
            cpu.status.zero = result & 0xFF == 0;
            cpu.a = result as u8;
            cpu.pc += 2;
            cycles = 6;
        }
        0x71 => {
            // (Indirect), Y
            let value = cpu.get_mapped_byte(
                &mut system.clone(),
                (cpu.get_mapped_byte(&mut system.clone(), operand as usize) as usize
                    | (cpu.get_mapped_byte(&mut system.clone(), operand as usize + 1) as usize)
                        << 8)
                    + cpu.y as usize,
            );
            let result = cpu.a as u16 + value as u16 + (cpu.status.carry as u8) as u16;
            cpu.status.carry = result > 0xFF;
            cpu.status.overflow = ((cpu.a ^ value) & 0x80) != 0;
            cpu.status.zero = result & 0xFF == 0;
            cpu.a = result as u8;
            cpu.pc += 2;
            cycles = 5;
        }
        _ => {}
    }

    // Sleep for num of cycles
    return cycles;
}
