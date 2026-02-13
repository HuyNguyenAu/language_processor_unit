#[derive(Debug)]
pub enum ImmediateType {
    TEXT,
    NUMBER,
}

impl ImmediateType {
    pub fn from_be_bytes(be_bytes: [u8; 4]) -> Result<ImmediateType, &'static str> {
        return match u32::from_be_bytes(be_bytes) {
            0x00 => Ok(ImmediateType::NUMBER),
            0x01 => Ok(ImmediateType::TEXT),
            _ => Err("Invalid immediate type in bytes."),
        };
    }

    pub fn to_be_bytes(&self) -> [u8; 4] {
        let value: u32 = match self {
            ImmediateType::NUMBER => 0x00,
            ImmediateType::TEXT => 0x01,
        };

        return value.to_be_bytes();
    }
}

#[derive(Debug)]
pub enum Immediate {
    Text(String),
    Number(u32),
}
