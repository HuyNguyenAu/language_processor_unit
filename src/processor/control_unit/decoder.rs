use std::sync::{Arc, Mutex, MutexGuard};

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

pub struct Decoder {
    memory: Arc<Mutex<Memory>>,
    registers: Arc<Mutex<Registers>>,
}

impl Decoder {
    pub fn new(memory: &Arc<Mutex<Memory>>, registers: &Arc<Mutex<Registers>>) -> Self {
        Decoder {
            memory: Arc::clone(memory),
            registers: Arc::clone(registers),
        }
    }

    fn memory_lock(&self) -> MutexGuard<'_, Memory> {
        match self.memory.lock() {
            Ok(memory) => memory,
            Err(error) => {
                panic!("Failed to access memory: memory lock error: {}", error);
            }
        }
    }

    fn registers_lock(&self) -> MutexGuard<'_, Registers> {
        match self.registers.lock() {
            Ok(registers) => registers,
            Err(error) => {
                panic!(
                    "Failed to access registers: registers lock error: {}",
                    error
                );
            }
        }
    }

    fn op_code(&mut self, bytes: &[u8; 4]) -> OpCode {
        match OpCode::from_be_bytes(*bytes) {
            Ok(op_code) => op_code,
            Err(error) => panic!(
                "Failed to decode opcode from byte code. Error: {}. Byte code: {:?}.",
                error, bytes
            ),
        }
    }

    fn number(&mut self, bytes: &[u8; 4]) -> u32 {
        u32::from_be_bytes(*bytes)
    }

    fn text(&mut self, pointer: usize, message: &str) -> String {
        let memory = self.memory_lock();
        let registers = self.registers_lock();

        let mut text_bytes: Vec<u8> = Vec::new();

        let memory_length = memory.length();
        let mut address = pointer + registers.get_data_section_pointer();

        while address < memory_length {
            let data_byte = match memory.read(address) {
                Ok(bytes) => bytes,
                Err(error) => panic!(
                    "Failed to read text from memory at address {}: {}. Message: {}.",
                    address, error, message
                ),
            };
            let text_byte: u8 = match u32::from_be_bytes(*data_byte).try_into() {
                Ok(byte) => byte,
                Err(error) => panic!(
                    "Failed to decode text byte from memory at address {}: {}. Message: {}.",
                    address, error, message
                ),
            };

            text_bytes.push(text_byte);

            // Check for null terminator.
            if data_byte == &[0, 0, 0, 0] {
                println!("Read text bytes: {:?}.", text_bytes);
                return match String::from_utf8(text_bytes) {
                    Ok(text) => text.trim_matches(char::from(0)).to_string(),
                    Err(error) => panic!(
                        "Failed to read text from byte code. Error: {}. Byte code: {:?}. Message: {}.",
                        error, data_byte, message
                    ),
                };
            }

            address += 1;
        }

        panic!(
            "Failed to read text. Reached end of data bytes without encountering null terminator. Error: {}",
            message
        );
    }

    fn l_type(&mut self, op_code: OpCode, instruction_bytes: [[u8; 4]; 4]) -> Instruction {
        // Decode the destination register.
        let destination_register = self.number(&instruction_bytes[1]);

        match op_code {
            OpCode::LoadString => {
                // Decode the pointer.
                let pointer: usize = match self.number(&instruction_bytes[2]).try_into() {
                    Ok(pointer) => pointer,
                    Err(error) => panic!(
                        "Failed to decode pointer for {:?} instruction. Error: {}. Byte code: {:?}.",
                        op_code, error, instruction_bytes[2]
                    ),
                };
                let value = self.text(
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
                // Decode the immediate value.
                let immediate_value = self.number(&instruction_bytes[2]);

                Instruction::LoadImmediate(LoadImmediateInstruction {
                    destination_register,
                    value: immediate_value,
                })
            }
            OpCode::LoadFile => {
                // Decode the pointer.
                let pointer: usize = match self.number(&instruction_bytes[2]).try_into() {
                    Ok(pointer) => pointer,
                    Err(error) => panic!(
                        "Failed to decode pointer for {:?} instruction. Error: {}. Byte code: {:?}.",
                        op_code, error, instruction_bytes[2]
                    ),
                };
                let value = self.text(
                    pointer,
                    &format!(
                        "Failed to decode file path for {:?} instruction with pointer {}.",
                        op_code, pointer
                    ),
                );

                Instruction::LoadFile(LoadFileInstruction {
                    destination_register,
                    value,
                })
            }
            OpCode::Move => {
                // Decode the source register.
                let source_register = self.number(&instruction_bytes[2]);

                Instruction::Move(MoveInstruction {
                    destination_register,
                    source_register,
                })
            }
            _ => panic!("Invalid opcode '{:?}' for L-type instruction.", op_code),
        }
    }

    fn r_type(&mut self, op_code: OpCode, instruction_bytes: [[u8; 4]; 4]) -> Instruction {
        // Decode the destination register.
        let destination_register = self.number(&instruction_bytes[1]);

        // Decode source register 1.
        let source_register_1 = self.number(&instruction_bytes[2]);

        // Decode source register 2.
        let source_register_2 = self.number(&instruction_bytes[3]);

        let r_type = match op_code {
            // Generative operations.
            OpCode::Morph => RType::Morph,
            OpCode::Project => RType::Project,
            // Cognitive operations.
            OpCode::Distill => RType::Distill,
            OpCode::Correlate => RType::Correlate,
            // Guardrails operations.
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

    fn b_type(&mut self, op_code: OpCode, instruction_bytes: [[u8; 4]; 4]) -> Instruction {
        // Decode source register 1.
        let source_register_1 = self.number(&instruction_bytes[1]);

        // Decode source register 2.
        let source_register_2 = self.number(&instruction_bytes[2]);

        // Decode the instruction pointer jump index.
        let instruction_pointer_jump_index = self.number(&instruction_bytes[3]);

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

    fn output(&mut self, instruction_bytes: [[u8; 4]; 4]) -> Instruction {
        // Decode the source register.
        let source_register = self.number(&instruction_bytes[1]);

        Instruction::Output(OutputInstruction { source_register })
    }

    fn exit(&mut self) -> Instruction {
        Instruction::Exit(ExitInstruction)
    }

    pub fn decode(&mut self, instruction_bytes: [[u8; 4]; 4]) -> Instruction {
        let op_code = self.op_code(&instruction_bytes[0]);

        match op_code {
            // Data movement instructions.
            OpCode::LoadString => self.l_type(op_code, instruction_bytes),
            OpCode::LoadImmediate => self.l_type(op_code, instruction_bytes),
            OpCode::LoadFile => self.l_type(op_code, instruction_bytes),
            OpCode::Move => self.l_type(op_code, instruction_bytes),
            // Control flow instructions.
            OpCode::BranchEqual => self.b_type(op_code, instruction_bytes),
            OpCode::BranchLess => self.b_type(op_code, instruction_bytes),
            OpCode::BranchLessEqual => self.b_type(op_code, instruction_bytes),
            OpCode::BranchGreater => self.b_type(op_code, instruction_bytes),
            OpCode::BranchGreaterEqual => self.b_type(op_code, instruction_bytes),
            OpCode::Exit => self.exit(),
            // I/O instructions.
            OpCode::Out => self.output(instruction_bytes),
            // Generative, cognitive, and guardrails operations.
            OpCode::Morph
            | OpCode::Project
            | OpCode::Distill
            | OpCode::Correlate
            | OpCode::Audit
            | OpCode::Similarity => self.r_type(op_code, instruction_bytes),
        }
    }
}
