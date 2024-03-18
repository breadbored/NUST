use crate::cartridge::Cartridge;
use crate::cpu::CPU;
use std::sync::Arc;
use std::sync::Mutex;

pub fn beq(
    cpu: &mut CPU,
    instruction: u8,
    operand: u8,
    rom: Cartridge,
    ram: &Arc<Mutex<Vec<u8>>>,
) -> u64 {
    let mut cycles: u64 = 2;

    match instruction {
        0xF0 => {
            // Relative
            if cpu.status.zero {
                let offset = operand as i8;
                cpu.pc += 2;
                let new_addr = cpu.pc.wrapping_add(offset as u16);
                // If the branch is taken, page boundary crossing should be checked to adjust `cycles`
                cycles = if (cpu.pc & 0xFF00) != (new_addr & 0xFF00) {
                    4
                } else {
                    3
                };
                cpu.pc = new_addr; // Branch is taken, update the PC
            } else {
                cpu.pc += 2; // Branch not taken, move to the next instruction
                cycles = 2;
            }
        }
        _ => {}
    }

    // Sleep for num of cycles
    return cycles;
}
