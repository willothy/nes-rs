use std::{fmt::Display, path::Path};

use serde::{Deserialize, Serialize};

use crate::bus::Device;

#[derive(Debug)]
pub struct Cartridge {
    prog_memory: Vec<u8>,
    char_memory: Vec<u8>,
    prog_banks: u8,
    char_banks: u8,
    mapper_id: u8,
}

// iNES Format Header
#[derive(Serialize, Deserialize, Debug)]
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
        todo!()
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
