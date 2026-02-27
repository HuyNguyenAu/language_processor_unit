use crate::{
    assembler::opcode::OpCode,
    processor::{
        control_unit::instruction::{
            AuditInstruction, BranchInstruction, BranchType, ContextClearInstruction,
            ContextDropInstruction, ContextPopInstruction, ContextPushInstruction,
            ContextRestoreInstruction, ContextSetRoleInstruction, ContextSnapshotInstruction,
            CorrelateInstruction, DistillInstruction, ExitInstruction, Instruction,
            LoadFileInstruction, LoadImmediateInstruction, LoadStringInstruction, MorphInstruction,
            MoveInstruction, OutputInstruction, ProjectInstruction, SimilarityInstruction,
        },
        memory::Memory,
        registers::Registers,
    },
};

pub struct Decoder;

impl Decoder {
    fn op_code(bytes: &[u8; 4]) -> OpCode {
        let value = u32::from_be_bytes(*bytes);

        OpCode::try_from(value).unwrap_or_else(|error| {
            panic!(
                "Failed to decode opcode from byte code. Error: {}. Word: 0x{:08X}",
                error, value
            )
        })
    }

    fn string(memory: &Memory, registers: &Registers, pointer: usize, message: &str) -> String {
        let mut bytes = Vec::new();
        let mut address = pointer + registers.get_data_section_pointer();

        while let Ok(word) = memory.read(address) {
            let value: u8 = u32::from_be_bytes(*word)
                .try_into()
                .expect("Failed to convert word to byte");

            if value == 0 {
                return String::from_utf8(bytes).expect("Failed to decode string bytes");
            }

            bytes.push(value);
            address += 1;
        }

        panic!(
            "Failed to read string: reached end of data segment without null terminator. {}",
            message
        );
    }

    fn immediate(
        memory: &Memory,
        registers: &Registers,
        op_code: OpCode,
        instruction_bytes: [[u8; 4]; 4],
    ) -> Instruction {
        let destination_register = u32::from_be_bytes(instruction_bytes[1]);

        match op_code {
            OpCode::LoadString | OpCode::LoadFile => {
                let pointer = u32::from_be_bytes(instruction_bytes[2]) as usize;
                let string = Self::string(
                    memory,
                    registers,
                    pointer,
                    &format!("Failed to decode {:?} string", op_code),
                );

                match op_code {
                    OpCode::LoadString => Instruction::LoadString(LoadStringInstruction {
                        destination_register,
                        value: string,
                    }),
                    OpCode::LoadFile => Instruction::LoadFile(LoadFileInstruction {
                        destination_register,
                        file_path: string,
                    }),
                    _ => panic!(
                        "Invalid opcode '{:?}' for string-loading instruction.",
                        op_code
                    ),
                }
            }
            OpCode::LoadImmediate => Instruction::LoadImmediate(LoadImmediateInstruction {
                destination_register,
                value: u32::from_be_bytes(instruction_bytes[2]),
            }),
            OpCode::Move => Instruction::Move(MoveInstruction {
                destination_register,
                source_register: u32::from_be_bytes(instruction_bytes[2]),
            }),
            _ => panic!("Invalid opcode '{:?}' for L-type instruction.", op_code),
        }
    }

    fn branch(op_code: OpCode, instruction_bytes: [[u8; 4]; 4]) -> Instruction {
        let source_register_1 = u32::from_be_bytes(instruction_bytes[1]);
        let source_register_2 = u32::from_be_bytes(instruction_bytes[2]);
        let instruction_pointer_jump_index = u32::from_be_bytes(instruction_bytes[3]);

        let branch_type = match op_code {
            OpCode::BranchEqual => BranchType::Equal,
            OpCode::BranchLess => BranchType::Less,
            OpCode::BranchLessEqual => BranchType::LessEqual,
            OpCode::BranchGreater => BranchType::Greater,
            OpCode::BranchGreaterEqual => BranchType::GreaterEqual,
            _ => panic!("Invalid opcode '{:?}' for branch instruction.", op_code),
        };

        Instruction::Branch(BranchInstruction {
            branch_type,
            source_register_1,
            source_register_2,
            instruction_pointer_jump_index,
        })
    }

    fn no_register(op_code: OpCode) -> Instruction {
        match op_code {
            // Control flow.
            OpCode::Exit => Instruction::Exit(ExitInstruction),
            // Context operations.
            OpCode::ContextClear => Instruction::ContextClear(ContextClearInstruction),
            OpCode::ContextDrop => Instruction::ContextDrop(ContextDropInstruction),
            _ => panic!(
                "Invalid opcode '{:?}' for zero-operand instruction.",
                op_code
            ),
        }
    }

