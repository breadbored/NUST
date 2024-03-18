mod instructions;

use std::sync::{Arc, Mutex};

use crate::cartridge::Cartridge;
use crate::cpu::instructions::beq;
use instructions::adc;
use instructions::and;
use instructions::asl;
use instructions::bcc;
use instructions::bcs;
use instructions::bne;
use instructions::dec;
use instructions::dex;
use instructions::dey;
use instructions::jmp;
use instructions::jsr;
use instructions::lda;
use instructions::ldx;
use instructions::ldy;
use instructions::nop;
use instructions::ora;
use instructions::sta;
use instructions::stx;
use instructions::sty;
use instructions::{CPU_CLOCK_SPEED, IRQ_VECTOR, NMI_VECTOR, RESET_VECTOR};

#[derive(Clone, Copy)]
pub struct CPU {
    // Registers
    a: u8,
    x: u8,
    y: u8,
    pc: u16,
    sp: u8,
    s: u8,

    // CPU state
    status: Status,
    reset: bool,
}

#[derive(Clone, Copy)]
pub struct Status {
    negative: bool,
    overflow: bool,
    reserved: bool,
    break_mode: bool,
    decimal_mode: bool,
    interrupt_disable: bool,
    zero: bool,
    carry: bool,
}

impl CPU {
    pub fn new() -> CPU {
        CPU {
            a: 0,
            x: 0,
            y: 0,
            pc: 0,
            sp: 0xFF,
            s: 0,

            status: Status {
                negative: false,
                overflow: false,
                reserved: true,
                break_mode: true,
                decimal_mode: false,
                interrupt_disable: true,
                zero: false,
                carry: false,
            },
            reset: true,
        }
    }

