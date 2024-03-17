use crate::cartridge::Cartridge;
use crate::cpu::CPU;
use crate::cpu::CPU_CLOCK_SPEED;
use std::sync::Arc;
use std::sync::Mutex;
use std::time::Duration;

pub fn sta(
    cpu: &mut CPU,
    instruction: u8,
    operand: u8,
    operand2: u8,
    rom: Cartridge,
    ram: &Arc<Mutex<Vec<u8>>>,
) -> u64 {
    let mut cycles: u64 = 2;

    match instruction {
        0x85 => {
            // Zero Page
            cpu.set_mapped_byte(rom, &ram.clone(), operand as usize, cpu.a);
            cpu.pc += 2;
            cycles = 3;
        }
        0x95 => {
            // Zero Page, X
            cpu.set_mapped_byte(rom, &ram.clone(), operand as usize + cpu.x as usize, cpu.a);
            cpu.pc += 2;
            cycles = 4;
        }
        0x8D => {
            // Absolute
            cpu.set_mapped_byte(
                rom,
                &ram.clone(),
                (operand2 as usize) | ((operand as usize) << 8),
                cpu.a,
            );
            cpu.pc += 3;
            cycles = 4;
        }
        0x9D => {
            // Absolute, X
            cpu.set_mapped_byte(
                rom,
                &ram.clone(),
                (operand2 as usize) | ((operand as usize) << 8) + cpu.x as usize,
                cpu.a,
            );
            cpu.pc += 3;
            cycles = 5;
        }
        0x99 => {
            // Absolute, Y
            cpu.set_mapped_byte(
                rom,
                &ram.clone(),
                (operand2 as usize) | ((operand as usize) << 8) + cpu.y as usize,
                cpu.a,
            );
            cpu.pc += 3;
            cycles = 5;
        }
        0x81 => {
            // (Indirect, X)
            cpu.set_mapped_byte(
                rom,
                &ram.clone(),
                (operand2 as usize) | ((operand as usize) << 8) + cpu.x as usize,
                cpu.a,
            );
            cpu.pc += 2;
            cycles = 6;
        }
        0x91 => {
            // (Indirect), Y
            cpu.set_mapped_byte(
                rom,
                &ram.clone(),
                (operand2 as usize) | ((operand as usize) << 8) + cpu.y as usize,
                cpu.a,
            );
            cpu.pc += 2;
            cycles = 6;
        }
        _ => {}
    }

    // Sleep for num of cycles
    return cycles;
}
