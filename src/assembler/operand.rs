pub enum OperandType {
    NUMBER,
    TEXT,
    REGISTER,
}

impl OperandType {
    pub fn from_be_bytes(be_bytes: [u8; 4]) -> Result<OperandType, &'static str> {
        return match u32::from_be_bytes(be_bytes) {
            0x00 => Ok(OperandType::NUMBER),
            0x01 => Ok(OperandType::TEXT),
            0x02 => Ok(OperandType::REGISTER),
            _ => Err("Invalid operand type byte."),
        };
    }

    pub fn to_be_bytes(&self) -> [u8; 4] {
        let value: u32 = match self {
            OperandType::NUMBER => 0x00,
            OperandType::TEXT => 0x01,
            OperandType::REGISTER => 0x02,
        };

        return value.to_be_bytes();
    }
}

#[derive(Debug)]
pub enum Operand {
    Number(u32),
    Text(String),
    Register(u32),
}
