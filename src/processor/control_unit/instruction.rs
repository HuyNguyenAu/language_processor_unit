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
pub enum ArithmeticType {
    Add,
    Sub,
    Mul,
    Div,
    Similarity,
}

#[derive(Debug)]
pub struct ArithmeticInstruction {
    pub arithmetic_type: ArithmeticType,
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
    Arithmetic(ArithmeticInstruction),
    Branch(BranchInstruction),
    Output(OutputInstruction),
}
