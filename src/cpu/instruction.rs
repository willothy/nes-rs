use super::{AddrMode};

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

pub struct Instruction {
    pub name: String,
    pub opcode: OpCode,
    pub addr_mode: AddrMode,
    pub cycles: u8,
}

impl From<u8> for Instruction {
    fn from(opcode: u8) -> Self {
        use AddrMode::*;
        use OpCode::*;
        match opcode {
            _ => Instruction {
                name: "Illegal".to_owned(),
                opcode: IGL,
                addr_mode: IMM,
                cycles: 0,
            },
        }
    }
}
