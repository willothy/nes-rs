use std::{
    borrow::BorrowMut,
    cell::RefCell,
    fmt::Display,
    ptr::null_mut,
    rc::{Rc, Weak},
};

use crate::{cartridge::Cartridge, cpu::OLC6502, memory::RAM, ppu::OLC2C02};

pub trait Device: std::fmt::Display {
    fn in_addr_space(&self, addr: u16) -> bool;
    fn read(&self, addr: u16, readonly: bool) -> u8;
    fn write(&mut self, addr: u16, val: u8);
}

impl std::fmt::Debug for dyn Device {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "Series{{{}}}", self.to_string())
    }
}

pub struct Devices {
    pub ppu: Rc<RefCell<OLC2C02>>,
    pub memory: Rc<RefCell<RAM>>,
    pub cartridge: Rc<RefCell<Cartridge>>,
}

impl Devices {
    pub fn iter(&self) -> DeviceIter {
        DeviceIter {
            devices: vec![
                self.cartridge.clone(),
                self.ppu.clone(),
                self.memory.clone(),
            ],
        }
    }
}

pub struct DeviceIter {
    devices: Vec<Rc<RefCell<dyn Device>>>,
}

impl Iterator for DeviceIter {
    type Item = Rc<RefCell<dyn Device>>;

    fn next(&mut self) -> Option<Self::Item> {
        self.devices.pop()
    }
}

pub struct Bus {
    pub devices: Devices,
    pub clock_counter: usize,
    pub cpu: Weak<RefCell<OLC6502>>,
}

impl Display for Bus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            self.devices
                .iter()
                .map(|d| format!("{}", d.borrow().to_string()))
                .collect::<Vec<_>>()
                .join("\n")
        )
    }
}

impl Bus {
    pub fn new() -> Bus {
        let cartridge = Rc::new(RefCell::new(Cartridge::new("".to_string())));
        let ppu = Rc::new(RefCell::new(OLC2C02::new(cartridge.clone())));
        let bus = Bus {
            devices: Devices {
                ppu,
                memory: Rc::new(RefCell::new(RAM::new())),
                cartridge,
            },
            clock_counter: 0,
            cpu: Weak::new(),
        };
        bus
    }

    pub fn connect_cpu(&mut self, cpu: Rc<RefCell<OLC6502>>) {
        self.cpu = Rc::downgrade(&cpu);
    }

    pub fn cpu_read(&self, addr: u16, readonly: bool) -> u8 {
        for device in self.devices.iter() {
            if device.borrow().in_addr_space(addr) {
                return device.borrow().read(addr, readonly);
            }
        }
        0
    }

    pub fn cpu_write(&mut self, addr: u16, val: u8) {
        for device in self.devices.iter() {
            if device.borrow().in_addr_space(addr) {
                (*device).borrow_mut().write(addr, val);
                return;
            }
        }
    }

    pub fn reset(&mut self) {
        match self.cpu.upgrade() {
            Some(cpu) => (*cpu).borrow_mut().reset(),
            None => {}
        }
        self.clock_counter = 0;
    }

    pub fn clock(&mut self) {}
}
