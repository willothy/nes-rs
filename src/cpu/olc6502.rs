use std::fmt::Display;

use crate::{
    bus::Bus, cpu::{addr_mode::{AddrMode, AddressModes}, opcode::OpCode},
};

use super::instruction::*;

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

pub struct OLC6502<'cpu> {
    pub(in super) bus: &'cpu mut Bus<'cpu>,
    pub(in super) status: u8,
    pub(in super) stack_ptr: u8,
    pub(in super) pc: u16,
    pub(in super) a: u8,
    pub(in super) x: u8,
    pub(in super) y: u8,
    pub(in super) fetched: u8,
    pub(in super) addr_abs: u16,
    pub(in super) addr_rel: u8,
    pub(in super) opcode: u8,
    pub(in super) cycles: u8,
}

impl Display for OLC6502<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", self.bus.to_string())
    }
}

impl<'cpu> OLC6502<'cpu> {
    pub fn new(bus: &'cpu mut Bus<'cpu>) -> OLC6502<'cpu> {
        OLC6502 {
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
        }
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
            IGL => {0}
            _ => {0}
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
        self.bus.read(addr, false)
    }

    pub fn write(&mut self, addr: u16, val: u8) {
        self.bus.write(addr, val);
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

    pub fn reset() {}

    pub fn irq() {}

    pub fn nmi() {}
}

