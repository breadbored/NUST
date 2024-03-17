use crate::cartridge::Cartridge;
use crate::cpu::CPU;
use crate::cpu::CPU_CLOCK_SPEED;
use std::sync::Arc;
use std::sync::Mutex;
use std::time::Duration;

pub fn ldx(
    cpu: &mut CPU,
    instruction: u8,
    operand: u8,
    operand2: u8,
    rom: Cartridge,
    ram: &Arc<Mutex<Vec<u8>>>,
) -> u64 {
    let mut cycles: u64 = 2;

    match instruction {
        0xA2 => {
            // Immediate
            cpu.x = operand;
            cpu.pc += 2;
            cycles = 2;
        }
        0xA6 => {
            // Zero Page
            cpu.x = cpu.get_mapped_byte(rom, &ram.clone(), operand as usize);
            cpu.pc += 2;
            cycles = 3;
        }
        0xB6 => {
            // Zero Page, Y
            cpu.x = cpu.get_mapped_byte(rom, &ram.clone(), operand as usize + cpu.y as usize);
            cpu.pc += 2;
            cycles = 4;
        }
        0xAE => {
            // Absolute
            cpu.x = cpu.get_mapped_byte(
                rom,
                &ram.clone(),
                (operand2 as usize) | ((operand as usize) << 8),
            );
            cpu.pc += 3;
            cycles = 4;
        }
        0xBE => {
            // Absolute, Y
            cpu.x = cpu.get_mapped_byte(
                rom,
                &ram.clone(),
                (operand2 as usize) | ((operand as usize) << 8) + cpu.y as usize,
            );
            cpu.pc += 3;
            cycles = 4;
        }
        _ => {}
    }

    // Sleep for num of cycles
    return cycles;
}
