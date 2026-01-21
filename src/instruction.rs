#[derive(Debug)]
pub enum OpCode {
    MOV,
    ADD,
    SUB,
    SIM,
    JLT,
}

impl OpCode {
    pub fn from_byte(byte: &u8) -> Result<OpCode, &'static str> {
        match byte {
            0x00 => Ok(OpCode::MOV),
            0x01 => Ok(OpCode::ADD),
            0x02 => Ok(OpCode::SUB),
            0x03 => Ok(OpCode::SIM),
            0x04 => Ok(OpCode::JLT),
            _ => Err("Invalid opcode byte."),
        }
    }
}

pub enum OperandType {
    NUMBER,
    TEXT,
    REGISTER,
}

impl OperandType {
    pub fn from_byte(byte: &u8) -> Result<OperandType, &'static str> {
        match byte {
            0x00 => Ok(OperandType::NUMBER),
            0x01 => Ok(OperandType::TEXT),
            0x02 => Ok(OperandType::REGISTER),
            _ => Err("Invalid operand type byte."),
        }
    }
}

#[derive(Debug)]
pub enum Operand {
    Number(u8),
    Text(String),
    Register(u8),
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

pub struct JumpLessThanInstruction {
    pub bytecode_jump_index: u8,
    pub first_operand: Operand,
    pub second_operand: Operand,
}

pub enum Instruction {
    Move(MoveInstruction),
    Add(AddInstruction),
    Sub(SubInstruction),
    Similarity(SimilarityInstruction),
    JumpLessThan(JumpLessThanInstruction),
}