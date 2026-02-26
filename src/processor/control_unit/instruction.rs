#[derive(Debug)]
pub struct LoadStringInstruction {
    pub id: u32,
    pub destination_register: u32,
    pub value: String,
}
#[derive(Debug)]
pub struct LoadImmediateInstruction {
    pub id: u32,
    pub destination_register: u32,
    pub value: u32,
}

#[derive(Debug)]
pub struct LoadFileInstruction {
    pub id: u32,
    pub destination_register: u32,
    pub file_path: String,
}

#[derive(Debug)]
pub struct MoveInstruction {
    pub id: u32,
    pub destination_register: u32,
    pub source_register: u32,
}

#[derive(Debug)]
pub enum RType {
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

#[derive(Debug)]
pub struct RTypeInstruction {
    pub id: u32,
    pub r_type: RType,
    pub destination_register: u32,
    pub source_register_1: u32,
    pub source_register_2: u32,
}

#[derive(Debug)]
pub enum BType {
    Equal,
    LessEqual,
    Less,
    GreaterEqual,
    Greater,
}

#[derive(Debug)]
pub struct BTypeInstruction {
    pub b_type: BType,
    pub source_register_1: u32,
    pub source_register_2: u32,
    pub instruction_pointer_jump_index: u32,
}

#[derive(Debug)]
pub struct OutputInstruction {
    pub source_register: u32,
}

#[derive(Debug)]
pub struct ExitInstruction;

#[derive(Debug)]
pub enum Instruction {
    LoadString(LoadStringInstruction),
    LoadImmediate(LoadImmediateInstruction),
    LoadFile(LoadFileInstruction),
    Move(MoveInstruction),
    RType(RTypeInstruction),
    BType(BTypeInstruction),
    Output(OutputInstruction),
    Exit(ExitInstruction),
}
