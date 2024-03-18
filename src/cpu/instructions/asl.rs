use crate::cartridge::Cartridge;
use crate::cpu::CPU;
use std::sync::Arc;
use std::sync::Mutex;

pub fn asl(
    cpu: &mut CPU,
    instruction: u8,
    operand: u8,
    operand2: u8,
    rom: Cartridge,
    ram: &Arc<Mutex<Vec<u8>>>,
) -> u64 {
    let mut cycles: u64 = 2;

    match instruction {
        0x0A => {
            // Accumulator
            let carry = cpu.a & 0x80;
            cpu.a = cpu.a << 1;
            cpu.status.carry = carry > 0;
            cpu.status.zero = cpu.a == 0;
            cpu.pc += 1;
            cycles = 2;
        }
        0x06 => {
            // Zero Page
            let addr = operand as usize;
            let value = cpu.get_mapped_byte(rom.clone(), &ram.clone(), addr);
            let carry = value & 0x80;
            let result = value << 1;
            cpu.set_mapped_byte(rom.clone(), &ram.clone(), addr, result);
            cpu.status.carry = carry > 0;
            cpu.status.zero = cpu.a == 0;
            cpu.pc += 2;
            cycles = 5;
        }
        0x16 => {
            // Zero Page, X
            let addr = (operand + cpu.x) as usize;
            let value = cpu.get_mapped_byte(rom.clone(), &ram.clone(), addr);
            let carry = value & 0x80;
            let result = value << 1;
            cpu.set_mapped_byte(rom.clone(), &ram.clone(), addr, result);
            cpu.status.carry = carry > 0;
            cpu.status.zero = cpu.a == 0;
            cpu.pc += 2;
            cycles = 6;
        }
        0x0E => {
            // Absolute
            let addr = (operand as usize) | ((operand2 as usize) << 8);
            let value = cpu.get_mapped_byte(rom.clone(), &ram.clone(), addr);
            let carry = value & 0x80;
            let result = value << 1;
            cpu.set_mapped_byte(rom.clone(), &ram.clone(), addr, result);
            cpu.status.carry = carry > 0;
            cpu.status.zero = cpu.a == 0;
            cpu.pc += 3;
            cycles = 6;
        }
        0x1E => {
            // Absolute, X
            let addr = (operand as usize) | ((operand2 as usize) << 8) + cpu.x as usize;
            let value = cpu.get_mapped_byte(rom.clone(), &ram.clone(), addr);
            let carry = value & 0x80;
            let result = value << 1;
            cpu.set_mapped_byte(rom.clone(), &ram.clone(), addr, result);
            cpu.status.carry = carry > 0;
            cpu.status.zero = cpu.a == 0;
            cpu.pc += 3;
            cycles = 7;
        }
        _ => {}
    }

    // Sleep for num of cycles
    return cycles;
}
