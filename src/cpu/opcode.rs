use crate::cpu::StatusFlags;

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
    fn ADC(&mut self) -> u8;
    fn AND(&mut self) -> u8;
    fn ASL(&mut self) -> u8;
    fn BCC(&mut self) -> u8;
    fn BCS(&mut self) -> u8;
    fn BEQ(&mut self) -> u8;
    fn BIT(&mut self) -> u8;
    fn BMI(&mut self) -> u8;
    fn BNE(&mut self) -> u8;
    fn BPL(&mut self) -> u8;
    fn BRK(&mut self) -> u8;
    fn BVC(&mut self) -> u8;
    fn BVS(&mut self) -> u8;
    fn CLC(&mut self) -> u8;
    fn CLD(&mut self) -> u8;
    fn CLI(&mut self) -> u8;
    fn CLV(&mut self) -> u8;
    fn CMP(&mut self) -> u8;
    fn CPX(&mut self) -> u8;
    fn CPY(&mut self) -> u8;
    fn DEC(&mut self) -> u8;
    fn DEX(&mut self) -> u8;
    fn DEY(&mut self) -> u8;
    fn EOR(&mut self) -> u8;
    fn INC(&mut self) -> u8;
    fn INX(&mut self) -> u8;
    fn INY(&mut self) -> u8;
    fn JMP(&mut self) -> u8;
    fn JSR(&mut self) -> u8;
    fn LDA(&mut self) -> u8;
    fn LDX(&mut self) -> u8;
    fn LDY(&mut self) -> u8;
    fn LSR(&mut self) -> u8;
    fn NOP(&mut self) -> u8;
    fn ORA(&mut self) -> u8;
    fn PHA(&mut self) -> u8;
    fn PHP(&mut self) -> u8;
    fn PLA(&mut self) -> u8;
    fn PLP(&mut self) -> u8;
    fn ROL(&mut self) -> u8;
    fn ROR(&mut self) -> u8;
    fn RTI(&mut self) -> u8;
    fn RTS(&mut self) -> u8;
    fn SBC(&mut self) -> u8;
    fn SEC(&mut self) -> u8;
    fn SED(&mut self) -> u8;
    fn SEI(&mut self) -> u8;
    fn STA(&mut self) -> u8;
    fn STX(&mut self) -> u8;
    fn STY(&mut self) -> u8;
    fn TAX(&mut self) -> u8;
    fn TAY(&mut self) -> u8;
    fn TSX(&mut self) -> u8;
    fn TXA(&mut self) -> u8;
    fn TXS(&mut self) -> u8;
    fn TYA(&mut self) -> u8;
    fn IGL(&mut self) -> u8;
}

impl<'cpu> Operations for OLC6502<'cpu> {
    fn ADC(&mut self) -> u8 {
        todo!()
    }

    fn AND(&mut self) -> u8 {
        use StatusFlags::*;
        self.fetch();
        self.a = self.a & self.fetched;
        self.set_flag(Z, self.a == 0x00);
        self.set_flag(N, (self.a & 0x80) != 0);
        1
    }

    fn ASL(&mut self) -> u8 {
        todo!()
    }

    fn BCC(&mut self) -> u8 {
        todo!()
    }

    fn BCS(&mut self) -> u8 {
        todo!()
    }

    fn BEQ(&mut self) -> u8 {
        todo!()
    }

    fn BIT(&mut self) -> u8 {
        todo!()
    }

    fn BMI(&mut self) -> u8 {
        todo!()
    }

    fn BNE(&mut self) -> u8 {
        todo!()
    }

    fn BPL(&mut self) -> u8 {
        todo!()
    }

    fn BRK(&mut self) -> u8 {
        todo!()
    }

    fn BVC(&mut self) -> u8 {
        todo!()
    }

    fn BVS(&mut self) -> u8 {
        todo!()
    }

    fn CLC(&mut self) -> u8 {
        todo!()
    }

    fn CLD(&mut self) -> u8 {
        todo!()
    }

    fn CLI(&mut self) -> u8 {
        todo!()
    }

    fn CLV(&mut self) -> u8 {
        todo!()
    }

    fn CMP(&mut self) -> u8 {
        todo!()
    }

    fn CPX(&mut self) -> u8 {
        todo!()
    }

    fn CPY(&mut self) -> u8 {
        todo!()
    }

    fn DEC(&mut self) -> u8 {
        todo!()
    }

    fn DEX(&mut self) -> u8 {
        todo!()
    }

    fn DEY(&mut self) -> u8 {
        todo!()
    }

    fn EOR(&mut self) -> u8 {
        todo!()
    }

    fn INC(&mut self) -> u8 {
        todo!()
    }

    fn INX(&mut self) -> u8 {
        todo!()
    }

    fn INY(&mut self) -> u8 {
        todo!()
    }

    fn JMP(&mut self) -> u8 {
        todo!()
    }

    fn JSR(&mut self) -> u8 {
        todo!()
    }

    fn LDA(&mut self) -> u8 {
        todo!()
    }

    fn LDX(&mut self) -> u8 {
        todo!()
    }

    fn LDY(&mut self) -> u8 {
        todo!()
    }

    fn LSR(&mut self) -> u8 {
        todo!()
    }

    fn NOP(&mut self) -> u8 {
        todo!()
    }

    fn ORA(&mut self) -> u8 {
        todo!()
    }

    fn PHA(&mut self) -> u8 {
        todo!()
    }

    fn PHP(&mut self) -> u8 {
        todo!()
    }

    fn PLA(&mut self) -> u8 {
        todo!()
    }

    fn PLP(&mut self) -> u8 {
        todo!()
    }

    fn ROL(&mut self) -> u8 {
        todo!()
    }

    fn ROR(&mut self) -> u8 {
        todo!()
    }

    fn RTI(&mut self) -> u8 {
        todo!()
    }

    fn RTS(&mut self) -> u8 {
        todo!()
    }

    fn SBC(&mut self) -> u8 {
        todo!()
    }

    fn SEC(&mut self) -> u8 {
        todo!()
    }

    fn SED(&mut self) -> u8 {
        todo!()
    }

    fn SEI(&mut self) -> u8 {
        todo!()
    }

    fn STA(&mut self) -> u8 {
        todo!()
    }

    fn STX(&mut self) -> u8 {
        todo!()
    }

    fn STY(&mut self) -> u8 {
        todo!()
    }

    fn TAX(&mut self) -> u8 {
        todo!()
    }

    fn TAY(&mut self) -> u8 {
        todo!()
    }

    fn TSX(&mut self) -> u8 {
        todo!()
    }

    fn TXA(&mut self) -> u8 {
        todo!()
    }

    fn TXS(&mut self) -> u8 {
        todo!()
    }

    fn TYA(&mut self) -> u8 {
        todo!()
    }

    fn IGL(&mut self) -> u8 {
        todo!()
    }
}