#[derive(Debug, PartialEq, Clone)]
pub enum ImmediateType {
    TEXT,
    NUMBER,
    REGISTER,
}

static IMMEDIATE_TYPE_MAPPING: [(ImmediateType, u32); 3] = [
    (ImmediateType::NUMBER, 0x00),
    (ImmediateType::TEXT, 0x01),
    (ImmediateType::REGISTER, 0x02),
];

impl ImmediateType {
    pub fn from_be_bytes(be_bytes: [u8; 4]) -> Result<ImmediateType, &'static str> {
        let value = u32::from_be_bytes(be_bytes);

        for (immediate_type, type_value) in IMMEDIATE_TYPE_MAPPING.iter() {
            if type_value == &value {
                return Ok(immediate_type.to_owned());
            }
        }

        Err("Byte value does not correspond to any known immediate type.")
    }

    pub fn to_be_bytes(&self) -> Result<[u8; 4], &'static str> {
        for (immediate_type, type_value) in IMMEDIATE_TYPE_MAPPING.iter() {
            if immediate_type == self {
                return Ok(type_value.to_be_bytes());
            }
        }

        Err("Immediate type does not correspond to any known byte value.")
    }
}

#[derive(Debug)]
pub enum Immediate {
    Text(String),
    Number(u32),
    Register(u32),
}
