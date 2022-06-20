use std::fmt::Display;

use crate::bus::Device;

#[derive(Debug)]
pub struct RAM {
    data: [u8; 2048],
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
        RAM { data: [0; 2048] }
    }
}

impl Device for RAM {
    fn read(&self, addr: u16, _readonly: bool) -> u8 {
        self.data[(addr & 0x07FF) as usize]
    }

    fn write(&mut self, addr: u16, val: u8) {
        self.data[(addr & 0x07FF) as usize] = val;
    }

    fn in_addr_space(&self, addr: u16) -> bool {
        addr <= 0x1FFF
    }
}
