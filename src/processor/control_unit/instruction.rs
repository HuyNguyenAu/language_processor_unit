use crate::assembler::immediate::Immediate;

#[derive(Debug)]
pub struct LoadImmediateInstruction {
    pub destination_register: u32,
    pub value: Immediate,
}

#[derive(Debug)]
pub struct LoadFileInstruction {
    pub destination_register: u32,
    pub value: String,
}

#[derive(Debug)]
pub struct MoveInstruction {
    pub destination_register: u32,
    pub source_register: u32,
}

#[derive(Debug)]
pub enum RType {
    // Generative operations.
    Sum,
    Xpn,
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

#[derive(Debug)]
pub struct RTypeInstruction {
    pub r_type: RType,
    pub destination_register: u32,
    pub immediate_1: Immediate,
    pub immediate_2: Immediate,
}

#[derive(Debug)]
pub enum BranchType {
    Eq,
    Le,
    Lt,
    Ge,
    Gt,
}

#[derive(Debug)]
pub struct BranchInstruction {
    pub branch_type: BranchType,
    pub immediate_1: Immediate,
    pub immediate_2: Immediate,
    pub byte_code_index: u32,
}

#[derive(Debug)]
pub struct OutputInstruction {
    pub immediate: Immediate,
}

#[derive(Debug)]
pub struct ExitInstruction;

#[derive(Debug)]
pub enum Instruction {
    LoadImmediate(LoadImmediateInstruction),
    LoadFile(LoadFileInstruction),
    Move(MoveInstruction),
    RType(RTypeInstruction),
    Branch(BranchInstruction),
    Output(OutputInstruction),
    Exit(ExitInstruction),
}
