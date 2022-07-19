use std::{fmt::Display, path::Path};

use crate::bus::Device;

pub enum Mirroring {
    VERTICAL,
    HORIZONTAL,
    FOUR_SCREEN,
}

pub struct Rom {
    pub prg_rom: Vec<u8>,
    pub chr_rom: Vec<u8>,
    pub mapper: u8,
    pub mirroring: Mirroring,
}

impl Rom {
    pub fn new(rom: &Vec<u8>) -> Result<Rom, String> {
        if rom[0..4] != [0x4E, 0x45, 0x53, 0x1A] {
            return Err("Invalid ROM header".to_owned())
        }

        let mapper = (rom[7] & 0b1111_0000) | (rom[6] >> 4);

        let ines_ver = (rom[7] >> 2) & 0b11;
        if ines_ver != 0 {
            return Err(format!("Unsupported ines version: {}", ines_ver))
        }

        let four_screen = rom[6] & 0b1000 != 0;
        let vertical = rom[6] & 0b1 != 0;
        let mirroring = match (four_screen, vertical) {
            (true, _) => Mirroring::FOUR_SCREEN,
            (false, true) => Mirroring::VERTICAL,
            (false, false) => Mirroring::HORIZONTAL,
        };
        const PRG_PAGE_SIZE: usize = 16 * 1024;
        const CHR_PAGE_SIZE: usize = 8 * 1024;
        let prg_rom_size = (rom[4] as usize) * PRG_PAGE_SIZE;
        let chr_rom_size = (rom[5] as usize) * CHR_PAGE_SIZE;

        let skip_trainer = rom[6] & 0b100 != 0;

        const INES_HEADER_SIZE: usize = 16;
        let prg_start = INES_HEADER_SIZE + (if skip_trainer { 0 } else { 512 });
        let prg_end = prg_start + prg_rom_size;
        
        let chr_end = prg_end + chr_rom_size;

        Ok(Rom {
            prg_rom: rom[prg_start..prg_end].to_vec(),
            chr_rom: rom[prg_end..chr_end].to_vec(),
            mapper,
            mirroring,
        })
    }
}

#[derive(Debug)]
pub struct Cartridge {
    prog_memory: Vec<u8>,
    char_memory: Vec<u8>,
    prog_banks: u8,
    char_banks: u8,
    mapper_id: u8,
}

// iNES Format Header
struct Header {
    name: [u8; 4],
    prog_rom_chunks: u8,
    char_rom_chunks: u8,
    mapper1: u8,
    mapper2: u8,
    prog_ram_size: u8,
    tv_system1: u8,
    tv_system2: u8,
    unused: [u8; 5],
}

impl Cartridge {
    pub fn new(file_name: String) -> Cartridge {
        let data = match std::fs::read(Path::new(&file_name)) {
            Ok(data) => data,
            Err(_) => return Cartridge {
                prog_memory: vec![],
                char_memory: vec![],
                prog_banks: 0,
                char_banks: 0,
                mapper_id: 0,
            },
        };
        
        unsafe {
            let header: Header = std::mem::transmute::<&[u8], Header>(&data[..16]);
        }



        Cartridge {
            prog_memory: todo!(),
            char_memory: todo!(),
            prog_banks: todo!(),
            char_banks: todo!(),
            mapper_id: todo!(),
        }
    }

    pub fn ppu_read(&self, addr: u16, readonly: bool) -> u8 {
        0x00
    }

    pub fn ppu_write(&mut self, addr: u16, val: u8) {}
}

impl Display for Cartridge {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Cartridge")
    }
}

impl Device for Cartridge {
    fn in_addr_space(&self, addr: u16) -> bool {
        false
    }

    fn read(&self, addr: u16, readonly: bool) -> u8 {
        match addr {
            0x0000 => {}
            0x0001 => {}
            0x0002 => {}
            0x0003 => {}
            0x0004 => {}
            0x0005 => {}
            0x0006 => {}
            0x0007 => {}
            _ => {}
        }
        0x00
    }

    fn write(&mut self, addr: u16, val: u8) {
        match addr {
            0x0000 => {}
            0x0001 => {}
            0x0002 => {}
            0x0003 => {}
            0x0004 => {}
            0x0005 => {}
            0x0006 => {}
            0x0007 => {}
            _ => {}
        }
    }
}
