use crate::cpu::StatusFlags;
use StatusFlags::*;

use super::OLC6502;

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
}

pub trait Operations {
    fn adc(&mut self) -> u8;
    fn and(&mut self) -> u8;
    fn asl(&mut self) -> u8;
    fn bcc(&mut self) -> u8;
    fn bcs(&mut self) -> u8;
    fn beq(&mut self) -> u8;
    fn bit(&mut self) -> u8;
    fn bmi(&mut self) -> u8;
    fn bne(&mut self) -> u8;
    fn bpl(&mut self) -> u8;
    fn brk(&mut self) -> u8;
    fn bvc(&mut self) -> u8;
    fn bvs(&mut self) -> u8;
    fn clc(&mut self) -> u8;
    fn cld(&mut self) -> u8;
    fn cli(&mut self) -> u8;
    fn clv(&mut self) -> u8;
    fn cmp(&mut self) -> u8;
    fn cpx(&mut self) -> u8;
    fn cpy(&mut self) -> u8;
    fn dec(&mut self) -> u8;
    fn dex(&mut self) -> u8;
    fn dey(&mut self) -> u8;
    fn eor(&mut self) -> u8;
    fn inc(&mut self) -> u8;
    fn inx(&mut self) -> u8;
    fn iny(&mut self) -> u8;
    fn jmp(&mut self) -> u8;
    fn jsr(&mut self) -> u8;
    fn lda(&mut self) -> u8;
    fn ldx(&mut self) -> u8;
    fn ldy(&mut self) -> u8;
    fn lsr(&mut self) -> u8;
    fn nop(&mut self) -> u8;
    fn ora(&mut self) -> u8;
    fn pha(&mut self) -> u8;
    fn php(&mut self) -> u8;
    fn pla(&mut self) -> u8;
    fn plp(&mut self) -> u8;
    fn rol(&mut self) -> u8;
    fn ror(&mut self) -> u8;
    fn rti(&mut self) -> u8;
    fn rts(&mut self) -> u8;
    fn sbc(&mut self) -> u8;
    fn sec(&mut self) -> u8;
    fn sed(&mut self) -> u8;
    fn sei(&mut self) -> u8;
    fn sta(&mut self) -> u8;
    fn stx(&mut self) -> u8;
    fn sty(&mut self) -> u8;
    fn tax(&mut self) -> u8;
    fn tay(&mut self) -> u8;
    fn tsx(&mut self) -> u8;
    fn txa(&mut self) -> u8;
    fn txs(&mut self) -> u8;
    fn tya(&mut self) -> u8;
    fn igl(&mut self) -> u8;
}

impl Operations for OLC6502 {
    fn adc(&mut self) -> u8 {
        self.fetch();
        let temp = self.a as u16 + self.fetched as u16 + self.get_flag(C) as u16;
        self.set_flag(C, temp > 255);
        self.set_flag(Z, (temp & 0x80) != 0);
        self.set_flag(
            V,
            (!(self.a as u16 ^ self.fetched as u16) & (self.a as u16 ^ temp as u16) & 0x0080) != 0,
        );
        self.set_flag(N, (temp & 0x80) != 0);
        self.a = (temp & 0x00FF) as u8;
        1
    }

    fn and(&mut self) -> u8 {
        self.fetch();
        self.a = self.a & self.fetched;
        self.set_flag(Z, self.a == 0x00);
        self.set_flag(N, (self.a & 0x80) != 0);
        1
    }

    fn asl(&mut self) -> u8 {
        todo!()
    }

    fn bcc(&mut self) -> u8 {
        todo!()
    }

    fn bcs(&mut self) -> u8 {
        if self.get_flag(C) == true {
            self.cycles += 1;
            self.addr_abs = self.pc + self.addr_rel as u16;

            if (self.addr_abs & 0xFF00) != (self.pc & 0xFF00) {
                self.cycles += 1;
            }

            self.pc = self.addr_abs;
        }
        0
    }

    fn beq(&mut self) -> u8 {
        todo!()
    }

    fn bit(&mut self) -> u8 {
        todo!()
    }

    fn bmi(&mut self) -> u8 {
        todo!()
    }

    fn bne(&mut self) -> u8 {
        todo!()
    }

    fn bpl(&mut self) -> u8 {
        todo!()
    }

