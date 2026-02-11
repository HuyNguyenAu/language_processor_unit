#[derive(Debug)]
pub enum OpCode {
    MOV = 0x00,
    ADD = 0x01,
    SUB = 0x02,
    SIM = 0x03,
    JEQ = 0x04,
    JLT = 0x05,
    JLE = 0x06,
    JGT = 0x07,
    JGE = 0x08,
    OUT = 0x09,
    LOAD = 0x0A,
}

impl OpCode {
    pub fn from_byte(byte: &u8) -> Result<OpCode, &'static str> {
        match byte {
            0x00 => Ok(OpCode::MOV),
            0x01 => Ok(OpCode::ADD),
            0x02 => Ok(OpCode::SUB),
            0x03 => Ok(OpCode::SIM),
            0x04 => Ok(OpCode::JEQ),
            0x05 => Ok(OpCode::JLT),
            0x06 => Ok(OpCode::JLE),
            0x07 => Ok(OpCode::JGT),
            0x08 => Ok(OpCode::JGE),
            0x09 => Ok(OpCode::OUT),
            0x0A => Ok(OpCode::LOAD),
            _ => Err("Invalid opcode byte."),
        }
    }
}