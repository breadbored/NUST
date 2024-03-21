use crate::cartridge::Cartridge;
use crate::cpu::CPU;
use std::sync::Arc;
use std::sync::Mutex;

pub fn ror(
    cpu: &mut CPU,
    instruction: u8,
    operand: u8,
    operand2: u8,
    rom: Cartridge,
    ram: &Arc<Mutex<Vec<u8>>>,
) -> u64 {
    let mut cycles: u64 = 2;

    match instruction {
        0x6A => {
            // Accumulator
            let old_carry = cpu.status.carry;
            let old_a = cpu.a;
            cpu.a = (cpu.a >> 1) | ((old_carry as u8) << 7);
            cpu.status.zero = cpu.a > 0;
            cpu.status.negative = cpu.a > 0;
            cpu.status.carry = old_a & 0x01 > 0;
            cpu.pc += 1;
            cycles = 2;
        }
        0x66 => {
            // Zero Page
            let mut ram = ram.lock().unwrap();
            let old_carry = cpu.status.carry;
            let old_val = ram[operand as usize];
            ram[operand as usize] = (ram[operand as usize] >> 1) | ((old_carry as u8) << 7);
            cpu.status.zero = ram[operand as usize] > 0;
            cpu.status.negative = ram[operand as usize] > 0;
            cpu.status.carry = old_val & 0x01 > 0;
            cpu.pc += 2;
            cycles = 5;
        }
        0x76 => {
            // Zero Page, X
            let mut ram = ram.lock().unwrap();
            let old_carry = cpu.status.carry;
            let old_val = ram[(operand + cpu.x) as usize];
            ram[(operand + cpu.x) as usize] =
                (ram[(operand + cpu.x) as usize] >> 1) | ((old_carry as u8) << 7);
            cpu.status.zero = ram[(operand + cpu.x) as usize] > 0;
            cpu.status.negative = ram[(operand + cpu.x) as usize] > 0;
            cpu.status.carry = old_val & 0x01 > 0;
            cpu.pc += 2;
            cycles = 6;
        }
        0x6E => {
            // Absolute
            let mut ram = ram.lock().unwrap();
            let old_carry = cpu.status.carry;
            let addr = operand as u16 | ((operand2 as u16) << 8);
            let old_val = ram[addr as usize];
            ram[addr as usize] = (ram[addr as usize] >> 1) | ((old_carry as u8) << 7);
            cpu.status.zero = ram[addr as usize] > 0;
            cpu.status.negative = ram[addr as usize] > 0;
            cpu.status.carry = old_val & 0x01 > 0;
            cpu.pc += 3;
            cycles = 6;
        }
        0x7E => {
            // Absolute, X
            let mut ram = ram.lock().unwrap();
            let old_carry = cpu.status.carry;
            let addr = (operand as u16 | ((operand2 as u16) << 8)).wrapping_add(cpu.x as u16);
            let old_val = ram[addr as usize];
            ram[addr as usize] = (ram[addr as usize] >> 1) | ((old_carry as u8) << 7);
            cpu.status.zero = ram[addr as usize] > 0;
            cpu.status.negative = ram[addr as usize] > 0;
            cpu.status.carry = old_val & 0x01 > 0;
            cpu.pc += 3;
            cycles = 7;
        }
        _ => {}
    }

    // Sleep for num of cycles
    return cycles;
}
