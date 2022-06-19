use crate::cpu::opcode::OpCode;

use super::addr_mode::AddrMode;

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
