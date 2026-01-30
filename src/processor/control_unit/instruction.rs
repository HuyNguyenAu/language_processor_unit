use crate::assembler::operand::Operand;

#[derive(Debug)]
pub enum ComparisonType {
    LessThan,
    LessThanOrEqual,
    GreaterThan,
    GreaterThanOrEqual,
    Equal,
}

pub struct MoveInstruction {
    pub destination_register: u8,
    pub value: Operand,
}

pub struct AddInstruction {
    pub destination_register: u8,
    pub first_operand: Operand,
    pub second_operand: Operand,
}

pub struct SubInstruction {
    pub destination_register: u8,
    pub first_operand: Operand,
    pub second_operand: Operand,
}

pub struct SimilarityInstruction {
    pub destination_register: u8,
    pub first_operand: Operand,
    pub second_operand: Operand,
}

pub struct JumpCompareInstruction {
    pub comparison_type: ComparisonType,
    pub bytecode_jump_index: u8,
    pub first_operand: Operand,
    pub second_operand: Operand,
}

pub struct OutputInstruction {
    pub source_operand: Operand,
}

pub struct LoadInstruction {
    pub destination_register: u8,
    pub file_path: String,
}

pub struct TextToSpeechInstruction {
    pub source_operand: Operand,
    pub destination_register: u8,
}

pub enum Instruction {
    Move(MoveInstruction),
    Add(AddInstruction),
    Sub(SubInstruction),
    Similarity(SimilarityInstruction),
    JumpCompare(JumpCompareInstruction),
    Output(OutputInstruction),
    Load(LoadInstruction),
    TextToSpeech(TextToSpeechInstruction),
}
