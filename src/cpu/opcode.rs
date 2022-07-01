use crate::cpu::StatusFlags;
use StatusFlags::*;

use super::{OLC6502, addr_mode::AddrMode, instruction::Instruction};

/* #[allow(unused)]
pub enum OpCode {
    ADC,
    AND,
    ASL,
    BCC,
    BCS,
    BEQ,
    BIT,
    BMI,
    BNE,
    BPL,
    BRK,
    BVC,
    BVS,
    CLC,
    CLD,
    CLI,
    CLV,
    CMP,
    CPX,
    CPY,
    DEC,
    DEX,
    DEY,
    EOR,
    INC,
    INX,
    INY,
    JMP,
    JSR,
    LDA,
    LDX,
    LDY,
    LSR,
    NOP,
    ORA,
    PHA,
    PHP,
    PLA,
    PLP,
    ROL,
    ROR,
    RTI,
    RTS,
    SBC,
    SEC,
    SED,
    SEI,
    STA,
    STX,
    STY,
    TAX,
    TAY,
    TSX,
    TXA,
    TXS,
    TYA,
    IGL,
} */




pub fn adc(cpu: &mut OLC6502) -> u8 {
    cpu.fetch();
    let temp = cpu.registers.a as u16 + cpu.fetched as u16 + cpu.get_flag(C) as u16;
    cpu.set_flag(C, temp > 255);
    cpu.set_flag(Z, (temp & 0x80) != 0);
    cpu.set_flag(
        V,
        (!(cpu.registers.a as u16 ^ cpu.fetched as u16) & (cpu.registers.a as u16 ^ temp as u16) & 0x0080) != 0,
    );
    cpu.set_flag(N, (temp & 0x80) != 0);
    cpu.registers.a = (temp & 0x00FF) as u8;
    1
}

pub fn and(cpu: &mut OLC6502) -> u8 {
    cpu.fetch();
    cpu.registers.a = cpu.registers.a & cpu.fetched;
    cpu.set_flag(Z, cpu.registers.a == 0x00);
    cpu.set_flag(N, (cpu.registers.a & 0x80) != 0);
    1
}

pub fn asl(cpu: &mut OLC6502) -> u8 {
    todo!()
}

pub fn bcc(cpu: &mut OLC6502) -> u8 {
    todo!()
}

pub fn bcs(cpu: &mut OLC6502) -> u8 {
    if cpu.get_flag(C) == true {
        cpu.cycles += 1;
        cpu.addr_abs = cpu.pc + cpu.addr_rel as u16;

        if (cpu.addr_abs & 0xFF00) != (cpu.pc & 0xFF00) {
            cpu.cycles += 1;
        }

        cpu.pc = cpu.addr_abs;
    }
    0
}

pub fn beq(cpu: &mut OLC6502) -> u8 {
    todo!()
}

pub fn bit(cpu: &mut OLC6502) -> u8 {
    todo!()
}

pub fn bmi(cpu: &mut OLC6502) -> u8 {
    todo!()
}

pub fn bne(cpu: &mut OLC6502) -> u8 {
    todo!()
}

pub fn bpl(cpu: &mut OLC6502) -> u8 {
    todo!()
}

pub fn brk(cpu: &mut OLC6502) -> u8 {
    cpu.set_flag(B, true);
    0
}

pub fn bvc(cpu: &mut OLC6502) -> u8 {
    todo!()
}

pub fn bvs(cpu: &mut OLC6502) -> u8 {
    todo!()
}

pub fn clc(cpu: &mut OLC6502) -> u8 {
    cpu.set_flag(C, false);
    0
}

pub fn cld(cpu: &mut OLC6502) -> u8 {
    cpu.set_flag(D, false);
    0
}

pub fn cli(cpu: &mut OLC6502) -> u8 {
    cpu.set_flag(I, false);
    0
}

pub fn clv(cpu: &mut OLC6502) -> u8 {
    cpu.set_flag(V, false);
    0
}

pub fn cmp(cpu: &mut OLC6502) -> u8 {
    todo!()
}

pub fn cpx(cpu: &mut OLC6502) -> u8 {
    todo!()
}

pub fn cpy(cpu: &mut OLC6502) -> u8 {
    todo!()
}

pub fn dec(cpu: &mut OLC6502) -> u8 {
    todo!()
}

pub fn dex(cpu: &mut OLC6502) -> u8 {
    todo!()
}

pub fn dey(cpu: &mut OLC6502) -> u8 {
    todo!()
}

pub fn eor(cpu: &mut OLC6502) -> u8 {
    todo!()
}

pub fn inc(cpu: &mut OLC6502) -> u8 {
    todo!()
}

