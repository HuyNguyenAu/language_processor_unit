pub enum OperandType {
    NUMBER = 0x00,
    TEXT = 0x01,
    REGISTER = 0x02,
}

impl OperandType {
    pub fn from_bytecode(byte: &u8) -> Result<OperandType, &'static str> {
        match byte {
            0x00 => Ok(OperandType::NUMBER),
            0x01 => Ok(OperandType::TEXT),
            0x02 => Ok(OperandType::REGISTER),
            _ => Err("Invalid operand type byte."),
        }
    }
}

#[derive(Debug)]
pub enum Operand {
    Number(u8),
    Text(String),
    Register(u8),
}