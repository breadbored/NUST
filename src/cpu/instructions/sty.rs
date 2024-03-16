use crate::cartridge::Cartridge;
use crate::cpu::CPU;
use crate::cpu::CPU_CLOCK_SPEED;
use std::sync::Arc;
use std::sync::Mutex;
use std::time::Duration;

pub fn sty(
    cpu: &mut CPU,
    instruction: u8,
    operand: u8,
    operand2: u8,
    rom: Cartridge,
    ram: &Arc<Mutex<Vec<u8>>>,
) {
    let mut cycles: u64 = 2;

    match instruction {
        0x84 => {
            // Zero Page
            cpu.set_mapped_byte(rom, &ram.clone(), operand as usize, cpu.y);
            cpu.pc += 2;
            cycles = 3;
        }
        0x94 => {
            // Zero Page, X
            cpu.set_mapped_byte(rom, &ram.clone(), operand as usize + cpu.x as usize, cpu.y);
            cpu.pc += 2;
            cycles = 4;
        }
        0x8C => {
            // Absolute
            cpu.set_mapped_byte(
                rom,
                &ram.clone(),
                (operand2 as usize) | ((operand as usize) << 8),
                cpu.y,
            );
            cpu.pc += 3;
            cycles = 4;
        }
        _ => {}
    }

    // Sleep for num of cycles
    std::thread::sleep(Duration::from_nanos(cycles * CPU_CLOCK_SPEED));
}