    pub fn tick(
        &mut self,
        rom: Cartridge,
        ram: &Arc<Mutex<Vec<u8>>>,
        vram: &Arc<Mutex<Vec<u8>>>,
    ) -> u64 {
        if self.reset {
            println!("Resetting CPU");
            // self.reset_vector = ((rom.header.prg_rom_size as u16 * 0x4000) % 0x8000) - 4 + 0x7FFF;
            println!("Reset Vector {:X}", RESET_VECTOR);
            let high = self.get_mapped_byte(rom.clone(), ram, RESET_VECTOR as usize) as u16;
            let low = self.get_mapped_byte(rom.clone(), ram, RESET_VECTOR as usize + 1) as u16;
            self.pc = (low << 8) | high;
            self.reset = false;
            return 0;
        }

        // println!("PC: {}", self.pc);
        let instruction = self.get_mapped_byte(rom.clone(), ram, self.pc as usize);
        let operand = self.get_mapped_byte(rom.clone(), ram, self.pc as usize + 1);
        let operand2 = self.get_mapped_byte(rom.clone(), ram, self.pc as usize + 2);

        // Since we're managing status flags in a struct and limited cases require reading flags directly,
        // we call this before every instruction to sync the status register with the flags.
        self.set_flags();

        match instruction {
            0x01 | 0x11 | 0x05 | 0x15 | 0x09 | 0x19 | 0x0D | 0x1D => {
                // ORA
                println!("ORA");
                return ora(self, instruction, operand, rom.clone(), ram);
            }
            0xA1 | 0xB1 | 0xA5 | 0xB5 | 0xA9 | 0xB9 | 0xAD | 0xBD => {
                // LDA
                println!("LDA");
                return lda(self, instruction, operand, operand2, rom.clone(), ram);
            }
            0xA2 | 0xA6 | 0xB6 | 0xAE | 0xBE => {
                // LDX
                println!("LDX");
                return ldx(self, instruction, operand, operand2, rom.clone(), ram);
            }
            0xA0 | 0xA4 | 0xB4 | 0xAC | 0xBC => {
                // LDY
                println!("LDY");
                return ldy(self, instruction, operand, operand2, rom.clone(), ram);
            }
            0x85 | 0x95 | 0x8D | 0x9D | 0x99 | 0x81 | 0x91 => {
                // STA
                println!("STA");
                return sta(self, instruction, operand, operand2, rom.clone(), ram);
            }
            0x86 | 0x96 | 0x8E => {
                // STX
                println!("STX");
                return stx(self, instruction, operand, operand2, rom.clone(), ram);
            }
            0x84 | 0x94 | 0x8C => {
                // STY
                println!("STY");
                return sty(self, instruction, operand, operand2, rom.clone(), ram);
            }
            0x1A | 0x3A | 0x5A | 0x7A | 0xDA | 0xFA | 0x80 | 0x82 | 0x89 | 0xC2 | 0xE2 | 0x04
            | 0x44 | 0x64 | 0x14 | 0x34 | 0x54 | 0x74 | 0xD4 | 0xF4 | 0x0C | 0x1C | 0x3C | 0x5C
            | 0x7C | 0xDC | 0xFC => {
                // NOP
                println!("NOP");
                return nop(self, instruction);
            }
            0x4C | 0x6C => {
                // JMP
                println!("JMP 0x{:X?}{:X?}", operand, operand2);
                return jmp(self, instruction, operand, operand2, rom.clone(), ram);
            }
            0xD0 => {
                // BNE
                println!("BNE");
                return bne(self, instruction, operand, rom.clone(), ram);
            }
            0x20 => {
                // JSR
                println!("JSR");
                return jsr(self, instruction, operand, operand2, rom.clone(), ram);
            }
            0x69 | 0x65 | 0x75 | 0x6D | 0x7D | 0x79 | 0x61 | 0x71 => {
                // ADC
                println!("ADC");
                return adc(self, instruction, operand, operand2, rom.clone(), ram);
            }
            0x29 | 0x25 | 0x35 | 0x2D | 0x3D | 0x39 | 0x21 | 0x31 => {
                // AND
                println!("AND");
                return and(self, instruction, operand, operand2, rom.clone(), ram);
            }
            0x0A | 0x06 | 0x16 | 0x0E | 0x1E => {
                // ASL
                println!("ASL");
                return asl(self, instruction, operand, operand2, rom.clone(), ram);
            }
            0xCA => {
                // DEX
                println!("DEX");
                return dex(self, instruction, rom.clone(), ram);
            }
            0x88 => {
                // DEY
                println!("DEY");
                return dey(self, instruction, rom.clone(), ram);
            }
            0xC6 | 0xD6 | 0xCE | 0xDE => {
                // DEC
                println!("DEC");
                return dec(self, instruction, operand, operand2, rom.clone(), ram);
            }
            0x90 => {
                // BCC
                println!("BCC");
                return bcc(self, instruction, operand, rom.clone(), ram);
            }
            0xB0 => {
                // BCS
                println!("BCS");
                return bcs(self, instruction, operand, rom.clone(), ram);
            }
            0xF0 => {
                // BEQ
                println!("BEQ");
                return beq(self, instruction, operand, rom.clone(), ram);
            }
            _ => {
                println!("Unknown instruction: 0x{:X?}", instruction);
                return nop(self, instruction);
            }
        }
    }

    pub fn get_mapped_byte(&self, rom: Cartridge, ram: &Arc<Mutex<Vec<u8>>>, address: usize) -> u8 {
        // println!("---------------------------");
        // println!("Getting byte at 0x{:X?}", address);
        if address <= 0x1FFF {
            // println!("Getting byte from RAM");
            let ram = ram.lock().unwrap();
            return ram[address & 0x7FF];
        }

        if address <= 0x3FFF {
            // println!("TODO: PPU REGISTERS");
            if address == 0x2002 {
                // println!("TODO: PPU STATUS");
                return 0;
            }
            if address == 0x2007 {
                // println!("TODO: PPU DATA");
                return 0;
            }
            return 0;
        }

        if address <= 0x401F {
            // println!("TODO: HARDWARE REGISTERS");
            if address == 0x4016 {
                // println!("TODO: JOYPAD 1");
                return 0;
            }
            if address == 0x4017 {
                // println!("TODO: JOYPAD 2");
                return 0;
            }
            if address == 0x4014 {
                // println!("TODO: OAM DMA");
                return 0;
            }
            if address == 0x4015 {
                // println!("TODO: APU STATUS");
                return 0;
            }
            return 0;
        }

        if address <= 0x5FFF {
            // println!("TODO: EXPANSION ROM");
            return 0;
        }

        if address <= 0xBFFF {
            // println!("TODO: LT ROM");
            return 0;
        }

        if address <= 0xFFFF {
            // println!("PRG ROM");
            return rom.get_prg_from_address(address as u16);
        }

        println!("WHAT THE FUCK?");

        return 0;
    }

