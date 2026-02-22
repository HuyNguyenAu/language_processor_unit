#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OpCode {
    // Data movement.
    Li,
    Lf,
    Mv,
    // Control flow.
    Beq,
    Ble,
    Blt,
    Bge,
    Bgt,
    Exit,
    // I/O.
    Out,
    // Generative operations.
    Sum,
    Exp,
    Trn,
    // Cognitive operations.
    Cmp,
    Syn,
    Flt,
    Prd,
    // Guardrails operations.
    Vfy,
    Sim,
}

static OP_CODE_MAPPING: [(OpCode, u32); 19] = [
    // Data movement.
    (OpCode::Li, 0x00),
    (OpCode::Lf, 0x01),
    (OpCode::Mv, 0x02),
    // Control flow.
    (OpCode::Beq, 0x03),
    (OpCode::Ble, 0x04),
    (OpCode::Blt, 0x05),
    (OpCode::Bge, 0x06),
    (OpCode::Bgt, 0x07),
    (OpCode::Exit, 0x08),
    // I/O.
    (OpCode::Out, 0x09),
    // Generative operations.
    (OpCode::Sum, 0x0A),
    (OpCode::Exp, 0x0B),
    (OpCode::Trn, 0x0C),
    // Cognitive operations.
    (OpCode::Cmp, 0x0D),
    (OpCode::Syn, 0x0E),
    (OpCode::Flt, 0x0F),
    (OpCode::Prd, 0x10),
    // Guardrails operations.
    (OpCode::Vfy, 0x11),
    (OpCode::Sim, 0x12),
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
