use crate::cartridge::Cartridge;
use crate::cpu::CPU;
use crate::system::System;
use std::sync::Arc;
use std::sync::Mutex;

pub fn rol(
    cpu: &mut CPU,
    instruction: u8,
    operand: u8,
    operand2: u8,
    system: &mut Arc<Mutex<System>>,
) -> u64 {
    let mut cycles: u64 = 2;

    match instruction {
        0x2A => {
            // Accumulator
            let ram = system.lock().unwrap().ram.clone();
            let carry = cpu.status.carry as u8;
            let new_carry = (cpu.a & 0x80) >> 7;
            cpu.a = (cpu.a << 1) | carry;
            cpu.status.carry = new_carry == 1;
            cpu.status.zero = cpu.a == 0;
            cpu.status.negative = (cpu.a & 0x80) == 0x80;
            cpu.pc += 1;
            cycles = 2;
        }
        0x26 => {
            // Zero Page
            let ram = system.lock().unwrap().ram.clone();
            let carry = cpu.status.carry as u8;
            let addr: u16 = operand as u16;
            let value = ram.lock().unwrap()[addr as usize];
            let new_carry = (value & 0x80) >> 7;
            let result = (value << 1) | carry;
            ram.lock().unwrap()[addr as usize] = result;
            cpu.status.carry = new_carry == 1;
            cpu.status.zero = result == 0;
            cpu.status.negative = (result & 0x80) == 0x80;
            cpu.pc += 2;
            cycles = 5;
        }
        0x36 => {
            // Zero Page, X
            let ram = system.lock().unwrap().ram.clone();
            let carry = cpu.status.carry as u8;
            let addr: u16 = (operand + cpu.x) as u16;
            let value = ram.lock().unwrap()[addr as usize];
            let new_carry = (value & 0x80) >> 7;
            let result = (value << 1) | carry;
            ram.lock().unwrap()[addr as usize] = result;
            cpu.status.carry = new_carry == 1;
            cpu.status.zero = result == 0;
            cpu.status.negative = (result & 0x80) == 0x80;
            cpu.pc += 2;
            cycles = 6;
        }
        0x2E => {
            // Absolute
            let ram = system.lock().unwrap().ram.clone();
            let carry = cpu.status.carry as u8;
            let addr: u16 = operand as u16 | ((operand2 as u16) << 8);
            let value = ram.lock().unwrap()[addr as usize];
            let new_carry = (value & 0x80) >> 7;
            let result = (value << 1) | carry;
            ram.lock().unwrap()[addr as usize] = result;
            cpu.status.carry = new_carry == 1;
            cpu.status.zero = result == 0;
            cpu.status.negative = (result & 0x80) == 0x80;
            cpu.pc += 3;
            cycles = 6;
        }
        0x3E => {
            // Absolute, X
            let ram = system.lock().unwrap().ram.clone();
            let carry = cpu.status.carry as u8;
            let addr: u16 = (operand as u16 | ((operand2 as u16) << 8)).wrapping_add(cpu.x as u16);
            let value = ram.lock().unwrap()[addr as usize];
            let new_carry = (value & 0x80) >> 7;
            let result = (value << 1) | carry;
            ram.lock().unwrap()[addr as usize] = result;
            cpu.status.carry = new_carry == 1;
            cpu.status.zero = result == 0;
            cpu.status.negative = (result & 0x80) == 0x80;
            cpu.pc += 3;
            cycles = 7;
        }
        _ => {}
    }

    // Sleep for num of cycles
    return cycles;
}