pub fn inx(cpu: &mut OLC6502) -> u8 {
    cpu.registers.x += 1;
    0
}

pub fn iny(cpu: &mut OLC6502) -> u8 {
    todo!()
}

pub fn jmp(cpu: &mut OLC6502) -> u8 {
    todo!()
}

pub fn jsr(cpu: &mut OLC6502) -> u8 {
    todo!()
}

pub fn lda(cpu: &mut OLC6502) -> u8 {
    cpu.registers.a = cpu.read(cpu.pc);
    //cpu.pc += 1;

    cpu.set_flag(Z, cpu.registers.a == 0x00);
    cpu.set_flag(N, cpu.registers.a & 0x80 != 0);
    0
}

pub fn ldx(cpu: &mut OLC6502) -> u8 {
    todo!()
}

pub fn ldy(cpu: &mut OLC6502) -> u8 {
    todo!()
}

pub fn lsr(cpu: &mut OLC6502) -> u8 {
    todo!()
}

pub fn nop(cpu: &mut OLC6502) -> u8 {
    todo!()
}

pub fn ora(cpu: &mut OLC6502) -> u8 {
    todo!()
}

pub fn pha(cpu: &mut OLC6502) -> u8 {
    cpu.write(0x0100 + cpu.sp as u16, cpu.registers.a);
    cpu.sp -= 1;
    0
}

pub fn php(cpu: &mut OLC6502) -> u8 {
    todo!()
}

pub fn pla(cpu: &mut OLC6502) -> u8 {
    todo!()
}

pub fn plp(cpu: &mut OLC6502) -> u8 {
    todo!()
}

pub fn rol(cpu: &mut OLC6502) -> u8 {
    todo!()
}

pub fn ror(cpu: &mut OLC6502) -> u8 {
    todo!()
}

pub fn rti(cpu: &mut OLC6502) -> u8 {
    cpu.sp += 1;
    cpu.st = cpu.read(0x0100 + cpu.sp as u16);
    cpu.st &= !(B as u8);
    cpu.st &= !(U as u8);

    cpu.sp += 1;
    cpu.pc = cpu.read(0x0100 + cpu.sp as u16) as u16;
    cpu.sp += 1;
    cpu.pc |= (cpu.read(0x0100 + cpu.sp as u16) as u16) << 8;
    0
}

pub fn rts(cpu: &mut OLC6502) -> u8 {
    todo!()
}

pub fn sbc(cpu: &mut OLC6502) -> u8 {
    cpu.fetch();
    let value = (cpu.fetched as u16) ^ 0x00FF;
    let temp = cpu.registers.a as u16 + value + cpu.get_flag(C) as u16;
    cpu.set_flag(C, temp > 255);
    cpu.set_flag(Z, (temp & 0x80) != 0);
    cpu.set_flag(V, (!(temp ^ cpu.registers.a as u16) & (temp ^ value) & 0x0080) != 0);
    cpu.set_flag(N, (temp & 0x0080) != 0);
    cpu.registers.a = (temp & 0x00FF) as u8;
    1
}

pub fn sec(cpu: &mut OLC6502) -> u8 {
    todo!()
}

pub fn sed(cpu: &mut OLC6502) -> u8 {
    todo!()
}

pub fn sei(cpu: &mut OLC6502) -> u8 {
    todo!()
}

pub fn sta(cpu: &mut OLC6502) -> u8 {
    todo!()
}

pub fn stx(cpu: &mut OLC6502) -> u8 {
    todo!()
}

pub fn sty(cpu: &mut OLC6502) -> u8 {
    todo!()
}

pub fn tax(cpu: &mut OLC6502) -> u8 {
    cpu.registers.x = cpu.registers.a;
    cpu.set_flag(Z, cpu.registers.x != 0);
    cpu.set_flag(N, cpu.registers.x & 0b10000000 != 0);
    0x00
}

pub fn tay(cpu: &mut OLC6502) -> u8 {
    cpu.registers.y = cpu.registers.a;
    cpu.set_flag(Z, cpu.registers.y != 0);
    cpu.set_flag(N, cpu.registers.y & 0b10000000 != 0);
    0x00
}

pub fn tsx(cpu: &mut OLC6502) -> u8 {
    todo!()
}

pub fn txa(cpu: &mut OLC6502) -> u8 {
    todo!()
}

pub fn txs(cpu: &mut OLC6502) -> u8 {
    todo!()
}

pub fn tya(cpu: &mut OLC6502) -> u8 {
    todo!()
}

pub fn igl(cpu: &mut OLC6502) -> u8 {
    todo!()
}
