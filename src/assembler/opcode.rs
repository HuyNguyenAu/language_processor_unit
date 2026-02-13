#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OpCode {
    LI,
    LF,
    MV,
    ADD,
    SUB,
    MUL,
    DIV,
    SIM,
    BEQ,
    BLT,
    BLE,
    BGT,
    BGE,
    OUT,
}

impl OpCode {
    pub fn from_be_bytes(be_bytes: [u8; 4]) -> Result<OpCode, &'static str> {
        return match u32::from_be_bytes(be_bytes) {
            0x00 => Ok(OpCode::LI),
            0x01 => Ok(OpCode::LF),
            0x02 => Ok(OpCode::MV),
            0x03 => Ok(OpCode::ADD),
            0x04 => Ok(OpCode::SUB),
            0x05 => Ok(OpCode::MUL),
            0x06 => Ok(OpCode::DIV),
            0x07 => Ok(OpCode::SIM),
            0x08 => Ok(OpCode::BEQ),
            0x09 => Ok(OpCode::BLT),
            0x0A => Ok(OpCode::BLE),
            0x0B => Ok(OpCode::BGT),
            0x0C => Ok(OpCode::BGE),
            0x0D => Ok(OpCode::OUT),
            _ => Err("Invalid opcode byte."),
        };
    }

    pub fn to_be_bytes(&self) -> [u8; 4] {
        let value: u32 = match self {
            OpCode::LI => 0x00,
            OpCode::LF => 0x01,
            OpCode::MV => 0x02,
            OpCode::ADD => 0x03,
            OpCode::SUB => 0x04,
            OpCode::MUL => 0x05,
            OpCode::DIV => 0x06,
            OpCode::SIM => 0x07,
            OpCode::BEQ => 0x08,
            OpCode::BLT => 0x09,
            OpCode::BLE => 0x0A,
            OpCode::BGT => 0x0B,
            OpCode::BGE => 0x0C,
            OpCode::OUT => 0x0D,
        };

        return value.to_be_bytes();
    }
}
