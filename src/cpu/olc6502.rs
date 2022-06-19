use std::fmt::Display;

use crate::{
    bus::Bus,
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
    bus: &'cpu mut Bus<'cpu>,
    status: u8,
    stack_ptr: u8,
    pc: u16,
    a: u8,
    x: u8,
    y: u8,
    fetched: u8,
    addr_abs: u16,
    addr_rel: u8,
    opcode: u8,
    cycles: u8,
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

    pub fn addr_mode(&mut self, addr_mode: AddrMode) -> bool {
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

    pub fn operate(&mut self, opcode: OpCode) {
        use OpCode::*;
        match opcode {
            IGL => {}
            _ => {}
        }
    }

    pub fn get_flag(&self, flag: StatusFlags) -> bool {
        self.status & flag as u8 != 0
    }

    pub fn read(&self, addr: u16) -> u8 {
        self.bus.read(addr, false)
    }

    pub fn write(&mut self, addr: u16, val: u8) {
        self.bus.write(addr, val);
    }

    pub fn fetch(&mut self) {}

    pub fn clock(&mut self) {
        if self.cycles == 0 {
            self.opcode = self.read(self.pc);
            self.pc += 1;

            let inst = Instruction::from(self.opcode);
            self.cycles = inst.cycles;
            self.addr_mode(inst.addr_mode);
            self.operate(inst.opcode);
        }
    }

    pub fn reset() {}

    pub fn irq() {}

    pub fn nmi() {}
}

pub enum AddrMode {
    IMP,
    IMM,
    ZP0,
    ZPX,
    ZPY,
    REL,
    ABS,
    ABX,
    ABY,
    IND,
    IZX,
    IZY,
}

pub trait AddressModes {
    fn imp(&mut self) -> bool;
    fn imm(&mut self) -> bool;
    fn zp0(&mut self) -> bool;
    fn zpx(&mut self) -> bool;
    fn zpy(&mut self) -> bool;
    fn rel(&mut self) -> bool;
    fn abs(&mut self) -> bool;
    fn abx(&mut self) -> bool;
    fn aby(&mut self) -> bool;
    fn ind(&mut self) -> bool;
    fn izx(&mut self) -> bool;
    fn izy(&mut self) -> bool;
}

impl<'cpu> AddressModes for OLC6502<'cpu> {
    fn imp (&mut self) -> bool {
        self.fetched = self.a;
        false
    }

    fn imm(&mut self) -> bool {
        self.pc += 1;
        self.addr_abs = self.pc;
        false
    }

    fn zp0(&mut self) -> bool {
        self.addr_abs = self.read(self.pc as u16) as u16;
        self.pc += 1;
        self.addr_abs &= 0x00FF;
        false
    }

    fn zpx(&mut self) -> bool {
        self.addr_abs = self.read(self.pc + self.x as u16) as u16;
        self.pc += 1;
        self.addr_abs &= 0x00FF;
        false
    }

    fn zpy(&mut self) -> bool {
        self.addr_abs = self.read(self.pc + self.y as u16) as u16;
        self.pc += 1;
        self.addr_abs &= 0x00FF;
        false
    }

    fn rel(&mut self) -> bool {
        self.addr_rel = self.read(self.pc as u16);
        self.pc += 1;
        if (self.addr_rel & 0x80) != 0x0 {
            self.addr_rel |= 0xFF;
        }
        false
    }

    fn abs(&mut self) -> bool {
        let low: u16 = self.read(self.pc as u16) as u16;
        self.pc += 1;
        let high: u16 = self.read(self.pc as u16) as u16;
        self.pc += 1;

        self.addr_abs = ((high) << 8) | low;

        false
    }

    fn abx(&mut self) -> bool {
        let low: u16 = self.read(self.pc as u16) as u16;
        self.pc += 1;
        let high: u16 = self.read(self.pc as u16) as u16;
        self.pc += 1;

        self.addr_abs = (((high) << 8) | low) + self.x as u16;

        if (self.addr_abs & 0xFF00) != (high << 8) {
            true
        } else {
            false
        }
    }

    fn aby(&mut self) -> bool {
        let low: u16 = self.read(self.pc as u16) as u16;
        self.pc += 1;
        let high: u16 = self.read(self.pc as u16) as u16;
        self.pc += 1;

        self.addr_abs = (((high) << 8) | low) + self.y as u16;

        if (self.addr_abs & 0xFF00) != (high << 8) {
            true
        } else {
            false
        }
    }

    fn ind(&mut self) -> bool {
        let ptr_low = self.read(self.pc as u16) as u16;
        self.pc += 1;
        let ptr_high = self.read(self.pc as u16) as u16;
        self.pc += 1;

        let ptr = (ptr_high << 8) | ptr_low;

        if ptr_low == 0x00FF { // Simulate page boundary hardware bug
            self.addr_abs = ((self.read(ptr & 0xFF00) as u16) << 8) | self.read(ptr) as u16;
        } else { // Normal operation
            self.addr_abs = ((self.read(ptr + 1) as u16) << 8) | self.read(ptr) as u16;
        }

        false
    }

    fn izx(&mut self) -> bool {
        let t: u16 = self.read(self.pc as u16) as u16;
        self.pc += 1;

        let low = self.read(t + self.x as u16) as u16 & 0x00FF;
        let high = self.read(t + self.x as u16 + 1) as u16 & 0x00FF;

        self.addr_abs = (high << 8) | low;

        false
    }

    fn izy(&mut self) -> bool {
        let t: u16 = self.read(self.pc as u16) as u16;
        self.pc += 1;

        let low = self.read(t & 0x00FF) as u16;
        let high = self.read((t + 1) & 0x00FF) as u16;

        self.addr_abs = (high << 8) | low;
        self.addr_abs += self.y as u16;

        if (self.addr_abs & 0xFF00) != (high << 8) {
            true
        } else {
            false
        }
    }
}