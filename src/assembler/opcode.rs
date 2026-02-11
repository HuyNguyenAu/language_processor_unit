#[derive(Debug)]
pub enum OpCode {
    MOV = 0x00,
    ADD = 0x01,
    SUB = 0x02,
    MUL = 0x03,
    DIV = 0x04,
    SIM = 0x05,
    JEQ = 0x06,
    JLT = 0x07,
    JLE = 0x08,
    JGT = 0x09,
    JGE = 0x0A,
    OUT = 0x0B,
    LOAD = 0x0C,
}

impl OpCode {
    pub fn from_byte(byte: &u8) -> Result<OpCode, &'static str> {
        return match byte {
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
}
