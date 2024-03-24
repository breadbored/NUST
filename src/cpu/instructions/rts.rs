use crate::cartridge::Cartridge;
use crate::cpu::CPU;
use crate::system::System;
use std::sync::{Arc, Mutex};

pub fn rts(cpu: &mut CPU, instruction: u8, system: &mut Arc<Mutex<System>>) -> u64 {
    let mut cycles: u64 = 2;

    match instruction {
        0x60 => {
            // Implied
            let low = cpu.pop_stack(&system.lock().unwrap().ram.clone());
            let high = cpu.pop_stack(&system.lock().unwrap().ram.clone());
            cpu.pc = ((high as u16) << 8) | low as u16;
            cpu.pc += 1;
            cycles = 6;
        }
        _ => {}
    }

    // Sleep for num of cycles
    return cycles;
}
