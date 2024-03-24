use crate::cartridge::Cartridge;
use crate::cpu::CPU;
use crate::system::System;
use std::sync::{Arc, Mutex};

pub fn php(cpu: &mut CPU, instruction: u8, system: &mut Arc<Mutex<System>>) -> u64 {
    let mut cycles: u64 = 2;

    match instruction {
        0x08 => {
            // Implied
            let ram = system.lock().unwrap().ram.clone();
            cpu.push_stack(&ram.clone(), cpu.status.get_byte());
            cpu.pc += 1;
            cycles = 3;
        }
        _ => {}
    }

    // Sleep for num of cycles
    return cycles;
}
