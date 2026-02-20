#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OpCode {
    // Data movement.
    LI,
    LF,
    MV,
    // Semantic operations.
    ADD,
    SUB,
    MUL,
    DIV,
    INF,
    ADT,
    // Heuristic operations.
    EQV,
    INT,
    HAL,
    SIM,
    // Control flow.
    BEQ,
    BLE,
    BLT,
    BGE,
    BGT,
    // I/O.
    OUT,
    // Misc.
    EXIT,
}

static OP_CODE_MAPPING: [(OpCode, u32); 20] = [
    // Data movement.
    (OpCode::LI, 0x00),
    (OpCode::LF, 0x01),
    (OpCode::MV, 0x02),
    // Semantic operations.
    (OpCode::ADD, 0x03),
    (OpCode::SUB, 0x04),
    (OpCode::MUL, 0x05),
    (OpCode::DIV, 0x06),
    (OpCode::INF, 0x07),
    (OpCode::ADT, 0x08),
    // Heuristic operations.
    (OpCode::EQV, 0x09),
    (OpCode::INT, 0x0A),
    (OpCode::HAL, 0x0B),
    (OpCode::SIM, 0x0C),
    // Control flow.
    (OpCode::BEQ, 0x0D),
    (OpCode::BLE, 0x0E),
    (OpCode::BLT, 0x0F),
    (OpCode::BGE, 0x10),
    (OpCode::BGT, 0x11),
    // I/O.
    (OpCode::OUT, 0x12),
    // Misc.
    (OpCode::EXIT, 0xFF),
];

impl OpCode {
    pub fn from_be_bytes(be_bytes: [u8; 4]) -> Result<OpCode, &'static str> {
        let value = u32::from_be_bytes(be_bytes);

        for (opcode, code_value) in OP_CODE_MAPPING.iter() {
            if code_value == &value {
                return Ok(opcode.clone());
            }
        }

        Err("Byte value does not correspond to any known opcode.")
    }

    pub fn to_be_bytes(&self) -> Result<[u8; 4], &'static str> {
        for (opcode, code_value) in OP_CODE_MAPPING.iter() {
            if opcode == self {
                return Ok(code_value.to_be_bytes());
            }
        }

        Err("Opcode not found in mapping.")
    }
}
