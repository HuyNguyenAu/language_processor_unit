#[derive(Debug)]
pub struct LoadStringInstruction {
    pub destination_register: u32,
    pub value: String,
}
#[derive(Debug)]
pub struct LoadImmediateInstruction {
    pub destination_register: u32,
    pub value: u32,
}

#[derive(Debug)]
pub struct LoadFileInstruction {
    pub destination_register: u32,
    pub file_path: String,
}

#[derive(Debug)]
pub struct MoveInstruction {
    pub destination_register: u32,
    pub source_register: u32,
}

#[derive(Debug)]
pub struct MorphInstruction {
    pub destination_register: u32,
    pub source_register: u32,
}

#[derive(Debug)]
pub struct ProjectInstruction {
    pub destination_register: u32,
    pub source_register: u32,
}

#[derive(Debug)]
pub struct DistillInstruction {
    pub destination_register: u32,
    pub source_register: u32,
}

#[derive(Debug)]
pub struct CorrelateInstruction {
    pub destination_register: u32,
    pub source_register: u32,
}

#[derive(Debug)]
pub struct AuditInstruction {
    pub destination_register: u32,
    pub source_register: u32,
}

#[derive(Debug)]
pub struct SimilarityInstruction {
    pub destination_register: u32,
    pub source_register_1: u32,
    pub source_register_2: u32,
}

#[derive(Debug)]
pub enum BranchType {
    Equal,
    LessEqual,
    Less,
    GreaterEqual,
    Greater,
}

#[derive(Debug)]
pub struct BranchInstruction {
    pub branch_type: BranchType,
    pub source_register_1: u32,
    pub source_register_2: u32,
    pub instruction_pointer_jump_index: u32,
}

#[derive(Debug)]
pub struct ContextClearInstruction;

#[derive(Debug)]
pub struct ContextSnapshotInstruction {
    pub destination_register: u32,
}

#[derive(Debug)]
pub struct ContextRestoreInstruction {
    pub source_register: u32,
}

#[derive(Debug)]
pub struct ContextPushInstruction {
    pub source_register: u32,
}

#[derive(Debug)]
pub struct ContextPopInstruction {
    pub destination_register: u32,
}

#[derive(Debug)]
pub struct ContextDropInstruction;

#[derive(Debug)]
pub struct ContextSetRoleInstruction {
    pub role: String,
}

#[derive(Debug)]
pub struct DecrementInstruction {
    pub source_register: u32,
    pub value: u32,
}

#[derive(Debug)]
pub struct OutputInstruction {
    pub source_register: u32,
}

#[derive(Debug)]
pub struct ExitInstruction;

#[derive(Debug)]
pub enum Instruction {
    // Data movement.
    LoadString(LoadStringInstruction),
    LoadImmediate(LoadImmediateInstruction),
    LoadFile(LoadFileInstruction),
    Move(MoveInstruction),
    // Control flow.
    Branch(BranchInstruction),
    Exit(ExitInstruction),
    // I/O.
    Output(OutputInstruction),
    // Generative operations.
    Morph(MorphInstruction),
    Project(ProjectInstruction),
    // Cognitive operations.
    Distill(DistillInstruction),
    Correlate(CorrelateInstruction),
    // Guardrails operations.
    Audit(AuditInstruction),
    Similarity(SimilarityInstruction),
    // Context operations.
    ContextClear(ContextClearInstruction),
    ContextSnapshot(ContextSnapshotInstruction),
    ContextRestore(ContextRestoreInstruction),
    ContextPush(ContextPushInstruction),
    ContextPop(ContextPopInstruction),
    ContextDrop(ContextDropInstruction),
    ContextSetRole(ContextSetRoleInstruction),
    // Misc.
    Decrement(DecrementInstruction),
}
