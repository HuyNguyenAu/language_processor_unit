pub enum OpCode {
    MOV,
    ADD,
    SUB,
    SIM,
    JLT,
}

pub enum OperandType {
    NUMBER,
    TEXT,
    REGISTER,
}

pub enum Operand {
    Number(u8),
    Text(String),
    Register(u8),
}

pub struct Instruction {
    pub op_code: OpCode,
    pub operand_1: Option<Operand>,
    pub operand_2: Option<Operand>,
    pub operand_3: Option<Operand>,
}
