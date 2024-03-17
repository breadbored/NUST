use crate::cartridge::Cartridge;
use crate::cpu::CPU;
use crate::cpu::CPU_CLOCK_SPEED;
use std::sync::{Arc, Mutex};
use std::time::Duration;

pub fn ldy(
    cpu: &mut CPU,
    instruction: u8,
    operand: u8,
    operand2: u8,
    rom: Cartridge,
    ram: &Arc<Mutex<Vec<u8>>>,
) -> u64 {
    let mut cycles: u64 = 2;

    match instruction {
        0xA0 => {
            // Immediate
            cpu.y = operand;
            cpu.pc += 2;
            cycles = 2;
        }
        0xA4 => {
            // Zero Page
            cpu.y = cpu.get_mapped_byte(rom, &ram.clone(), operand as usize);
            cpu.pc += 2;
            cycles = 3;
        }
        0xB4 => {
            // Zero Page, X
            cpu.y = cpu.get_mapped_byte(rom, &ram.clone(), operand as usize + cpu.x as usize);
            cpu.pc += 2;
            cycles = 4;
        }
        0xAC => {
            // Absolute
            cpu.y = cpu.get_mapped_byte(
                rom,
                &ram.clone(),
                (operand2 as usize) | ((operand as usize) << 8),
            );
            cpu.pc += 3;
            cycles = 4;
        }
        0xBC => {
            // Absolute, X
            cpu.y = cpu.get_mapped_byte(
                rom,
                &ram.clone(),
                (operand2 as usize) | ((operand as usize) << 8) + cpu.x as usize,
            );
            cpu.pc += 3;
            cycles = 4;
        }
        _ => {}
    }

    // Sleep for num of cycles
    return cycles;
}
