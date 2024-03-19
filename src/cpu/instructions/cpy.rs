use crate::cartridge::Cartridge;
use crate::cpu::CPU;
use std::sync::Arc;
use std::sync::Mutex;

pub fn cpy(
    cpu: &mut CPU,
    instruction: u8,
    operand: u8,
    operand2: u8,
    rom: Cartridge,
    ram: &Arc<Mutex<Vec<u8>>>,
) -> u64 {
    let mut cycles: u64 = 2;

    match instruction {
        0xC0 => {
            // Immediate
            let value = cpu.y.wrapping_sub(operand);
            cpu.status.carry = cpu.y >= operand;
            cpu.status.zero = value == 0;
            cpu.status.negative = value & 0x80 != 0;
            cpu.pc += 2;
            cycles = 2;
        }
        0xC4 => {
            // Zero Page
            let addr = operand as usize;
            let value = cpu
                .get_mapped_byte(rom.clone(), &ram.clone(), addr)
                .wrapping_sub(1);
            cpu.status.carry = cpu.y >= value;
            cpu.status.zero = value == 0;
            cpu.status.negative = value & 0x80 != 0;
            cpu.pc += 2;
            cycles = 3;
        }
        _ => {}
    }

    // Sleep for num of cycles
    return cycles;
}