    pub fn set_mapped_byte(
        &self,
        rom: Cartridge,
        ram: &Arc<Mutex<Vec<u8>>>,
        address: usize,
        value: u8,
    ) {
        if address <= 0x1FFF {
            let mut ram = ram.lock().unwrap();
            ram[address & 0x7FF] = value;
            return;
        }

        if address <= 0x3FFF {
            // println!("TODO: PPU REGISTERS");
            if address == 0x2000 {
                // println!("TODO: PPU CTRL");
                return;
            }
            if address == 0x2001 {
                // println!("TODO: PPU MASK");
                return;
            }
            if address == 0x2003 {
                // println!("TODO: PPU OAM ADDR");
                return;
            }
            if address == 0x2004 {
                // println!("TODO: PPU OAM DATA");
                return;
            }
            if address == 0x2005 {
                // println!("TODO: PPU SCROLL");
                return;
            }
            if address == 0x2006 {
                // println!("TODO: PPU ADDR");
                return;
            }
            if address == 0x2007 {
                // println!("TODO: PPU DATA");
                return;
            }
            if address == 0x2008 {
                // println!("TODO: PPU OAM DMA");
                return;
            }
            return;
        }

        if address <= 0x401F {
            // println!("TODO: HARDWARE REGISTERS");
            if address == 0x4014 {
                // println!("TODO: OAM DMA");
                return;
            }
            if address == 0x4015 {
                // println!("TODO: APU STATUS");
                return;
            }
            if address == 0x4016 {
                // println!("TODO: JOYPAD 1");
                return;
            }
            if address == 0x4017 {
                // println!("TODO: JOYPAD 2");
                return;
            }
            return;
        }

        if address <= 0x5FFF {
            // println!("TODO: EXPANSION ROM");
            return;
        }

        if address <= 0x7FFF {
            // println!("TODO: SRAM");
            return;
        }

        if address <= 0xFFFF {
            // println!("TODO: PRG ROM");
            return;
        }
    }

    pub fn get_mapped_word(
        &self,
        rom: Cartridge,
        ram: &Arc<Mutex<Vec<u8>>>,
        address: usize,
    ) -> u16 {
        let low = self.get_mapped_byte(rom.clone(), ram, address) as u16;
        let high = self.get_mapped_byte(rom.clone(), ram, address + 1) as u16;
        return (high << 8) | low;
    }

    pub fn set_mapped_word(
        &self,
        rom: Cartridge,
        ram: &Arc<Mutex<Vec<u8>>>,
        address: usize,
        value: u16,
    ) {
        let low = (value & 0x00FF) as u8;
        let high = (value & 0xFF00) as u8;
        self.set_mapped_byte(rom.clone(), ram, address, low);
        self.set_mapped_byte(rom.clone(), ram, address + 1, high);
    }

    pub fn push_stack(&mut self, ram: &Arc<Mutex<Vec<u8>>>, value: u8) {
        let mut ram = ram.lock().unwrap();
        ram[0x100 | self.sp as usize] = value;
        self.sp -= 1;
    }

    pub fn pop_stack(&mut self, ram: &Arc<Mutex<Vec<u8>>>) -> u8 {
        let ram = ram.lock().unwrap();
        self.sp += 1;
        return ram[0x100 | self.sp as usize];
    }

    pub fn set_flags(&mut self) {
        self.s = self.status.carry as u8
            | (self.status.zero as u8) << 1
            | (self.status.interrupt_disable as u8) << 2
            | (self.status.decimal_mode as u8) << 3
            | (self.status.break_mode as u8) << 4
            | (self.status.reserved as u8) << 5
            | (self.status.overflow as u8) << 6
            | (self.status.negative as u8) << 7;
    }
}
