use crate::cartridge::Cartridge;
use crate::cpu::CPU;
use crate::system::System;
use std::sync::{Arc, Mutex};

pub fn rti(cpu: &mut CPU, instruction: u8, system: &mut Arc<Mutex<System>>) -> u64 {
    let mut cycles: u64 = 2;

    match instruction {
        0x40 => {
            // Implied
            let stack_result = cpu.pop_stack(&system.lock().unwrap().ram.clone());
            cpu.status.set_byte(stack_result);
            cpu.pc = cpu.pop_stack_word(&system.lock().unwrap().ram.clone());
            cycles = 6;
        }
        _ => {}
    }

    // Sleep for num of cycles
    return cycles;
}
