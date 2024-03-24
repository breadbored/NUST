use crate::cartridge::Cartridge;
use crate::cpu::CPU;
use crate::system::System;
use std::sync::{Arc, Mutex};

pub fn pla(cpu: &mut CPU, instruction: u8, system: &mut Arc<Mutex<System>>) -> u64 {
    let mut cycles: u64 = 2;

    match instruction {
        0x68 => {
            // Implied
            let ram = system.lock().unwrap().ram.clone();
            cpu.a = cpu.pop_stack(&ram.clone());
            cpu.status.zero = cpu.a == 0;
            cpu.status.negative = cpu.a & 0x80 != 0;
            cpu.pc += 1;
            cycles = 4;
        }
        _ => {}
    }

    // Sleep for num of cycles
    return cycles;
}
