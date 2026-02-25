use crate::{
    assembler::opcode::OpCode,
    processor::{
        control_unit::instruction::{
            BType, BTypeInstruction, ExitInstruction, Instruction, LoadFileInstruction,
            LoadImmediateInstruction, LoadStringInstruction, MoveInstruction, OutputInstruction,
            RType, RTypeInstruction,
        },
        memory::Memory,
        registers::Registers,
    },
};

pub struct Decoder;

impl Decoder {
    fn op_code(bytes: &[u8; 4]) -> OpCode {
        OpCode::from_be_bytes(*bytes).unwrap_or_else(|error| {
            panic!(
                "Failed to decode opcode from byte code. Error: {}. Byte code: {:?}.",
                error, bytes
            )
        })
    }

    fn number(bytes: &[u8; 4]) -> u32 {
        u32::from_be_bytes(*bytes)
    }

    fn text(memory: &Memory, registers: &Registers, pointer: usize, message: &str) -> String {
        let mut text_bytes = Vec::new();
        let mut address = pointer + registers.get_data_section_pointer();

        while address < memory.length() {
            let data_byte = memory.read(address).unwrap_or_else(|err| {
                panic!(
                    "Failed to read text from memory at address {}: {}. Message: {}.",
                    address, err, message
                )
            });
            let text_byte: u8 = u32::from_be_bytes(*data_byte)
                .try_into()
                .unwrap_or_else(|err| {
                    panic!(
                        "Failed to decode text byte from memory at address {}: {}. Message: {}.",
                        address, err, message
                    )
                });

            if data_byte == &[0, 0, 0, 0] {
                let s = String::from_utf8(text_bytes)
                    .unwrap_or_else(|err| panic!(
                        "Failed to read text from byte code. Error: {}. Byte code: {:?}. Message: {}.",
                        err, data_byte, message
                    ));
                return s.trim_matches(char::from(0)).to_string();
            }

            text_bytes.push(text_byte);
            address += 1;
        }

        panic!(
            "Failed to read text. Reached end of data bytes without encountering null terminator. Error: {}",
            message
        );
    }

    fn l_type(
        memory: &Memory,
        registers: &Registers,
        op_code: OpCode,
        instruction_bytes: [[u8; 4]; 4],
    ) -> Instruction {
        let destination_register = Self::number(&instruction_bytes[1]);

        match op_code {
            OpCode::LoadString => {
                let pointer: usize = Self::number(&instruction_bytes[2])
                    .try_into()
                    .unwrap_or_else(|err| panic!(
                        "Failed to decode pointer for {:?} instruction. Error: {}. Byte code: {:?}.",
                        op_code, err, instruction_bytes[2]
                    ));
                let value = Self::text(
                    memory,
                    registers,
                    pointer,
                    &format!(
                        "Failed to decode string for {:?} instruction with pointer {}.",
                        op_code, pointer
                    ),
                );

                Instruction::LoadString(LoadStringInstruction {
                    destination_register,
                    value,
                })
            }
            OpCode::LoadImmediate => {
                let immediate_value = Self::number(&instruction_bytes[2]);
                Instruction::LoadImmediate(LoadImmediateInstruction {
                    destination_register,
                    value: immediate_value,
                })
            }
            OpCode::LoadFile => {
                let pointer: usize = Self::number(&instruction_bytes[2])
                    .try_into()
                    .unwrap_or_else(|err| panic!(
                        "Failed to decode pointer for {:?} instruction. Error: {}. Byte code: {:?}.",
                        op_code, err, instruction_bytes[2]
                    ));
                let file_path = Self::text(
                    memory,
                    registers,
                    pointer,
                    &format!(
                        "Failed to decode file path for {:?} instruction with pointer {}.",
                        op_code, pointer
                    ),
                );

                Instruction::LoadFile(LoadFileInstruction {
                    destination_register,
                    file_path,
                })
            }
            OpCode::Move => {
                let source_register = Self::number(&instruction_bytes[2]);
                Instruction::Move(MoveInstruction {
                    destination_register,
                    source_register,
                })
            }
            _ => panic!("Invalid opcode '{:?}' for L-type instruction.", op_code),
        }
    }

    fn r_type(op_code: OpCode, instruction_bytes: [[u8; 4]; 4]) -> Instruction {
        let destination_register = Self::number(&instruction_bytes[1]);
        let source_register_1 = Self::number(&instruction_bytes[2]);
        let source_register_2 = Self::number(&instruction_bytes[3]);

        let r_type = match op_code {
            OpCode::Morph => RType::Morph,
            OpCode::Project => RType::Project,
            OpCode::Distill => RType::Distill,
            OpCode::Correlate => RType::Correlate,
            OpCode::Audit => RType::Audit,
            OpCode::Similarity => RType::Similarity,
            _ => panic!("Invalid opcode '{:?}' for R-type instruction.", op_code),
        };

        Instruction::RType(RTypeInstruction {
            r_type,
            destination_register,
            source_register_1,
            source_register_2,
        })
    }

    fn b_type(op_code: OpCode, instruction_bytes: [[u8; 4]; 4]) -> Instruction {
        let source_register_1 = Self::number(&instruction_bytes[1]);
        let source_register_2 = Self::number(&instruction_bytes[2]);
        let instruction_pointer_jump_index = Self::number(&instruction_bytes[3]);

        let b_type = match op_code {
            OpCode::BranchEqual => BType::Equal,
            OpCode::BranchLess => BType::Less,
            OpCode::BranchLessEqual => BType::LessEqual,
            OpCode::BranchGreater => BType::Greater,
            OpCode::BranchGreaterEqual => BType::GreaterEqual,
            _ => panic!("Invalid opcode '{:?}' for branch instruction.", op_code),
        };

        Instruction::BType(BTypeInstruction {
            b_type,
            source_register_1,
            source_register_2,
            instruction_pointer_jump_index,
        })
    }

    fn output(instruction_bytes: [[u8; 4]; 4]) -> Instruction {
        let source_register = Self::number(&instruction_bytes[1]);
        
        Instruction::Output(OutputInstruction { source_register })
    }

    fn exit() -> Instruction {
        Instruction::Exit(ExitInstruction)
    }

    pub fn decode(
        memory: &Memory,
        registers: &Registers,
        instruction_bytes: [[u8; 4]; 4],
    ) -> Instruction {
        let op_code = Self::op_code(&instruction_bytes[0]);

        match op_code {
            OpCode::LoadString | OpCode::LoadImmediate | OpCode::LoadFile | OpCode::Move => {
                Self::l_type(memory, registers, op_code, instruction_bytes)
            }
            OpCode::BranchEqual
            | OpCode::BranchLess
            | OpCode::BranchLessEqual
            | OpCode::BranchGreater
            | OpCode::BranchGreaterEqual => Self::b_type(op_code, instruction_bytes),
            OpCode::Exit => Self::exit(),
            OpCode::Out => Self::output(instruction_bytes),
            OpCode::Morph
            | OpCode::Project
            | OpCode::Distill
            | OpCode::Correlate
            | OpCode::Audit
            | OpCode::Similarity => Self::r_type(op_code, instruction_bytes),
        }
    }
}
