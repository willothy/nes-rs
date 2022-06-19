use super::OLC6502;

#[derive(PartialEq)]
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
    fn imp(&mut self) -> u8;
    fn imm(&mut self) -> u8;
    fn zp0(&mut self) -> u8;
    fn zpx(&mut self) -> u8;
    fn zpy(&mut self) -> u8;
    fn rel(&mut self) -> u8;
    fn abs(&mut self) -> u8;
    fn abx(&mut self) -> u8;
    fn aby(&mut self) -> u8;
    fn ind(&mut self) -> u8;
    fn izx(&mut self) -> u8;
    fn izy(&mut self) -> u8;
}

impl AddressModes for OLC6502 {
    fn imp(&mut self) -> u8 {
        self.fetched = self.a;
        0
    }

    fn imm(&mut self) -> u8 {
        self.pc += 1;
        self.addr_abs = self.pc;
        0
    }

    fn zp0(&mut self) -> u8 {
        self.addr_abs = self.read(self.pc as u16) as u16;
        self.pc += 1;
        self.addr_abs &= 0x00FF;
        0
    }

    fn zpx(&mut self) -> u8 {
        self.addr_abs = self.read(self.pc + self.x as u16) as u16;
        self.pc += 1;
        self.addr_abs &= 0x00FF;
        0
    }

    fn zpy(&mut self) -> u8 {
        self.addr_abs = self.read(self.pc + self.y as u16) as u16;
        self.pc += 1;
        self.addr_abs &= 0x00FF;
        0
    }

    fn rel(&mut self) -> u8 {
        self.addr_rel = self.read(self.pc as u16);
        self.pc += 1;
        if (self.addr_rel & 0x80) != 0x0 {
            self.addr_rel |= 0xFF;
        }
        0
    }

    fn abs(&mut self) -> u8 {
        let low: u16 = self.read(self.pc as u16) as u16;
        self.pc += 1;
        let high: u16 = self.read(self.pc as u16) as u16;
        self.pc += 1;

        self.addr_abs = ((high) << 8) | low;

        0
    }

    fn abx(&mut self) -> u8 {
        let low: u16 = self.read(self.pc as u16) as u16;
        self.pc += 1;
        let high: u16 = self.read(self.pc as u16) as u16;
        self.pc += 1;

        self.addr_abs = (((high) << 8) | low) + self.x as u16;

        if (self.addr_abs & 0xFF00) != (high << 8) {
            1
        } else {
            0
        }
    }

    fn aby(&mut self) -> u8 {
        let low: u16 = self.read(self.pc as u16) as u16;
        self.pc += 1;
        let high: u16 = self.read(self.pc as u16) as u16;
        self.pc += 1;

        self.addr_abs = (((high) << 8) | low) + self.y as u16;

        if (self.addr_abs & 0xFF00) != (high << 8) {
            1
        } else {
            0
        }
    }

    fn ind(&mut self) -> u8 {
        let ptr_low = self.read(self.pc as u16) as u16;
        self.pc += 1;
        let ptr_high = self.read(self.pc as u16) as u16;
        self.pc += 1;

        let ptr = (ptr_high << 8) | ptr_low;

        if ptr_low == 0x00FF {
            // Simulate page boundary hardware bug
            self.addr_abs = ((self.read(ptr & 0xFF00) as u16) << 8) | self.read(ptr) as u16;
        } else {
            // Normal operation
            self.addr_abs = ((self.read(ptr + 1) as u16) << 8) | self.read(ptr) as u16;
        }

        0
    }

    fn izx(&mut self) -> u8 {
        let t: u16 = self.read(self.pc as u16) as u16;
        self.pc += 1;

        let low = self.read(t + self.x as u16) as u16 & 0x00FF;
        let high = self.read(t + self.x as u16 + 1) as u16 & 0x00FF;

        self.addr_abs = (high << 8) | low;

        0
    }

    fn izy(&mut self) -> u8 {
        let t: u16 = self.read(self.pc as u16) as u16;
        self.pc += 1;

        let low = self.read(t & 0x00FF) as u16;
        let high = self.read((t + 1) & 0x00FF) as u16;

        self.addr_abs = (high << 8) | low;
        self.addr_abs += self.y as u16;

        if (self.addr_abs & 0xFF00) != (high << 8) {
            1
        } else {
            0
        }
    }
}
