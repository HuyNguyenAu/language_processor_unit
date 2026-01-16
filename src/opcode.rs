#[derive(Debug)]
pub enum OpCode {
    MOV,
    ADD,
    SUB,
    SIM,
    JEQ,
    JLT,
    JLE,
    JGT,
    JGE,
    OUT,
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
            _ => Err("Invalid opcode byte."),
        }
    }
}