use crate::cartridge::Cartridge;
use crate::cpu::CPU;
use std::sync::Arc;
use std::sync::Mutex;

pub fn jmp(
    cpu: &mut CPU,
    instruction: u8,
    operand: u8,
    operand2: u8,
    rom: Cartridge,
    ram: &Arc<Mutex<Vec<u8>>>,
) -> u64 {
    let mut cycles: u64 = 2;

    match instruction {
        0x4C => {
            // Absolute
            let addr = (operand as u16) | ((operand2 as u16) << 8);
            cpu.pc = addr;
            cycles = 3;
        }
        0x6C => {
            // Indirect
            let addr = (operand as usize) | ((operand2 as usize) << 8);
            cpu.pc = cpu.get_mapped_word(rom, &ram.clone(), addr);
            cycles = 5;
        }
        _ => {}
    }

    // Sleep for num of cycles
    return cycles;
}
