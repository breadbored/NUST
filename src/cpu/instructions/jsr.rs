use crate::cartridge::Cartridge;
use crate::cpu::CPU;
use crate::system::System;
use std::sync::Arc;
use std::sync::Mutex;

pub fn jsr(
    cpu: &mut CPU,
    instruction: u8,
    operand: u8,
    operand2: u8,
    system: &mut Arc<Mutex<System>>,
) -> u64 {
    let mut cycles: u64 = 2;

    match instruction {
        0x20 => {
            // Absolute
            let ram = system.lock().unwrap().ram.clone();
            let addr = (operand as u16) | ((operand2 as u16) << 8);
            let pc = cpu.pc - 1;
            cpu.push_stack(&ram.clone(), (pc >> 8) as u8);
            cpu.push_stack(&ram.clone(), pc as u8);
            cpu.pc = addr;
            cycles = 6;
        }
        _ => {}
    }

    // Sleep for num of cycles
    return cycles;
}
