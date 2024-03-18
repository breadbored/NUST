use crate::cartridge::Cartridge;
use crate::cpu::CPU;
use std::sync::Arc;
use std::sync::Mutex;

pub fn and(
    cpu: &mut CPU,
    instruction: u8,
    operand: u8,
    operand2: u8,
    rom: Cartridge,
    ram: &Arc<Mutex<Vec<u8>>>,
) -> u64 {
    let mut cycles: u64 = 2;

    match instruction {
        0x29 => {
            // Immediate
            cpu.a &= operand;
            cpu.pc += 2;
            cycles = 2;
        }
        0x25 => {
            // Zero Page
            cpu.a &= cpu.get_mapped_byte(rom, &ram.clone(), operand as usize);
            cpu.pc += 2;
            cycles = 3;
        }
        0x35 => {
            // Zero Page, X
            cpu.a &= cpu.get_mapped_byte(rom, &ram.clone(), operand as usize + cpu.x as usize);
            cpu.pc += 2;
            cycles = 4;
        }
        0x2D => {
            // Absolute
            cpu.a &= cpu.get_mapped_byte(
                rom,
                &ram.clone(),
                (operand as usize) | ((operand2 as usize) << 8),
            );
            cpu.pc += 3;
            cycles = 4;
        }
        0x3D => {
            // Absolute, X
            cpu.a &= cpu.get_mapped_byte(
                rom,
                &ram.clone(),
                (operand as usize) | ((operand2 as usize) << 8) + cpu.x as usize,
            );
            cpu.pc += 3;
            cycles = 4;
        }
        0x39 => {
            // Absolute, Y
            cpu.a &= cpu.get_mapped_byte(
                rom,
                &ram.clone(),
                (operand as usize) | ((operand2 as usize) << 8) + cpu.y as usize,
            );
            cpu.pc += 3;
            cycles = 4;
        }
        0x21 => {
            // Indirect, X
            let addr = cpu.get_mapped_byte(rom, &ram.clone(), operand as usize + cpu.x as usize);
            let addr2 = cpu.get_mapped_byte(
                rom,
                &ram.clone(),
                (operand as usize + cpu.x as usize + 1) % 256,
            );
            let addr = (addr as usize) | ((addr2 as usize) << 8);
            cpu.a &= cpu.get_mapped_byte(rom, &ram.clone(), addr);
            cpu.pc += 2;
            cycles = 6;
        }
        0x31 => {
            // Indirect, Y
            let addr = cpu.get_mapped_byte(rom, &ram.clone(), operand as usize);
            let addr2 = cpu.get_mapped_byte(rom, &ram.clone(), (operand as usize + 1) % 256);
            let addr = (addr as usize) | ((addr2 as usize) << 8);
            cpu.a &= cpu.get_mapped_byte(rom, &ram.clone(), addr + cpu.y as usize);
            cpu.pc += 2;
            cycles = 5;
        }
        _ => {}
    }

    // Sleep for num of cycles
    return cycles;
}
