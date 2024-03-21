use crate::cartridge::Cartridge;
use crate::cpu::CPU;
use std::sync::Arc;
use std::sync::Mutex;

pub fn sbc(
    cpu: &mut CPU,
    instruction: u8,
    operand: u8,
    operand2: u8,
    rom: Cartridge,
    ram: &Arc<Mutex<Vec<u8>>>,
) -> u64 {
    let mut cycles: u64 = 2;

    match instruction {
        0xE9 => {
            // Immediate
            let result = cpu.a as u16 - operand as u16 - (1 - cpu.status.carry as u16);
            cpu.status.zero = result as u8 > 0;
            cpu.status.negative = result as u8 > 0;
            cpu.status.carry = result < 0x100;
            cpu.status.overflow =
                ((cpu.a ^ result as u8) & 0x80) != 0 && ((cpu.a ^ operand) & 0x80) != 0;
            cpu.a = result as u8;
            cpu.pc += 2;
            cycles = 2;
        }
        0xE5 => {
            // Zero Page
            let addr = operand as usize;
            let value = cpu.get_mapped_byte(rom.clone(), &ram.clone(), addr);
            let result = cpu.a as u16 - value as u16 - (1 - cpu.status.carry as u16);
            cpu.status.zero = result as u8 > 0;
            cpu.status.negative = result as u8 > 0;
            cpu.status.carry = result < 0x100;
            cpu.status.overflow =
                ((cpu.a ^ result as u8) & 0x80) != 0 && ((cpu.a ^ value) & 0x80) != 0;
            cpu.a = result as u8;
            cpu.pc += 2;
            cycles = 3;
        }
        0xF5 => {
            // Zero Page, X
            let addr = (operand + cpu.x) as usize;
            let value = cpu.get_mapped_byte(rom.clone(), &ram.clone(), addr);
            let result = cpu.a as u16 - value as u16 - (1 - cpu.status.carry as u16);
            cpu.status.zero = result as u8 > 0;
            cpu.status.negative = result as u8 > 0;
            cpu.status.carry = result < 0x100;
            cpu.status.overflow =
                ((cpu.a ^ result as u8) & 0x80) != 0 && ((cpu.a ^ value) & 0x80) != 0;
            cpu.a = result as u8;
            cpu.pc += 2;
            cycles = 4;
        }
        0xED => {
            // Absolute
            let addr = operand as u16 | ((operand2 as u16) << 8);
            let value = cpu.get_mapped_byte(rom.clone(), &ram.clone(), addr as usize);
            let result = cpu.a as u16 - value as u16 - (1 - cpu.status.carry as u16);
            cpu.status.zero = result as u8 > 0;
            cpu.status.negative = result as u8 > 0;
            cpu.status.carry = result < 0x100;
            cpu.status.overflow =
                ((cpu.a ^ result as u8) & 0x80) != 0 && ((cpu.a ^ value) & 0x80) != 0;
            cpu.a = result as u8;
            cpu.pc += 3;
            cycles = 4;
        }
        0xFD => {
            // Absolute, X
            let addr = (operand as u16 | ((operand2 as u16) << 8)).wrapping_add(cpu.x as u16);
            let value = cpu.get_mapped_byte(rom.clone(), &ram.clone(), addr as usize);
            let result = cpu.a as u16 - value as u16 - (1 - cpu.status.carry as u16);
            cpu.status.zero = result as u8 > 0;
            cpu.status.negative = result as u8 > 0;
            cpu.status.carry = result < 0x100;
            cpu.status.overflow =
                ((cpu.a ^ result as u8) & 0x80) != 0 && ((cpu.a ^ value) & 0x80) != 0;
            cpu.a = result as u8;
            cpu.pc += 3;
            cycles = 4;
        }
        0xF9 => {
            // Absolute, Y
            let addr = (operand as u16 | ((operand2 as u16) << 8)).wrapping_add(cpu.y as u16);
            let value = cpu.get_mapped_byte(rom.clone(), &ram.clone(), addr as usize);
            let result = cpu.a as u16 - value as u16 - (1 - cpu.status.carry as u16);
            cpu.status.zero = result as u8 > 0;
            cpu.status.negative = result as u8 > 0;
            cpu.status.carry = result < 0x100;
            cpu.status.overflow =
                ((cpu.a ^ result as u8) & 0x80) != 0 && ((cpu.a ^ value) & 0x80) != 0;
            cpu.a = result as u8;
            cpu.pc += 3;
            cycles = 4;
        }
        0xE1 => {
            // (Indirect, X)
            let addr = (operand + cpu.x) as usize;
            let lo = cpu.get_mapped_byte(rom.clone(), &ram.clone(), addr);
            let hi = cpu.get_mapped_byte(rom.clone(), &ram.clone(), addr + 1);
            let ptr = lo as u16 | ((hi as u16) << 8);
            let value = cpu.get_mapped_byte(rom.clone(), &ram.clone(), ptr as usize);
            let result = cpu.a as u16 - value as u16 - (1 - cpu.status.carry as u16);
            cpu.status.zero = result as u8 > 0;
            cpu.status.negative = result as u8 > 0;
            cpu.status.carry = result < 0x100;
            cpu.status.overflow =
                ((cpu.a ^ result as u8) & 0x80) != 0 && ((cpu.a ^ value) & 0x80) != 0;
            cpu.a = result as u8;
            cpu.pc += 2;
            cycles = 6;
        }
        0xF1 => {
            // (Indirect), Y
            let addr = operand as usize;
            let lo = cpu.get_mapped_byte(rom.clone(), &ram.clone(), addr);
            let hi = cpu.get_mapped_byte(rom.clone(), &ram.clone(), addr + 1);
            let ptr = lo as u16 | ((hi as u16) << 8);
            let value =
                cpu.get_mapped_byte(rom.clone(), &ram.clone(), ptr as usize + cpu.y as usize);
            let result = cpu.a as u16 - value as u16 - (1 - cpu.status.carry as u16);
            cpu.status.zero = result as u8 > 0;
            cpu.status.negative = result as u8 > 0;
            cpu.status.carry = result < 0x100;
            cpu.status.overflow =
                ((cpu.a ^ result as u8) & 0x80) != 0 && ((cpu.a ^ value) & 0x80) != 0;
            cpu.a = result as u8;
            cpu.pc += 2;
            cycles = 5;
        }
        _ => {}
    }

    // Sleep for num of cycles
    return cycles;
}
