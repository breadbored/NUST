use crate::cartridge::Cartridge;
use crate::cpu::CPU;
use std::sync::Arc;
use std::sync::Mutex;

use super::IRQ_VECTOR;

pub fn brk(cpu: &mut CPU, instruction: u8, rom: Cartridge, ram: &Arc<Mutex<Vec<u8>>>) -> u64 {
    let mut cycles: u64 = 2;

    match instruction {
        0x00 => {
            // Implied
            let pc = cpu.pc + 1;
            cpu.push_stack(&ram.clone(), (pc >> 8) as u8);
            cpu.push_stack(&ram.clone(), pc as u8);
            cpu.set_flags(); // Set the status register as a representation of the status struct
            cpu.push_stack(&ram.clone(), cpu.s);
            cpu.status.interrupt_disable = true;
            cpu.pc = cpu.get_mapped_word(rom, &ram.clone(), IRQ_VECTOR as usize);
            cycles = 7;
        }
        _ => {}
    }

    // Sleep for num of cycles
    return cycles;
}
