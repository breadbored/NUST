use std::fs;

pub fn get_rom() -> Cartridge {
    let contents = fs::read("test.nes");

    match contents {
        Ok(file) => {
            let is_ines = file[0..4] == [0x4E, 0x45, 0x53, 0x1A];
            if !is_ines {
                panic!("Not a valid iNES file");
            }
            let is_nes2 = file[7] & 0x0C == 0x08;

            let cart_header = CartridgeHeader {
                ines: is_ines,
                nes2: is_nes2,
                prg_rom_size: file[4],
                chr_rom_size: file[5],
                flags: (file[6] as u16) << 8 | file[7] as u16,
                mapper: (file[6] & 0xF0) | (file[7] >> 4),
                submapper: (file[8] & 0xF0) | (file[9] >> 4),
                prg_msb_rom_size: file[9] & 0x0F,
                chr_msb_rom_size: file[10] & 0x0F,
                prg_ram_size: file[10] >> 4,
                chr_ram_size: file[10] & 0x0F,
                cpu_ppu_timing: file[11] & 0x03,
                is_vs_unisystem: file[7] & 0x01 == 0x01,
                vs_unisystem: file[8] & 0x0F,
                is_extended_console: file[12] & 0x0C == 0x08,
                extended_console: file[12] & 0x0C,
                misc_roms: file[12] & 0x03,
                default_expansion_device: file[13],
            };

            let prg_rom_start = 16;
            let prg_rom_end = prg_rom_start + (cart_header.prg_rom_size as usize * 0x4000);
            let prg_rom = file[prg_rom_start..prg_rom_end].to_vec();

            // println!("DEBUG {:X?}", &prg_rom[prg_rom.len() - 16..]);

            let chr_rom_start = prg_rom_end;
            let chr_rom_end = chr_rom_start + (cart_header.chr_rom_size as usize * 0x2000);
            let chr_rom = file[chr_rom_start..chr_rom_end].to_vec();

            let cart: Cartridge = Cartridge {
                header: cart_header,
                prg_rom: prg_rom,
                chr_rom: chr_rom,
            };

            return cart;
        }
        Err(e) => {
            panic!("Error reading file: {}", e)
        }
    }
}

#[derive(Clone, Copy)]
pub struct CartridgeHeader {
    pub ines: bool,
    pub nes2: bool,
    pub prg_rom_size: u8,
    pub chr_rom_size: u8,
    pub flags: u16,
    pub mapper: u8,
    pub submapper: u8,
    pub prg_msb_rom_size: u8,
    pub chr_msb_rom_size: u8,
    pub prg_ram_size: u8,
    pub chr_ram_size: u8,
    pub cpu_ppu_timing: u8,
    pub is_vs_unisystem: bool,
    pub vs_unisystem: u8,
    pub is_extended_console: bool,
    pub extended_console: u8,
    pub misc_roms: u8,
    pub default_expansion_device: u8,
}

#[derive(Clone)]
pub struct Cartridge {
    pub header: CartridgeHeader,
    pub prg_rom: Vec<u8>,
    pub chr_rom: Vec<u8>,
}

impl Cartridge {
    pub fn get_prg_from_address(&self, address: u16) -> u8 {
        // println!("Address: {:X}", address);
        if address < 0x8000 {
            return 0;
        }

        let prg_rom_address = (address - 0x8000) % (self.header.prg_rom_size as u16 * 0x4000);
        return self.prg_rom[prg_rom_address as usize];
    }

    pub fn get_chr_from_address(&self, address: u16) -> u8 {
        if address < 0x2000 {
            return 0;
        }

        let chr_rom_address = address - 0x2000;
        return self.chr_rom[chr_rom_address as usize];
    }

    pub fn read_prg_word(&self, addr: u16) -> u16 {
        let low = self.get_prg_from_address(addr) as u16;
        let high = self.get_prg_from_address(addr + 1) as u16;
        return (high << 8) | low;
    }
}
