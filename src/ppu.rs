use std::{cell::RefCell, fmt::Display, ptr::null_mut, rc::Rc};

use crate::{bus::Device, cartridge::Cartridge};

#[derive(Debug)]
pub struct OLC2C02 {
    cartridge: Rc<RefCell<Cartridge>>,
    name_table: [[u8; 1024]; 2],
    palette_table: [u8; 32],
}

impl OLC2C02 {
    pub fn new(cartridge: Rc<RefCell<Cartridge>>) -> OLC2C02 {
        OLC2C02 {
            cartridge,
            name_table: [[0; 1024]; 2],
            palette_table: [0; 32],
        }
    }

    pub fn clock(&mut self) {}

    // PPU bus
    pub fn ppu_read(&self, addr: u16, readonly: bool) -> u8 {
        let addr = addr & 0x3FFF;
        0x00
    }

    pub fn ppu_write(&mut self, addr: u16, val: u8) {
        let addr = addr & 0x3FFF;
    }
}

impl Display for OLC2C02 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "PPU: OLC2C02")
    }
}

impl Device for OLC2C02 {
    fn in_addr_space(&self, addr: u16) -> bool {
        addr >= 0x2000 && addr <= 0x3FFF
    }

    fn read(&self, addr: u16, readonly: bool) -> u8 {
        0x00
    }

    fn write(&mut self, addr: u16, val: u8) {
        let addr = addr & 0x0007;
    }
}
