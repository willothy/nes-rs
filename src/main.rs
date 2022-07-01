use std::{cell::RefCell, rc::Rc};

use self::cpu::OLC6502;

mod bus;
mod cartridge;
mod cpu;
mod memory;
mod ppu;

fn main() {
    let bus = Rc::new(RefCell::new(bus::Bus::new()));

    let cpu_ptr = Rc::new(RefCell::new(OLC6502::new(bus)));

    let mut cpu = (*cpu_ptr).borrow_mut();

    cpu.bus.borrow_mut().connect_cpu(cpu_ptr.clone());
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_run_test() {
        let bus = Rc::new(RefCell::new(bus::Bus::new()));

        let cpu_ptr = Rc::new(RefCell::new(OLC6502::new(bus)));

        let mut cpu = (*cpu_ptr).borrow_mut();

        cpu.bus.borrow_mut().connect_cpu(cpu_ptr.clone());

        // Program start address (0x0010)
        cpu.write(0x0F00, 0x10); // low
        cpu.write(0x0F01, 0x00); // high
        cpu.reset_program(0x0F00);

        cpu.write(0x0010, 0xA9); // LDA
        cpu.write(0x0011, 0xC0); // #$c0
        cpu.write(0x0012, 0xAA); // TAX
        cpu.write(0x0013, 0xE8); // INX
        cpu.write(0x0014, 0x00); // BRK
        while cpu.is_running() {
            cpu.clock();
        }

        println!("{}", cpu);
        assert!(0 == 1);
        assert_eq!(cpu.registers.x, 0xC1);
    }
}