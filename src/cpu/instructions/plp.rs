use crate::cartridge::Cartridge;
use crate::cpu::CPU;
use crate::system::System;
use std::sync::{Arc, Mutex};

pub fn plp(cpu: &mut CPU, instruction: u8, system: &mut Arc<Mutex<System>>) -> u64 {
    let mut cycles: u64 = 2;

    match instruction {
        0x28 => {
            // Implied
            let ram = system.lock().unwrap().ram.clone();
            let stack_result = cpu.pop_stack(&ram.clone());
            cpu.status.set_byte(stack_result);
            cpu.pc += 1;
            cycles = 4;
        }
        _ => {}
    }

    // Sleep for num of cycles
    return cycles;
}
