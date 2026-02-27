use crate::{
    assembler::opcode::OpCode,
    processor::{
        control_unit::instruction::{
            BType, BTypeInstruction, ContextClearInstruction, ContextDropInstruction,
            ContextPopInstruction, ContextPushInstruction, ContextRestoreInstruction,
            ContextSnapshotInstruction, ExitInstruction, Instruction, LoadFileInstruction,
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
        let value = u32::from_be_bytes(*bytes);

        OpCode::try_from(value).unwrap_or_else(|error| {
            panic!(
                "Failed to decode opcode from byte code. Error: {}. Word: 0x{:08X}",
                error, value
            )
        })
    }

    fn text(memory: &Memory, registers: &Registers, pointer: usize, message: &str) -> String {
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
            "Failed to read text: reached end of data segment without null terminator. {}",
            message
        );
    }

    fn l_type(
        memory: &Memory,
        registers: &Registers,
        op_code: OpCode,
        instruction_bytes: [[u8; 4]; 4],
    ) -> Instruction {
        let destination_register = u32::from_be_bytes(instruction_bytes[1]);

        match op_code {
            OpCode::LoadString | OpCode::LoadFile => {
                let pointer = u32::from_be_bytes(instruction_bytes[2]) as usize;
                let message = format!("Failed to decode {:?} string/file", op_code);
                let text = Self::text(memory, registers, pointer, &message);
                if op_code == OpCode::LoadString {
                    Instruction::LoadString(LoadStringInstruction {
                        destination_register,
                        value: text,
                    })
                } else {
                    Instruction::LoadFile(LoadFileInstruction {
                        destination_register,
                        file_path: text,
                    })
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

    fn r_type(op_code: OpCode, instruction_bytes: [[u8; 4]; 4]) -> Instruction {
        let destination_register = u32::from_be_bytes(instruction_bytes[1]);
        let source_register_1 = u32::from_be_bytes(instruction_bytes[2]);
        let source_register_2 = u32::from_be_bytes(instruction_bytes[3]);

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
        let source_register_1 = u32::from_be_bytes(instruction_bytes[1]);
        let source_register_2 = u32::from_be_bytes(instruction_bytes[2]);
        let instruction_pointer_jump_index = u32::from_be_bytes(instruction_bytes[3]);

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

    fn zero_operand(op_code: OpCode) -> Instruction {
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
            _ => panic!(
                "Invalid opcode '{:?}' for zero-operand instruction.",
                op_code
            ),
        }
    }

    fn single_operand(op_code: OpCode, instruction_bytes: [[u8; 4]; 4]) -> Instruction {
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
            OpCode::Exit | OpCode::ContextClear | OpCode::ContextDrop => {
                Self::zero_operand(op_code)
            }
            OpCode::Out
            | OpCode::ContextSnapshot
            | OpCode::ContextRestore
            | OpCode::ContextPush
            | OpCode::ContextPop => Self::single_operand(op_code, instruction_bytes),
            OpCode::Morph
            | OpCode::Project
            | OpCode::Distill
            | OpCode::Correlate
            | OpCode::Audit
            | OpCode::Similarity => Self::r_type(op_code, instruction_bytes),
        }
    }
}
