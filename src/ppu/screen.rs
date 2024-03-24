use crate::cartridge::Cartridge;

#[derive(Clone, Copy)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    pub fn new(r: u8, g: u8, b: u8) -> Color {
        Color { r, g, b }
    }

    pub fn from_palette(palette: u8) -> Color {
        match palette & 0x3F {
            0x00 => Color::new(0x75, 0x75, 0x75),
            0x01 => Color::new(0x27, 0x1B, 0x8F),
            0x02 => Color::new(0x00, 0x00, 0xAB),
            0x03 => Color::new(0x47, 0x00, 0x9F),
            0x04 => Color::new(0x8F, 0x00, 0x77),
            0x05 => Color::new(0xAB, 0x00, 0x13),
            0x06 => Color::new(0xA7, 0x00, 0x00),
            0x07 => Color::new(0x7F, 0x0B, 0x00),
            0x08 => Color::new(0x43, 0x2F, 0x00),
            0x09 => Color::new(0x00, 0x47, 0x00),
            0x0A => Color::new(0x00, 0x51, 0x00),
            0x0B => Color::new(0x00, 0x3F, 0x17),
            0x0C => Color::new(0x1B, 0x3F, 0x5F),
            0x0D => Color::new(0x00, 0x00, 0x00),
            0x0E => Color::new(0x00, 0x00, 0x00),
            0x0F => Color::new(0x00, 0x00, 0x00),
            0x10 => Color::new(0xBC, 0xBC, 0xBC),
            0x11 => Color::new(0x00, 0x73, 0xEF),
            0x12 => Color::new(0x23, 0x3B, 0xEF),
            0x13 => Color::new(0x83, 0x00, 0xF3),
            0x14 => Color::new(0xBF, 0x00, 0xBF),
            0x15 => Color::new(0xE7, 0x00, 0x5B),
            0x16 => Color::new(0xDB, 0x2B, 0x00),
            0x17 => Color::new(0xCB, 0x4F, 0x0F),
            0x18 => Color::new(0x8B, 0x73, 0x00),
            0x19 => Color::new(0x00, 0x97, 0x00),
            0x1A => Color::new(0x00, 0xAB, 0x00),
            0x1B => Color::new(0x00, 0x93, 0x3B),
            0x1C => Color::new(0x00, 0x83, 0x8B),
            0x1D => Color::new(0x00, 0x00, 0x00),
            0x1E => Color::new(0x00, 0x00, 0x00),
            0x1F => Color::new(0x00, 0x00, 0x00),
            0x20 => Color::new(0xFF, 0xFF, 0xFF),
            0x21 => Color::new(0x3F, 0xBF, 0xFF),
            0x22 => Color::new(0x5F, 0x97, 0xFF),
            0x23 => Color::new(0xA7, 0x8B, 0xFD),
            0x24 => Color::new(0xF7, 0x7B, 0xFF),
            0x25 => Color::new(0xFF, 0x77, 0xB7),
            0x26 => Color::new(0xFF, 0x77, 0x63),
            0x27 => Color::new(0xFF, 0x9B, 0x3B),
            0x28 => Color::new(0xF3, 0xBF, 0x3F),
            0x29 => Color::new(0x83, 0xD3, 0x13),
            0x2A => Color::new(0x4F, 0xDF, 0x4B),
            0x2B => Color::new(0x58, 0xF8, 0x98),
            0x2C => Color::new(0x00, 0xEB, 0xDB),
            0x2D => Color::new(0x00, 0x00, 0x00),
            0x2E => Color::new(0x00, 0x00, 0x00),
            0x2F => Color::new(0x00, 0x00, 0x00),
            0x30 => Color::new(0xFF, 0xFF, 0xFF),
            0x31 => Color::new(0xAB, 0xE7, 0xFF),
            0x32 => Color::new(0xC7, 0xD7, 0xFF),
            0x33 => Color::new(0xD7, 0xCB, 0xFF),
            0x34 => Color::new(0xFF, 0xC7, 0xFF),
            0x35 => Color::new(0xFF, 0xC7, 0xDB),
            0x36 => Color::new(0xFF, 0xBF, 0xB3),
            0x37 => Color::new(0xFF, 0xDB, 0xAB),
            0x38 => Color::new(0xFF, 0xE7, 0xA3),
            0x39 => Color::new(0xE3, 0xFF, 0xA3),
            0x3A => Color::new(0xAB, 0xF3, 0xBF),
            0x3B => Color::new(0xB3, 0xFF, 0xCF),
            0x3C => Color::new(0x9F, 0xFF, 0xF3),
            0x3D => Color::new(0x00, 0x00, 0x00),
            0x3E => Color::new(0x00, 0x00, 0x00),
            0x3F => Color::new(0x00, 0x00, 0x00),
            _ => Color::new(0x69, 0x69, 0x69), // 0x69, 0x69, 0x69 is transparent
        }
    }
}

pub struct Screen {
    pub width: usize,
    pub height: usize,
    pub pixels: [[Color; 256]; 256],
}

impl Screen {
    pub fn new() -> Screen {
        Screen {
            width: 256,
            height: 240,
            pixels: [[Color::new(0, 0, 0); 256]; 256],
        }
    }

    pub fn get_pixel(&self, x: usize, y: usize) -> Color {
        self.pixels[x][y]
    }

    pub fn draw_sprite(&mut self, x: usize, y: usize, sprite: [[Color; 8]; 8]) {
        for i in 0..8 {
            for j in 0..8 {
                self.pixels[x + i][y + j] = sprite[i][j];
            }
        }
    }

    pub fn get_sprite(&self, chr: Vec<u8>, x: usize, y: usize) -> [[Color; 8]; 8] {
        let mut sprite = [[Color::new(0, 0, 0); 8]; 8];
        for col in 0..8 {
            for row in 0..8 {
                let tile_offset = y * 32 * 16 + x * 16; // y * # of tiles per row * bytes per tile + x * bytes per tile
                let byte1 = chr[tile_offset + row];
                let byte2 = chr[tile_offset + row + 8];
                let bit1 = (byte1 >> (7 - col)) & 1;
                let bit2 = (byte2 >> (7 - col)) & 1;
                let color = (bit2 << 1) | bit1;
                sprite[col][row] = Color::from_palette(color); // Corrected the indexing here
            }
        }
        sprite
    }

    pub fn draw_entire_sprite_map(&mut self, rom: Cartridge, x: usize, y: usize) {
        let chr = rom.chr_rom;
        let tiles_per_row = 32; // Assuming a 256x256 pixel sprite map, which means 32 tiles per row (256 / 8)
        let tiles_per_column = chr.len() / 32 / tiles_per_row; // Calculate the number of tiles based on CHR ROM size
        for i in 0..tiles_per_row {
            for j in 0..tiles_per_column {
                let sprite = self.get_sprite(chr.clone(), i, j);
                self.draw_sprite(x + i * 8, y + j * 8, sprite);
            }
        }
    }

    pub fn clear(&mut self) {
        for i in 0..self.width {
            for j in 0..self.height {
                self.pixels[i][j] = Color::new(0, 0, 0);
            }
        }
    }
}
