#[derive(Debug)]
pub enum OpCode {
    MOV,
    ADD,
    SUB,
    MUL,
    DIV,
    SIM,
    JEQ,
    JLT,
    JLE,
    JGT,
    JGE,
    OUT,
    LOAD,
}

impl OpCode {
    pub fn from_be_bytes(be_bytes: [u8; 4]) -> Result<OpCode, &'static str> {
        return match u32::from_be_bytes(be_bytes) {
            0x00 => Ok(OpCode::MOV),
            0x01 => Ok(OpCode::ADD),
            0x02 => Ok(OpCode::SUB),
            0x03 => Ok(OpCode::MUL),
            0x04 => Ok(OpCode::DIV),
            0x05 => Ok(OpCode::SIM),
            0x06 => Ok(OpCode::JEQ),
            0x07 => Ok(OpCode::JLT),
            0x08 => Ok(OpCode::JLE),
            0x09 => Ok(OpCode::JGT),
            0x0A => Ok(OpCode::JGE),
            0x0B => Ok(OpCode::OUT),
            0x0C => Ok(OpCode::LOAD),
            _ => Err("Invalid opcode byte."),
        };
    }

    pub fn to_be_bytes(&self) -> [u8; 4] {
        let value: u32 = match self {
            OpCode::MOV => 0x00,
            OpCode::ADD => 0x01,
            OpCode::SUB => 0x02,
            OpCode::MUL => 0x03,
            OpCode::DIV => 0x04,
            OpCode::SIM => 0x05,
            OpCode::JEQ => 0x06,
            OpCode::JLT => 0x07,
            OpCode::JLE => 0x08,
            OpCode::JGT => 0x09,
            OpCode::JGE => 0x0A,
            OpCode::OUT => 0x0B,
            OpCode::LOAD => 0x0C,
        };

        return value.to_be_bytes();
    }
}
