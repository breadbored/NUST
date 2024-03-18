use crate::cartridge::Cartridge;
use crate::cpu::CPU;
use std::sync::Arc;
use std::sync::Mutex;

pub fn bcc(
    cpu: &mut CPU,
    instruction: u8,
    operand: u8,
    rom: Cartridge,
    ram: &Arc<Mutex<Vec<u8>>>,
) -> u64 {
    let mut cycles: u64 = 2;

    match instruction {
        0x90 => {
            // Relative
            if !cpu.status.carry {
                let addr = cpu.pc.wrapping_add(operand as i8 as u16);
                cpu.pc = addr;
                cycles = if (cpu.pc & 0xFF00) != (addr & 0xFF00) {
                    4
                } else {
                    3
                };
            } else {
                cpu.pc += 2;
            }
        }
        _ => {}
    }

    // Sleep for num of cycles
    return cycles;
}
