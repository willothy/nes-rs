use std::fmt::Display;

use super::OLC6502;

#[derive(Debug, PartialEq)]
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

impl Display for AddrMode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(f, "{:?}", self)
    }
}

pub fn imp(cpu: &mut OLC6502) -> u8 {
    cpu.fetched = cpu.registers.a;
    0
}

pub fn imm(cpu: &mut OLC6502) -> u8 {
    cpu.pc += 1;
    cpu.addr_abs = cpu.pc;
    0
}

pub fn zp0(cpu: &mut OLC6502) -> u8 {
    cpu.addr_abs = cpu.read(cpu.pc as u16) as u16;
    cpu.pc += 1;
    cpu.addr_abs &= 0x00FF;
    0
}

pub fn zpx(cpu: &mut OLC6502) -> u8 {
    cpu.addr_abs = cpu.read(cpu.pc + cpu.registers.x as u16) as u16;
    cpu.pc += 1;
    cpu.addr_abs &= 0x00FF;
    0
}

pub fn zpy(cpu: &mut OLC6502) -> u8 {
    cpu.addr_abs = cpu.read(cpu.pc + cpu.registers.y as u16) as u16;
    cpu.pc += 1;
    cpu.addr_abs &= 0x00FF;
    0
}

pub fn rel(cpu: &mut OLC6502) -> u8 {
    cpu.addr_rel = cpu.read(cpu.pc as u16);
    cpu.pc += 1;
    if (cpu.addr_rel & 0x80) != 0x0 {
        cpu.addr_rel |= 0xFF;
    }
    0
}

pub fn abs(cpu: &mut OLC6502) -> u8 {
    let low: u16 = cpu.read(cpu.pc as u16) as u16;
    cpu.pc += 1;
    let high: u16 = cpu.read(cpu.pc as u16) as u16;
    cpu.pc += 1;

    cpu.addr_abs = ((high) << 8) | low;

    0
}

pub fn abx(cpu: &mut OLC6502) -> u8 {
    let low: u16 = cpu.read(cpu.pc as u16) as u16;
    cpu.pc += 1;
    let high: u16 = cpu.read(cpu.pc as u16) as u16;
    cpu.pc += 1;

    cpu.addr_abs = (((high) << 8) | low) + cpu.registers.x as u16;

    if (cpu.addr_abs & 0xFF00) != (high << 8) {
        1
    } else {
        0
    }
}

pub fn aby(cpu: &mut OLC6502) -> u8 {
    let low: u16 = cpu.read(cpu.pc as u16) as u16;
    cpu.pc += 1;
    let high: u16 = cpu.read(cpu.pc as u16) as u16;
    cpu.pc += 1;

    cpu.addr_abs = (((high) << 8) | low) + cpu.registers.y as u16;

    if (cpu.addr_abs & 0xFF00) != (high << 8) {
        1
    } else {
        0
    }
}

pub fn ind(cpu: &mut OLC6502) -> u8 {
    let ptr_low = cpu.read(cpu.pc as u16) as u16;
    cpu.pc += 1;
    let ptr_high = cpu.read(cpu.pc as u16) as u16;
    cpu.pc += 1;

    let ptr = (ptr_high << 8) | ptr_low;

    if ptr_low == 0x00FF {
        // Simulate page boundary hardware bug
        cpu.addr_abs = ((cpu.read(ptr & 0xFF00) as u16) << 8) | cpu.read(ptr) as u16;
    } else {
        // Normal operation
        cpu.addr_abs = ((cpu.read(ptr + 1) as u16) << 8) | cpu.read(ptr) as u16;
    }

    0
}

pub fn izx(cpu: &mut OLC6502) -> u8 {
    let t: u16 = cpu.read(cpu.pc as u16) as u16;
    cpu.pc += 1;

    let low = cpu.read(t + cpu.registers.x as u16) as u16 & 0x00FF;
    let high = cpu.read(t + cpu.registers.x as u16 + 1) as u16 & 0x00FF;

    cpu.addr_abs = (high << 8) | low;

    0
}

pub fn izy(cpu: &mut OLC6502) -> u8 {
    let t: u16 = cpu.read(cpu.pc as u16) as u16;
    cpu.pc += 1;

    let low = cpu.read(t & 0x00FF) as u16;
    let high = cpu.read((t + 1) & 0x00FF) as u16;

    cpu.addr_abs = (high << 8) | low;
    cpu.addr_abs += cpu.registers.y as u16;

    if (cpu.addr_abs & 0xFF00) != (high << 8) {
        1
    } else {
        0
    }
}