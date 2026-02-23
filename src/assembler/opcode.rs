#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OpCode {
    // Data movement.
    LoadImmediate,
    LoadFile,
    Move,
    // Control flow.
    BranchEqual,
    BranchLessEqual,
    BranchLess,
    BranchGreaterEqual,
    BranchGreater,
    Exit,
    // I/O.
    Out,
    // Generative operations.
    Morph,
    Project,
    // Cognitive operations.
    Distill,
    Correlate,
    // Guardrails operations.
    Audit,
    Similarity,
}

static OP_CODE_MAPPING: [(OpCode, u32); 16] = [
    // Data movement.
    (OpCode::LoadFile, 0x00),
    (OpCode::LoadImmediate, 0x01),
    (OpCode::Move, 0x02),
    // Control flow.
    (OpCode::BranchEqual, 0x03),
    (OpCode::BranchLessEqual, 0x04),
    (OpCode::BranchLess, 0x05),
    (OpCode::BranchGreaterEqual, 0x06),
    (OpCode::BranchGreater, 0x07),
    (OpCode::Exit, 0x08),
    // I/O.
    (OpCode::Out, 0x09),
    // Generative operations.
    (OpCode::Morph, 0x0A),
    (OpCode::Project, 0x0B),
    // Cognitive operations.
    (OpCode::Distill, 0x0C),
    (OpCode::Correlate, 0x0D),
    // Guardrails operations.
    (OpCode::Audit, 0x0E),
    (OpCode::Similarity, 0x0F),
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
