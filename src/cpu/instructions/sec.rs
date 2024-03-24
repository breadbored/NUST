use crate::cartridge::Cartridge;
use crate::cpu::CPU;
use crate::system::System;
use std::sync::{Arc, Mutex};

pub fn sec(cpu: &mut CPU, instruction: u8, system: &mut Arc<Mutex<System>>) -> u64 {
    let mut cycles: u64 = 2;

    match instruction {
        0x38 => {
            // Implied
            cpu.status.carry = true;
            cpu.pc += 1;
            cycles = 2;
        }
        _ => {}
    }

    // Sleep for num of cycles
    return cycles;
}
