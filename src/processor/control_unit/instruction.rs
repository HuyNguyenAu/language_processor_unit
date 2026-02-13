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
pub struct AddInstruction {
    pub destination_register: u32,
    pub source_register_1: u32,
    pub source_register_2: u32,
}

#[derive(Debug)]
pub struct SubInstruction {
    pub destination_register: u32,
    pub source_register_1: u32,
    pub source_register_2: u32,
}

#[derive(Debug)]
pub struct MulInstruction {
    pub destination_register: u32,
    pub source_register_1: u32,
    pub source_register_2: u32,
}

#[derive(Debug)]
pub struct DivInstruction {
    pub destination_register: u32,
    pub source_register_1: u32,
    pub source_register_2: u32,
}

#[derive(Debug)]
pub struct SimilarityInstruction {
    pub destination_register: u32,
    pub source_register_1: u32,
    pub source_register_2: u32,
}

#[derive(Debug)]
pub enum BranchType {
    LessThan,
    LessThanOrEqual,
    GreaterThan,
    GreaterThanOrEqual,
    Equal,
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
pub enum Instruction {
    LoadImmediate(LoadImmediateInstruction),
    LoadFile(LoadFileInstruction),
    Move(MoveInstruction),
    Add(AddInstruction),
    Sub(SubInstruction),
    Mul(MulInstruction),
    Div(DivInstruction),
    Similarity(SimilarityInstruction),
    Branch(BranchInstruction),
    Output(OutputInstruction),
}
