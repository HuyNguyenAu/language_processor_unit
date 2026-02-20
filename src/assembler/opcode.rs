#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OpCode {
    // Data movement.
    Li,
    Lf,
    Mv,
    // Semantic operations.
    Add,
    Sub,
    Mul,
    Div,
    Inf,
    Adt,
    // Heuristic operations.
    Eqv,
    Int,
    Hal,
    Sim,
    // Control flow.
    Beq,
    Ble,
    Blt,
    Bge,
    Bgt,
    // I/O.
    Out,
    // Misc.
    Exit,
}

static OP_CODE_MAPPING: [(OpCode, u32); 20] = [
    // Data movement.
    (OpCode::Li, 0x00),
    (OpCode::Lf, 0x01),
    (OpCode::Mv, 0x02),
    // Semantic operations.
    (OpCode::Add, 0x03),
    (OpCode::Sub, 0x04),
    (OpCode::Mul, 0x05),
    (OpCode::Div, 0x06),
    (OpCode::Inf, 0x07),
    (OpCode::Adt, 0x08),
    // Heuristic operations.
    (OpCode::Eqv, 0x09),
    (OpCode::Int, 0x0A),
    (OpCode::Hal, 0x0B),
    (OpCode::Sim, 0x0C),
    // Control flow.
    (OpCode::Beq, 0x0D),
    (OpCode::Ble, 0x0E),
    (OpCode::Blt, 0x0F),
    (OpCode::Bge, 0x10),
    (OpCode::Bgt, 0x11),
    // I/O.
    (OpCode::Out, 0x12),
    // Misc.
    (OpCode::Exit, 0xFF),
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
