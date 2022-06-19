use std::fmt::Display;

pub trait Device: std::fmt::Display {
    fn address_space(&self) -> u16;
    fn read(&self, addr: u16, readonly: bool) -> u8;
    fn write(&mut self, addr: u16, val: u8);
}

impl std::fmt::Debug for dyn Device {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "Series{{{}}}", self.to_string())
    }
}

pub struct Bus<'bus> {
    devices: Vec<&'bus mut dyn Device>,
}

impl<'bus> Display for Bus<'bus> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            self.devices
                .iter()
                .map(|d| d.to_string())
                .collect::<Vec<_>>()
                .join(", ")
        )
    }
}

impl<'bus> Bus<'bus> {
    pub fn new() -> Bus<'bus> {
        Bus {
            devices: Vec::new(),
        }
    }

    pub fn add_device(&mut self, device: &'bus mut dyn Device) {
        self.devices.push(device);
    }

    pub fn read(&self, addr: u16, readonly: bool) -> u8 {
        for device in self.devices.iter() {
            println!("addr: {}", addr);
            let space = device.address_space();
            println!("space: {}", space);
            if addr >= space {
                continue;
            }

            return device.read(addr, readonly);
        }
        0
    }

    pub fn write(&mut self, addr: u16, val: u8) {
        for device in self.devices.iter_mut() {
            if addr >= device.address_space() {
                continue;
            }

            device.write(addr, val);
            return;
        }
    }
}