    fn no_register_string(
        memory: &Memory,
        registers: &Registers,
        op_code: OpCode,
        instruction_bytes: [[u8; 4]; 4],
    ) -> Instruction {
        let pointer = u32::from_be_bytes(instruction_bytes[1]) as usize;
        let string = Self::string(
            memory,
            registers,
            pointer,
            &format!("Failed to decode {:?} string", op_code),
        );

        match op_code {
            OpCode::ContextSetRole => {
                Instruction::ContextSetRole(ContextSetRoleInstruction { role: string })
            }
            _ => panic!(
                "Invalid opcode '{:?}' for zero-register string instruction.",
                op_code
            ),
        }
    }

    fn single_register(op_code: OpCode, instruction_bytes: [[u8; 4]; 4]) -> Instruction {
        let register = u32::from_be_bytes(instruction_bytes[1]);

        match op_code {
            // I/O.
            OpCode::Out => Instruction::Output(OutputInstruction {
                source_register: register,
            }),
            // Context operations.
            OpCode::ContextSnapshot => Instruction::ContextSnapshot(ContextSnapshotInstruction {
                destination_register: register,
            }),
            OpCode::ContextRestore => Instruction::ContextRestore(ContextRestoreInstruction {
                source_register: register,
            }),
            OpCode::ContextPush => Instruction::ContextPush(ContextPushInstruction {
                source_register: register,
            }),
            OpCode::ContextPop => Instruction::ContextPop(ContextPopInstruction {
                destination_register: register,
            }),
            _ => panic!(
                "Invalid opcode '{:?}' for single-operand instruction.",
                op_code
            ),
        }
    }

    fn double_register(op_code: OpCode, instruction_bytes: [[u8; 4]; 4]) -> Instruction {
        let destination_register = u32::from_be_bytes(instruction_bytes[1]);
        let source_register = u32::from_be_bytes(instruction_bytes[2]);

        match op_code {
            OpCode::Morph => Instruction::Morph(MorphInstruction {
                destination_register,
                source_register,
            }),
            OpCode::Project => Instruction::Project(ProjectInstruction {
                destination_register,
                source_register,
            }),
            OpCode::Distill => Instruction::Distill(DistillInstruction {
                destination_register,
                source_register,
            }),
            OpCode::Correlate => Instruction::Correlate(CorrelateInstruction {
                destination_register,
                source_register,
            }),
            OpCode::Audit => Instruction::Audit(AuditInstruction {
                destination_register,
                source_register,
            }),
            _ => panic!(
                "Invalid opcode '{:?}' for double-register instruction.",
                op_code
            ),
        }
    }

    fn triple_register(op_code: OpCode, instruction_bytes: [[u8; 4]; 4]) -> Instruction {
        let destination_register = u32::from_be_bytes(instruction_bytes[1]);
        let source_register_1 = u32::from_be_bytes(instruction_bytes[2]);
        let source_register_2 = u32::from_be_bytes(instruction_bytes[3]);

        match op_code {
            OpCode::Similarity => Instruction::Similarity(SimilarityInstruction {
                destination_register,
                source_register_1,
                source_register_2,
            }),
            _ => panic!(
                "Invalid opcode '{:?}' for triple-register instruction.",
                op_code
            ),
        }
    }

    pub fn decode(
        memory: &Memory,
        registers: &Registers,
        instruction_bytes: [[u8; 4]; 4],
    ) -> Instruction {
        let op_code = Self::op_code(&instruction_bytes[0]);

        match op_code {
            OpCode::LoadString | OpCode::LoadImmediate | OpCode::LoadFile | OpCode::Move => {
                Self::immediate(memory, registers, op_code, instruction_bytes)
            }
            OpCode::BranchEqual
            | OpCode::BranchLess
            | OpCode::BranchLessEqual
            | OpCode::BranchGreater
            | OpCode::BranchGreaterEqual => Self::branch(op_code, instruction_bytes),
            OpCode::Exit | OpCode::ContextClear | OpCode::ContextDrop => Self::no_register(op_code),
            OpCode::Out
            | OpCode::ContextSnapshot
            | OpCode::ContextRestore
            | OpCode::ContextPush
            | OpCode::ContextPop => Self::single_register(op_code, instruction_bytes),
            OpCode::ContextSetRole => {
                Self::no_register_string(memory, registers, op_code, instruction_bytes)
            }
            OpCode::Morph
            | OpCode::Project
            | OpCode::Distill
            | OpCode::Correlate
            | OpCode::Audit => Self::double_register(op_code, instruction_bytes),
            OpCode::Similarity => Self::triple_register(op_code, instruction_bytes),
            OpCode::NoOp => panic!("NoOp is not a valid instruction and should not be decoded."),
        }
    }
}
