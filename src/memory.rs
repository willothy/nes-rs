use std::fmt::Display;

use crate::bus::Device;

#[derive(Debug)]
pub struct RAM {
    data: [u8; 0xFFFF],
}

impl Display for RAM {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        let mut res = String::new();
        for (addr, i) in self.data.iter().enumerate() {
            if *i != 0 {
                res.push_str(&format!("at {:02X}: {:02X}", addr, *i));
            }
        }
        write!(f, "{}", res)
    }
}

impl RAM {
    pub fn new() -> RAM {
        RAM { data: [0; 0xFFFF] }
    }
}

impl Device for RAM {
    fn address_space(&self) -> u16 {
        0xFFFF
    }

    fn read(&self, addr: u16, _readonly: bool) -> u8 {
        self.data[addr as usize]
    }

    fn write(&mut self, addr: u16, val: u8) {
        self.data[addr as usize] = val;
    }
}
