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
pub enum SemanticType {
    ADD,
    SUB,
    MUL,
    DIV,
    INF,
    ADT,
}

#[derive(Debug)]
pub struct SemanticInstruction {
    pub semantic_type: SemanticType,
    pub destination_register: u32,
    pub immediate_1: Immediate,
    pub immediate_2: Immediate,
}

#[derive(Debug)]
pub enum HeuristicType {
    EQV,
    INT,
    HAL,
    SIM,
}

#[derive(Debug)]
pub struct HeuristicInstruction {
    pub heuristic_type: HeuristicType,
    pub destination_register: u32,
    pub immediate_1: Immediate,
    pub immediate_2: Immediate,
}

#[derive(Debug)]
pub enum BranchType {
    EQ,
    LE,
    LT,
    GE,
    GT,
}

#[derive(Debug)]
pub struct BranchInstruction {
    pub branch_type: BranchType,
    pub source_register_1: u32,
    pub source_register_2: u32,
    pub byte_code_index: u32,
}

#[derive(Debug)]
pub struct OutputInstruction {
    pub source_register: u32,
}

#[derive(Debug)]
pub struct ExitInstruction;

#[derive(Debug)]
pub enum Instruction {
    LoadImmediate(LoadImmediateInstruction),
    LoadFile(LoadFileInstruction),
    Move(MoveInstruction),
    Semantic(SemanticInstruction),
    Heuristic(HeuristicInstruction),
    Branch(BranchInstruction),
    Output(OutputInstruction),
    Exit(ExitInstruction),
}
