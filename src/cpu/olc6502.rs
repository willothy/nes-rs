use std::{cell::RefCell, fmt::Display, rc::Rc};

use crate::{
    bus::Bus,
    cpu::{
        addr_mode::{AddrMode, AddressModes},
        opcode::OpCode,
    },
    ppu::OLC2C02,
};

use super::instruction::*;
use StatusFlags::*;

pub enum StatusFlags {
    C = (1 << 0), // Carry bit
    Z = (1 << 1), // Zero bit
    I = (1 << 2), // Interrupt disable
    D = (1 << 3), // Decimal mode
    B = (1 << 4), // Break command
    U = (1 << 5), // Unused
    V = (1 << 6), // Overflow
    N = (1 << 7), // Negative
}

pub struct OLC6502 {
    pub(crate) bus: Rc<RefCell<Bus>>,
    pub(super) status: u8,
    pub(super) stack_ptr: u8,
    pub(super) pc: u16,
    pub(super) a: u8,
    pub(super) x: u8,
    pub(super) y: u8,
    pub(super) fetched: u8,
    pub(super) addr_abs: u16,
    pub(super) addr_rel: u8,
    pub(super) opcode: u8,
    pub(super) cycles: u8,
}

impl Display for OLC6502 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(f, "{}", self.bus.borrow().to_string())
    }
}

impl OLC6502 {
    pub fn new(bus: Rc<RefCell<Bus>>) -> OLC6502 {
        let mut cpu = OLC6502 {
            bus,
            status: 0x00,
            stack_ptr: 0x00,
            pc: 0x0000,
            a: 0x00,
            x: 0x00,
            y: 0x00,
            fetched: 0x00,
            addr_abs: 0x0000,
            addr_rel: 0x00,
            opcode: 0x00,
            cycles: 0x00,
        };
        cpu
    }

    pub fn addr_mode(&mut self, addr_mode: AddrMode) -> u8 {
        use AddrMode::*;
        match addr_mode {
            IMP => self.imp(),
            IMM => self.imm(),
            ZP0 => self.zp0(),
            ZPX => self.zpx(),
            ZPY => self.zpy(),
            REL => self.rel(),
            ABS => self.abs(),
            ABX => self.abx(),
            ABY => self.aby(),
            IND => self.ind(),
            IZX => self.izx(),
            IZY => self.izy(),
        }
    }

    pub fn operate(&mut self, opcode: OpCode) -> u8 {
        use OpCode::*;
        match opcode {
            IGL => 0,
            _ => 0,
        }
    }

    pub fn get_flag(&self, flag: StatusFlags) -> bool {
        self.status & flag as u8 != 0
    }

    pub fn set_flag(&mut self, flag: StatusFlags, value: bool) {
        if value {
            self.status |= flag as u8;
        } else {
            self.status &= !(flag as u8);
        }
    }

    pub fn read(&self, addr: u16) -> u8 {
        self.bus.borrow().cpu_read(addr, false)
    }

    pub fn write(&mut self, addr: u16, val: u8) {
        self.bus.borrow_mut().cpu_write(addr, val);
    }

    pub fn fetch(&mut self) -> u8 {
        if !(Instruction::from(self.opcode).addr_mode == AddrMode::IMP) {
            self.fetched = self.read(self.addr_abs);
        }
        self.fetched
    }

    pub fn clock(&mut self) {
        if self.cycles == 0 {
            self.opcode = self.read(self.pc);
            self.pc += 1;

            let inst = Instruction::from(self.opcode);
            self.cycles = inst.cycles;
            let mut additional_cycles = 0;
            additional_cycles += self.addr_mode(inst.addr_mode);
            additional_cycles += self.operate(inst.opcode);

            self.cycles += additional_cycles;
        }
        self.cycles -= 1;
    }

    pub fn reset(&mut self) {
        self.a = 0x00;
        self.x = 0x00;
        self.y = 0x00;
        self.stack_ptr = 0xFD;
        self.status = 0x00 | U as u8;

        self.addr_abs = 0xFFFC;
        let low = self.read(self.addr_abs);
        let high = self.read(self.addr_abs + 1);
        self.pc = (high as u16) << 8 | low as u16;

        self.addr_abs = 0x0000;
        self.addr_rel = 0x0000;
        self.fetched = 0x00;

        self.cycles = 8;
    }

    pub fn irq(&mut self) {
        if !self.get_flag(I) {
            self.write(
                0x0100 + self.stack_ptr as u16,
                ((self.pc >> 8) & 0x00FF) as u8,
            );
            self.stack_ptr -= 1;
            self.write(0x0100 + self.stack_ptr as u16, (self.pc & 0x00FF) as u8);
            self.stack_ptr -= 1;

            self.set_flag(B, false);
            self.set_flag(U, true);
            self.set_flag(I, true);
            self.write(0x0100 + self.stack_ptr as u16, self.status);
            self.stack_ptr -= 1;

            self.addr_abs = 0xFFFE;
            let low = self.read(self.addr_abs) as u16;
            let high = self.read(self.addr_abs + 1) as u16;
            self.pc = (high << 8) | low;

            self.cycles = 7;
        }
    }

    pub fn nmi(&mut self) {
        self.write(
            0x0100 + self.stack_ptr as u16,
            ((self.pc >> 8) & 0x00FF) as u8,
        );
        self.stack_ptr -= 1;
        self.write(0x0100 + self.stack_ptr as u16, (self.pc & 0x00FF) as u8);
        self.stack_ptr -= 1;

        self.set_flag(B, false);
        self.set_flag(U, true);
        self.set_flag(I, true);
        self.write(0x0100 + self.stack_ptr as u16, self.status);
        self.stack_ptr -= 1;

        self.addr_abs = 0xFFFA;
        let low = self.read(self.addr_abs) as u16;
        let high = self.read(self.addr_abs + 1) as u16;
        self.pc = (high << 8) | low;

        self.cycles = 8;
    }
}
