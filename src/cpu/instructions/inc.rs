use crate::cartridge::Cartridge;
use crate::cpu::CPU;
use std::sync::Arc;
use std::sync::Mutex;

pub fn inc(
    cpu: &mut CPU,
    instruction: u8,
    operand: u8,
    operand2: u8,
    rom: Cartridge,
    ram: &Arc<Mutex<Vec<u8>>>,
) -> u64 {
    let mut cycles: u64 = 2;

    match instruction {
        0xE6 => {
            // Zero Page
            let addr = operand as usize;
            let value = cpu
                .get_mapped_byte(rom.clone(), &ram.clone(), addr)
                .wrapping_add(1);
            cpu.set_mapped_byte(rom.clone(), &ram.clone(), addr, value);
            cpu.status.zero = value == 0;
            cpu.status.negative = value & 0x80 != 0;
            cpu.pc += 2;
            cycles = 5;
        }
        0xF6 => {
            // Zero Page, X
            let addr = (operand + cpu.x) as usize;
            let value = cpu
                .get_mapped_byte(rom.clone(), &ram.clone(), addr)
                .wrapping_add(1);
            cpu.set_mapped_byte(rom.clone(), &ram.clone(), addr, value);
            cpu.status.zero = value == 0;
            cpu.status.negative = value & 0x80 != 0;
            cpu.pc += 2;
            cycles = 6;
        }
        0xEE => {
            // Absolute
            let addr = operand as u16 | ((operand2 as u16) << 8);
            let value = cpu
                .get_mapped_byte(rom.clone(), &ram.clone(), addr as usize)
                .wrapping_add(1);
            cpu.set_mapped_byte(rom.clone(), &ram.clone(), addr as usize, value);
            cpu.status.zero = value == 0;
            cpu.status.negative = value & 0x80 != 0;
            cpu.pc += 3;
            cycles = 6;
        }
        0xFE => {
            // Absolute, X
            let addr = (operand as u16 | ((operand2 as u16) << 8)).wrapping_add(cpu.x as u16);
            let value = cpu
                .get_mapped_byte(rom.clone(), &ram.clone(), addr as usize)
                .wrapping_add(1);
            cpu.set_mapped_byte(rom.clone(), &ram.clone(), addr as usize, value);
            cpu.status.zero = value == 0;
            cpu.status.negative = value & 0x80 != 0;
            cpu.pc += 3;
            cycles = 7;
        }
        _ => {}
    }

    // Sleep for num of cycles
    return cycles;
}