    fn brk(&mut self) -> u8 {
        todo!()
    }

    fn bvc(&mut self) -> u8 {
        todo!()
    }

    fn bvs(&mut self) -> u8 {
        todo!()
    }

    fn clc(&mut self) -> u8 {
        self.set_flag(C, false);
        0
    }

    fn cld(&mut self) -> u8 {
        self.set_flag(D, false);
        0
    }

    fn cli(&mut self) -> u8 {
        self.set_flag(I, false);
        0
    }

    fn clv(&mut self) -> u8 {
        self.set_flag(V, false);
        0
    }

    fn cmp(&mut self) -> u8 {
        todo!()
    }

    fn cpx(&mut self) -> u8 {
        todo!()
    }

    fn cpy(&mut self) -> u8 {
        todo!()
    }

    fn dec(&mut self) -> u8 {
        todo!()
    }

    fn dex(&mut self) -> u8 {
        todo!()
    }

    fn dey(&mut self) -> u8 {
        todo!()
    }

    fn eor(&mut self) -> u8 {
        todo!()
    }

    fn inc(&mut self) -> u8 {
        todo!()
    }

    fn inx(&mut self) -> u8 {
        todo!()
    }

    fn iny(&mut self) -> u8 {
        todo!()
    }

    fn jmp(&mut self) -> u8 {
        todo!()
    }

    fn jsr(&mut self) -> u8 {
        todo!()
    }

    fn lda(&mut self) -> u8 {
        todo!()
    }

    fn ldx(&mut self) -> u8 {
        todo!()
    }

    fn ldy(&mut self) -> u8 {
        todo!()
    }

    fn lsr(&mut self) -> u8 {
        todo!()
    }

    fn nop(&mut self) -> u8 {
        todo!()
    }

    fn ora(&mut self) -> u8 {
        todo!()
    }

    fn pha(&mut self) -> u8 {
        self.write(0x0100 + self.stack_ptr as u16, self.a);
        self.stack_ptr -= 1;
        0
    }

    fn php(&mut self) -> u8 {
        todo!()
    }

    fn pla(&mut self) -> u8 {
        todo!()
    }

    fn plp(&mut self) -> u8 {
        todo!()
    }

    fn rol(&mut self) -> u8 {
        todo!()
    }

    fn ror(&mut self) -> u8 {
        todo!()
    }

    fn rti(&mut self) -> u8 {
        self.stack_ptr += 1;
        self.status = self.read(0x0100 + self.stack_ptr as u16);
        self.status &= !(B as u8);
        self.status &= !(U as u8);

        self.stack_ptr += 1;
        self.pc = self.read(0x0100 + self.stack_ptr as u16) as u16;
        self.stack_ptr += 1;
        self.pc |= (self.read(0x0100 + self.stack_ptr as u16) as u16) << 8;
        0
    }

    fn rts(&mut self) -> u8 {
        todo!()
    }

    fn sbc(&mut self) -> u8 {
        self.fetch();
        let value = (self.fetched as u16) ^ 0x00FF;
        let temp = self.a as u16 + value + self.get_flag(C) as u16;
        self.set_flag(C, temp > 255);
        self.set_flag(Z, (temp & 0x80) != 0);
        self.set_flag(
            V,
            (!(temp ^ self.a as u16) & (temp ^ value) & 0x0080) != 0,
        );
        self.set_flag(N, (temp & 0x0080) != 0);
        self.a = (temp & 0x00FF) as u8;
        1
    }

    fn sec(&mut self) -> u8 {
        todo!()
    }

    fn sed(&mut self) -> u8 {
        todo!()
    }

    fn sei(&mut self) -> u8 {
        todo!()
    }

    fn sta(&mut self) -> u8 {
        todo!()
    }

    fn stx(&mut self) -> u8 {
        todo!()
    }

    fn sty(&mut self) -> u8 {
        todo!()
    }

    fn tax(&mut self) -> u8 {
        todo!()
    }

    fn tay(&mut self) -> u8 {
        todo!()
    }

    fn tsx(&mut self) -> u8 {
        todo!()
    }

    fn txa(&mut self) -> u8 {
        todo!()
    }

    fn txs(&mut self) -> u8 {
        todo!()
    }

    fn tya(&mut self) -> u8 {
        todo!()
    }

    fn igl(&mut self) -> u8 {
        todo!()
    }
}
