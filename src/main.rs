use std::{rc::Rc, cell::RefCell};

use self::cpu::OLC6502;

mod bus;
mod cpu;
mod memory;
mod ppu;
mod cartridge;

fn main() {
    let bus = Rc::new(RefCell::new(bus::Bus::new()));

    let cpu_ptr = Rc::new(RefCell::new(OLC6502::new(bus)));

    let mut cpu = (*cpu_ptr).borrow_mut();

    cpu.bus.borrow_mut().connect_cpu(cpu_ptr.clone());

    cpu.write(0x00FF, 15);
    println!("{}", cpu);
    println!("{:?}", cpu.read(0x00FF));
}
