use std::fmt::Display;

use crate::bus::Device;

#[derive(Debug)]
pub struct Cartridge {

}

impl Cartridge {
    pub fn new() -> Cartridge {
        Cartridge {
        
        }
    }


    pub fn ppu_read(&self, addr: u16, readonly: bool) -> u8 {
        0x00
    }

    pub fn ppu_write(&mut self, addr: u16, val: u8) {
    
    }
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