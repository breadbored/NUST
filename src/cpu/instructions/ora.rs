use crate::cartridge::Cartridge;
use crate::cpu::CPU;
use std::sync::{Arc, Mutex};

pub fn ora(
    cpu: &mut CPU,
    instruction: u8,
    operand: u8,
    rom: Cartridge,
    ram: &Arc<Mutex<Vec<u8>>>,
) -> u64 {
    let mut cycles: u64 = 2;

    match instruction {
        0x01 => {
            // (Indirect, X)
            cpu.a |= cpu.get_mapped_byte(rom, &ram.clone(), operand as usize + cpu.x as usize);
            cpu.pc += 2;
            cycles = 6;
        }
        0x11 => {
            // (Indirect), Y
            cpu.a |= cpu.get_mapped_byte(rom, &ram.clone(), operand as usize) + cpu.y;
            cpu.pc += 2;
            cycles = 5;
        }
        0x05 => {
            // Zero Page
            cpu.a |= cpu.get_mapped_byte(rom, &ram.clone(), operand as usize);
            cpu.pc += 2;
            cycles = 3;
        }
        0x15 => {
            // Zero Page, X
            cpu.a |= cpu.get_mapped_byte(rom, &ram.clone(), operand as usize + cpu.x as usize);
            cpu.pc += 2;
            cycles = 4;
        }
        0x09 => {
            // Immediate
            cpu.a |= operand;
            cpu.pc += 2;
            cycles = 2;
        }
        0x19 => {
            // Absolute, Y
            cpu.a |= cpu.get_mapped_byte(rom, &ram.clone(), operand as usize) + cpu.y;
            cpu.pc += 3;
            cycles = 4;
        }
        0x0D => {
            // Absolute
            cpu.a |= cpu.get_mapped_byte(rom, &ram.clone(), operand as usize);
            cpu.pc += 3;
            cycles = 4;
        }
        0x1D => {
            // Absolute, X
            cpu.a |= cpu.get_mapped_byte(rom, &ram.clone(), operand as usize + cpu.x as usize);
            cpu.pc += 3;
            cycles = 4;
        }
        _ => {}
    }

    return cycles;
}
