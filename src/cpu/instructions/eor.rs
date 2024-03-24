use crate::cartridge::Cartridge;
use crate::cpu::CPU;
use crate::system::System;
use std::sync::Arc;
use std::sync::Mutex;

pub fn eor(
    cpu: &mut CPU,
    instruction: u8,
    operand: u8,
    operand2: u8,
    system: &mut Arc<Mutex<System>>,
) -> u64 {
    let mut cycles: u64 = 2;

    match instruction {
        0x49 => {
            // Immediate
            let value = cpu.a ^ operand;
            cpu.status.zero = value == 0;
            cpu.status.negative = value & 0x80 != 0;
            cpu.a = value;
            cpu.pc += 2;
            cycles = 2;
        }
        0x45 => {
            // Zero Page
            let addr = operand as usize;
            let value = cpu.get_mapped_byte(&mut system.clone(), addr) ^ 0;
            cpu.status.zero = value == 0;
            cpu.status.negative = value & 0x80 != 0;
            cpu.a = value;
            cpu.pc += 2;
            cycles = 3;
        }
        0x55 => {
            // Zero Page, X
            let addr = (operand + cpu.x) as usize;
            let value = cpu.get_mapped_byte(&mut system.clone(), addr) ^ 0;
            cpu.status.zero = value == 0;
            cpu.status.negative = value & 0x80 != 0;
            cpu.a = value;
            cpu.pc += 2;
            cycles = 4;
        }
        0x4D => {
            // Absolute
            let addr = operand as u16 | ((operand2 as u16) << 8);
            let value = cpu.get_mapped_byte(&mut system.clone(), addr as usize) ^ 0;
            cpu.status.zero = value == 0;
            cpu.status.negative = value & 0x80 != 0;
            cpu.a = value;
            cpu.pc += 3;
            cycles = 4;
        }
        0x5D => {
            // Absolute, X
            let addr = (operand as u16 | ((operand2 as u16) << 8)).wrapping_add(cpu.x as u16);
            let value = cpu.get_mapped_byte(&mut system.clone(), addr as usize) ^ 0;
            cpu.status.zero = value == 0;
            cpu.status.negative = value & 0x80 != 0;
            cpu.a = value;
            cpu.pc += 3;
            cycles = 4;
        }
        0x59 => {
            // Absolute, Y
            let addr = (operand as u16 | ((operand2 as u16) << 8)).wrapping_add(cpu.y as u16);
            let value = cpu.get_mapped_byte(&mut system.clone(), addr as usize) ^ 0;
            cpu.status.zero = value == 0;
            cpu.status.negative = value & 0x80 != 0;
            cpu.a = value;
            cpu.pc += 3;
            cycles = 4;
        }
        0x41 => {
            // Indirect, X
            let addr = (operand + cpu.x) as usize;
            let indirect_addr = cpu.get_mapped_byte(&mut system.clone(), addr) as u16
                | ((cpu.get_mapped_byte(&mut system.clone(), addr + 1) as u16) << 8);
            let value = cpu.get_mapped_byte(&mut system.clone(), indirect_addr as usize) ^ 0;
            cpu.status.zero = value == 0;
            cpu.status.negative = value & 0x80 != 0;
            cpu.a = value;
            cpu.pc += 2;
            cycles = 6;
        }
        0x51 => {
            // Indirect, Y
            let addr = operand as usize;
            let indirect_addr = cpu.get_mapped_byte(&mut system.clone(), addr) as u16
                | ((cpu.get_mapped_byte(&mut system.clone(), addr + 1) as u16) << 8);
            let value = cpu.get_mapped_byte(&mut system.clone(), indirect_addr as usize) ^ 0;
            cpu.status.zero = value == 0;
            cpu.status.negative = value & 0x80 != 0;
            cpu.a = value;
            cpu.pc += 2;
            cycles = 5;
        }
        _ => {}
    }

    // Sleep for num of cycles
    return cycles;
}
