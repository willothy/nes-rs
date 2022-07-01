use std::{cell::RefCell, fmt::Display, rc::Rc};

use crate::{
    bus::Bus,
    cpu::{
        addr_mode::{AddrMode, self},
        //opcode::OpCode,
    },
    ppu::OLC2C02,
};

use super::{instruction::*, opcode};
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

#[derive(Debug)]
pub struct Registers {
    pub a: u8,
    pub x: u8,
    pub y: u8
}

impl Display for Registers {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{0}A: {1:02X} ({1}) {0}X: {2:02X} ({2}) {0}Y: {3:02X} ({3})", "\n\t", self.a, self.x, self.y)
    }
}

pub struct OLC6502 {
    pub(crate) bus: Rc<RefCell<Bus>>,
    pub(super) st: u8,
    pub(super) sp: u8,
    pub(super) pc: u16,
    pub(crate) registers: Registers,
    pub(super) fetched: u8,
    pub(super) addr_abs: u16,
    pub(super) addr_rel: u8,
    pub(super) opcode: u8,
    pub(super) cycles: u8,
}

impl Display for OLC6502 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        writeln!(f, "Registers: [{}\n]", self.registers)?;
        writeln!(f, "Status: {:02X}", self.st)?;
        writeln!(f, "Stack pointer: {:02X}", self.sp)?;
        writeln!(f, "Program counter: {:04X}", self.pc)?;
        /* writeln!(f, "Fetched: {:02X}", self.fetched)?;
        writeln!(f, "Address absolute: {:04X}", self.addr_abs)?;
        writeln!(f, "Address relative: {:02X}", self.addr_rel)?;
        writeln!(f, "Opcode: {:02X}", self.opcode)?;
        writeln!(f, "Cycles: {:?}", self.cycles)?; */
        write!(f, "{}", self.bus.borrow().to_string())
    }
}

impl OLC6502 {
    pub fn new(bus: Rc<RefCell<Bus>>) -> OLC6502 {
        let mut cpu = OLC6502 {
            bus,
            st: 0x00,
            sp: 0x00,
            pc: 0x0000,
            registers: Registers {
                a: 0x00,
                x: 0x00,
                y: 0x00,
            },
            fetched: 0x00,
            addr_abs: 0x0000,
            addr_rel: 0x00,
            opcode: 0x00,
            cycles: 0x00,
        };
        cpu.reset();
        cpu
    }

    pub fn addr_mode(&mut self, addr_mode: AddrMode) -> u8 {
        match addr_mode {
            AddrMode::IMP => addr_mode::imp(self),
            AddrMode::IMM => addr_mode::imm(self),
            AddrMode::ZP0 => addr_mode::zp0(self),
            AddrMode::ZPX => addr_mode::zpx(self),
            AddrMode::ZPY => addr_mode::zpy(self),
            AddrMode::REL => addr_mode::rel(self),
            AddrMode::ABS => addr_mode::abs(self),
            AddrMode::ABX => addr_mode::abx(self),
            AddrMode::ABY => addr_mode::aby(self),
            AddrMode::IND => addr_mode::ind(self),
            AddrMode::IZX => addr_mode::izx(self),
            AddrMode::IZY => addr_mode::izy(self),
        }
    }

    pub fn operate(&mut self, opcode: fn(&mut Self) -> u8) -> u8 {
        opcode(self)
    }

    pub fn get_flag(&self, flag: StatusFlags) -> bool {
        self.st & flag as u8 != 0
    }

    pub fn set_flag(&mut self, flag: StatusFlags, value: bool) {
        if value {
            self.st |= flag as u8;
        } else {
            self.st &= !(flag as u8);
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

            let inst = Instruction::from(self.opcode);
            println!("{:?}", inst);

            self.cycles = inst.cycles;
            let mut additional_cycles = 0;
            additional_cycles += self.addr_mode(inst.addr_mode);
            additional_cycles += self.operate(inst.opcode);

            self.pc += 1/* inst.len as u16 */;

            self.cycles += additional_cycles;
        }
        self.cycles -= 1;
    }

    pub fn is_running(&mut self) -> bool {
        self.st & B as u8 != B as u8
    }

    pub fn reset_program(&mut self, prog_addr: u16) {
        self.addr_abs = prog_addr;
        let low = self.read(self.addr_abs);
        let high = self.read(self.addr_abs + 1);
        self.pc = (high as u16) << 8 | low as u16;
    }

    pub fn reset(&mut self) {
        self.registers.a = 0x00;
        self.registers.x = 0x00;
        self.registers.y = 0x00;
        self.sp = 0xFD;
        self.st = 0x00 | U as u8;

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
                0x0100 + self.sp as u16,
                ((self.pc >> 8) & 0x00FF) as u8,
            );
            self.sp -= 1;
            self.write(0x0100 + self.sp as u16, (self.pc & 0x00FF) as u8);
            self.sp -= 1;

            self.set_flag(B, false);
            self.set_flag(U, true);
            self.set_flag(I, true);
            self.write(0x0100 + self.sp as u16, self.st);
            self.sp -= 1;

            self.addr_abs = 0xFFFE;
            let low = self.read(self.addr_abs) as u16;
            let high = self.read(self.addr_abs + 1) as u16;
            self.pc = (high << 8) | low;

            self.cycles = 7;
        }
    }

    pub fn nmi(&mut self) {
        self.write(
            0x0100 + self.sp as u16,
            ((self.pc >> 8) & 0x00FF) as u8,
        );
        self.sp -= 1;
        self.write(0x0100 + self.sp as u16, (self.pc & 0x00FF) as u8);
        self.sp -= 1;

        self.set_flag(B, false);
        self.set_flag(U, true);
        self.set_flag(I, true);
        self.write(0x0100 + self.sp as u16, self.st);
        self.sp -= 1;

        self.addr_abs = 0xFFFA;
        let low = self.read(self.addr_abs) as u16;
        let high = self.read(self.addr_abs + 1) as u16;
        self.pc = (high << 8) | low;

        self.cycles = 8;
    }
}
