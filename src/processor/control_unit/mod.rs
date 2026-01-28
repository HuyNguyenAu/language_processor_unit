use std::fs::read_to_string;

use crate::{
    assembler::{
        opcode::OpCode,
        operand::{Operand, OperandType},
    },
    processor::control_unit::{
        instruction::{
            AddInstruction, ComparisonType, Instruction, JumpCompareInstruction, LoadInstruction,
            MoveInstruction, OutputInstruction, SimilarityInstruction, SubInstruction,
        },
        memory_unit::MemoryUnit,
        registers::Registers,
        semantic_logic_unit::SemanticLogicUnit,
    },
};

mod instruction;
mod memory_unit;
mod registers;
mod semantic_logic_unit;

pub struct ControlUnit {
    memory: MemoryUnit,
    registers: Registers,
    semantic_logic_unit: SemanticLogicUnit,

    previous_bytecode: Option<u8>,
    current_bytecode: Option<u8>,
}

impl ControlUnit {
    pub fn new() -> Self {
        ControlUnit {
            memory: MemoryUnit::new(),
            registers: Registers::new(),
            semantic_logic_unit: SemanticLogicUnit::new(),
            previous_bytecode: None,
            current_bytecode: None,
        }
    }

    pub fn load_bytecode(&mut self, bytecode: Vec<u8>) {
        self.memory.load(bytecode);
    }

    fn is_at_end(&self) -> bool {
        return self.registers.get_instruction_pointer() as usize >= self.memory.data_length() - 1;
    }

    fn advance(&mut self) {
        self.registers.advance_instruction_pointer();

        self.previous_bytecode = self.current_bytecode;

        let current_bytecode = self
            .memory
            .read_byte(&self.registers.get_instruction_pointer());
        self.current_bytecode = Some(*current_bytecode);
    }

    fn decode_operand_type(&mut self, message: &str) -> OperandType {
        let operand_byte = match self.current_bytecode {
            Some(bytecode) => bytecode,
            None => panic!("No current bytecode to determine operand type."),
        };

        // Consume operand type bytecode.
        self.advance();

        let operand_type = match OperandType::from_bytecode(&operand_byte) {
            Ok(operand_type) => operand_type,
            Err(error) => panic!("{} {} Byte: {:02X}", message, error, operand_byte),
        };

        return operand_type;
    }

    fn decode_text(&mut self, message: &str) -> String {
        let mut text_length: usize = 0;

        if let Some(length_byte) = self.current_bytecode {
            // Consume text length bytecode.
            self.advance();

            text_length = length_byte as usize;
        }

        let mut text_bytes: Vec<u8> = Vec::new();

        while text_bytes.len() < text_length
            && let Some(text_byte) = self.current_bytecode
        {
            if !self.is_at_end() {
                // Consume text bytecode.
                self.advance();
            }

            text_bytes.push(text_byte);
        }

        if let Ok(text) = String::from_utf8(text_bytes) {
            return text;
        }

        panic!("{}", message);
    }

    fn decode_register(&mut self, length_byte: bool, message: &str) -> u8 {
        // Consume register length bytecode if needed.
        if length_byte {
            self.advance();
        }

        let register_byte = match self.current_bytecode {
            Some(bytecode) => bytecode,
            None => panic!("{}", message),
        };

        if !self.is_at_end() {
            // Consume register bytecode.
            self.advance();
        }

        return register_byte;
    }

    fn decode_number(&mut self, length_byte: bool, message: &str) -> u8 {
        // Consume number length bytecode if needed.
        if length_byte {
            self.advance();
        }

        let number_byte = match self.current_bytecode {
            Some(bytecode) => bytecode,
            None => panic!("{}", message),
        };

        if !self.is_at_end() {
            // Consume number bytecode.
            self.advance();
        }

        return number_byte;
    }

    fn decode_operand(
        &mut self,
        operand_type_message: &str,
        operand_number: &str,
        operand_text: &str,
        operand_register: &str,
    ) -> Operand {
        let operand_type = self.decode_operand_type(operand_type_message);

        return match operand_type {
            OperandType::NUMBER => Operand::Number(self.decode_number(true, operand_number)),
            OperandType::TEXT => Operand::Text(self.decode_text(operand_text)),
            OperandType::REGISTER => {
                Operand::Register(self.decode_register(true, operand_register))
            }
        };
    }

    fn decode_move(&mut self) -> MoveInstruction {
        // Consume MOV opcode.
        self.advance();

        // Consume the destination register.
        let destination_register = self.decode_register(
            false,
            "Failed to read destination register for MOV instruction.",
        );

        // Consume value operand.
        let value = self.decode_operand(
            "Failed to determine operand type for MOV instruction.",
            "Failed to read number for MOV instruction.",
            "Failed to read text for MOV instruction.",
            "Failed to read source register for MOV instruction.",
        );

        return MoveInstruction {
            destination_register,
            value,
        };
    }

    fn decode_subtract(&mut self) -> SubInstruction {
        // Consume SUB opcode.
        self.advance();

        // Consume the first operand.
        let first_operand = self.decode_operand(
            "Failed to determine first operand type for SUB instruction.",
            "Failed to read first number operand for SUB instruction.",
            "Failed to read first text operand for SUB instruction.",
            "Failed to read first register operand for SUB instruction.",
        );

        // Consume the second operand.
        let second_operand = self.decode_operand(
            "Failed to determine second operand type for SUB instruction.",
            "Failed to read second number operand for SUB instruction.",
            "Failed to read second text operand for SUB instruction.",
            "Failed to read second register operand for SUB instruction.",
        );

        // Consume the destination register.
        let destination_register = self.decode_register(
            false,
            "Failed to read destination register for SUB instruction.",
        );

        return SubInstruction {
            destination_register,
            first_operand,
            second_operand,
        };
    }

    fn decode_addition(&mut self) -> AddInstruction {
        // Consume ADD opcode.
        self.advance();

        // Consume the first operand.
        let first_operand = self.decode_operand(
            "Failed to determine first operand type for ADD instruction.",
            "Failed to read first number operand for ADD instruction.",
            "Failed to read first text operand for ADD instruction.",
            "Failed to read first register operand for ADD instruction.",
        );

        // Consume the second operand.
        let second_operand = self.decode_operand(
            "Failed to determine second operand type for ADD instruction.",
            "Failed to read second number operand for ADD instruction.",
            "Failed to read second text operand for ADD instruction.",
            "Failed to read second register operand for ADD instruction.",
        );

        // Consume the destination register.
        let destination_register = self.decode_register(
            false,
            "Failed to read destination register for ADD instruction.",
        );

        return AddInstruction {
            destination_register,
            first_operand,
            second_operand,
        };
    }

    fn decode_similarity(&mut self) -> SimilarityInstruction {
        // Consume SIM opcode.
        self.advance();

        // Consume the first operand.
        let first_operand = self.decode_operand(
            "Failed to determine first operand type for SIM instruction.",
            "Failed to read first number operand for SIM instruction.",
            "Failed to read first text operand for SIM instruction.",
            "Failed to read first register operand for SIM instruction.",
        );

        // Consume the second operand.
        let second_operand = self.decode_operand(
            "Failed to determine second operand type for SIM instruction.",
            "Failed to read second number operand for SIM instruction.",
            "Failed to read second text operand for SIM instruction.",
            "Failed to read second register operand for SIM instruction.",
        );

        // Consume the destination register.
        let destination_register = self.decode_register(
            false,
            "Failed to read destination register for SIM instruction.",
        );

        return SimilarityInstruction {
            destination_register,
            first_operand,
            second_operand,
        };
    }

    fn decode_jump_compare(&mut self, op_code: OpCode) -> JumpCompareInstruction {
        // Consume jump comparison opcode.
        self.advance();

        // Consume the first operand.
        let first_operand = self.decode_operand(
            &format!(
                "Failed to determine first operand type for {:?} instruction.",
                op_code
            ),
            &format!(
                "Failed to read first number operand for {:?} instruction.",
                op_code
            ),
            &format!(
                "Failed to read first text operand for {:?} instruction.",
                op_code
            ),
            &format!(
                "Failed to read first register operand for {:?} instruction.",
                op_code
            ),
        );

        // Consume the second operand.
        let second_operand = self.decode_operand(
            &format!(
                "Failed to determine second operand type for {:?} instruction.",
                op_code
            ),
            &format!(
                "Failed to read second number operand for {:?} instruction.",
                op_code
            ),
            &format!(
                "Failed to read second text operand for {:?} instruction.",
                op_code
            ),
            &format!(
                "Failed to read second register operand for {:?} instruction.",
                op_code
            ),
        );

        // Consume the bytecode jump index.
        let bytecode_jump_index = self.decode_number(
            false,
            format!(
                "Failed to read bytecode code jump index for {:?} instruction.",
                op_code
            )
            .as_str(),
        );
        let comparison_type = match op_code {
            OpCode::JEQ => ComparisonType::Equal,
            OpCode::JLT => ComparisonType::LessThan,
            OpCode::JLE => ComparisonType::LessThanOrEqual,
            OpCode::JGT => ComparisonType::GreaterThan,
            OpCode::JGE => ComparisonType::GreaterThanOrEqual,
            _ => panic!("Invalid opcode for jump compare instruction."),
        };

        return JumpCompareInstruction {
            comparison_type,
            bytecode_jump_index,
            first_operand,
            second_operand,
        };
    }

    fn decode_output(&mut self) -> OutputInstruction {
        // Consume OUT opcode.
        self.advance();

        // Consume the source operand.
        let source_operand = self.decode_operand(
            "Failed to determine source operand type for OUT instruction.",
            "Failed to read source number operand for OUT instruction.",
            "Failed to read source text operand for OUT instruction.",
            "Failed to read source register operand for OUT instruction.",
        );

        return OutputInstruction { source_operand };
    }

    fn decode_load(&mut self) -> LoadInstruction {
        // Consume LOAD opcode.
        self.advance();

        // Consume the destination register.
        let destination_register = self.decode_register(
            false,
            "Failed to read destination register for LOAD instruction.",
        );

        // Consume file path operand.
        let file_path = match self.decode_operand(
            "Failed to determine operand type for LOAD instruction.",
            "Failed to read number for LOAD instruction.",
            "Failed to read text for LOAD instruction.",
            "Failed to read source register for LOAD instruction.",
        ) {
            Operand::Text(text) => text,
            _ => panic!("LOAD instruction requires a text operand for the file path."),
        };

        return LoadInstruction {
            destination_register,
            file_path,
        };
    }

    fn decode_op_code(&mut self) -> Instruction {
        let current_bytecode = match self.current_bytecode {
            Some(bytecode) => bytecode,
            None => panic!("No current bytecode to determine opcode."),
        };
        let op_code = match OpCode::from_byte(&current_bytecode) {
            Ok(op_code) => op_code,
            Err(error) => panic!("{} Byte: {:02X}", error, current_bytecode),
        };

        return match op_code {
            OpCode::MOV => Instruction::Move(self.decode_move()),
            OpCode::ADD => Instruction::Add(self.decode_addition()),
            OpCode::SUB => Instruction::Sub(self.decode_subtract()),
            OpCode::SIM => Instruction::Similarity(self.decode_similarity()),
            OpCode::JEQ => Instruction::JumpCompare(self.decode_jump_compare(op_code)),
            OpCode::JLT => Instruction::JumpCompare(self.decode_jump_compare(op_code)),
            OpCode::JLE => Instruction::JumpCompare(self.decode_jump_compare(op_code)),
            OpCode::JGT => Instruction::JumpCompare(self.decode_jump_compare(op_code)),
            OpCode::JGE => Instruction::JumpCompare(self.decode_jump_compare(op_code)),
            OpCode::OUT => Instruction::Output(self.decode_output()),
            OpCode::LOAD => Instruction::Load(self.decode_load()),
        };
    }

    pub fn fetch_and_decode(&mut self) -> Option<Instruction> {
        // Initialise current bytecode.
        let current_bytecode = self
            .memory
            .read_byte(&self.registers.get_instruction_pointer());
        self.current_bytecode = Some(*current_bytecode);

        if self.is_at_end() {
            return None;
        }

        return Some(self.decode_op_code());
    }

    fn get_value(&self, operand: &Operand) -> String {
        return match operand {
            Operand::Number(number) => number.to_string(),
            Operand::Text(text) => text.clone(),
            Operand::Register(register_number) => {
                self.registers.get_register(register_number).to_string()
            }
        };
    }

    fn execute_move(&mut self, instruction: &MoveInstruction) {
        let value = self.get_value(&instruction.value);

        self.registers
            .set_register(&instruction.destination_register, &value);

        println!(
            "Executed MOV: r{} = \"{}\"",
            instruction.destination_register,
            self.registers
                .get_register(&instruction.destination_register)
        );
    }

    fn execute_add(&mut self, instruction: &AddInstruction) {
        let first_operand_value = self.get_value(&instruction.first_operand);
        let second_operand_value = self.get_value(&instruction.second_operand);

        let result = self
            .semantic_logic_unit
            .addition(&first_operand_value, &second_operand_value);

        self.registers
            .set_register(&instruction.destination_register, &result);

        println!(
            "Executed ADD: {:?} + {:?} -> r{} = \"{}\"",
            first_operand_value,
            second_operand_value,
            instruction.destination_register,
            self.registers
                .get_register(&instruction.destination_register)
        );
    }

    fn execute_subtract(&mut self, instruction: &SubInstruction) {
        let first_operand_value = self.get_value(&instruction.first_operand);
        let second_operand_value = self.get_value(&instruction.second_operand);

        let result = self
            .semantic_logic_unit
            .subtract(&first_operand_value, &second_operand_value);

        self.registers
            .set_register(&instruction.destination_register, &result);

        println!(
            "Executed SUB: {:?} - {:?} -> r{} = \"{}\"",
            first_operand_value,
            second_operand_value,
            instruction.destination_register,
            self.registers
                .get_register(&instruction.destination_register)
        );
    }

    fn execute_similarity(&mut self, instruction: &SimilarityInstruction) {
        let first_operand_value = self.get_value(&instruction.first_operand);
        let second_operand_value = self.get_value(&instruction.second_operand);

        let result = self
            .semantic_logic_unit
            .similarity(&first_operand_value, &second_operand_value);

        self.registers
            .set_register(&instruction.destination_register, &result);

        println!(
            "Executed SIM: {:?} ~ {:?} -> r{} = \"{}\"",
            first_operand_value,
            second_operand_value,
            instruction.destination_register,
            self.registers
                .get_register(&instruction.destination_register)
        );
    }

    fn execute_jump_compare(&mut self, instruction: &JumpCompareInstruction) {
        let first_operand_value = match self.get_value(&instruction.first_operand).parse::<u8>() {
            Ok(value) => value,
            _ => panic!(
                "{:?} instruction requires numeric operands.",
                instruction.comparison_type
            ),
        };
        let second_operand_value = match self.get_value(&instruction.second_operand).parse::<u8>() {
            Ok(value) => value,
            _ => panic!(
                "{:?} instruction requires numeric operands.",
                instruction.comparison_type
            ),
        };
        let address = instruction.bytecode_jump_index.clone();

        match instruction.comparison_type {
            ComparisonType::Equal => {
                if first_operand_value == second_operand_value {
                    self.registers.set_instruction_pointer(&address);
                }

                println!(
                    "Executed JEQ: {:?} == {:?} -> {}",
                    first_operand_value, second_operand_value, instruction.bytecode_jump_index
                );
            }
            ComparisonType::LessThan => {
                if first_operand_value < second_operand_value {
                    self.registers.set_instruction_pointer(&address);
                }

                println!(
                    "Executed JLT: {:?} < {:?} -> {}",
                    first_operand_value, second_operand_value, instruction.bytecode_jump_index
                );
            }
            ComparisonType::LessThanOrEqual => {
                if first_operand_value <= second_operand_value {
                    self.registers.set_instruction_pointer(&address);
                }

                println!(
                    "Executed JLE: {:?} <= {:?} -> {}",
                    first_operand_value, second_operand_value, instruction.bytecode_jump_index
                );
            }
            ComparisonType::GreaterThan => {
                if first_operand_value > second_operand_value {
                    self.registers.set_instruction_pointer(&address);
                }

                println!(
                    "Executed JGT: {:?} > {:?} -> {}",
                    first_operand_value, second_operand_value, instruction.bytecode_jump_index
                );
            }
            ComparisonType::GreaterThanOrEqual => {
                if first_operand_value >= second_operand_value {
                    self.registers.set_instruction_pointer(&address);
                }

                println!(
                    "Executed JGE: {:?} >= {:?} -> {}",
                    first_operand_value, second_operand_value, instruction.bytecode_jump_index
                );
            }
        }
    }

    fn execute_output(&mut self, instruction: &OutputInstruction) {
        let source_operand_value = self.get_value(&instruction.source_operand);

        println!("Executed OUT: {}", source_operand_value);
    }

    fn execute_load(&mut self, instruction: &LoadInstruction) {
        let file_contents = match read_to_string(&instruction.file_path) {
            Ok(value) => value,
            Err(error) => panic!("Run failed. Error: {}", error),
        };

        self.registers
            .set_register(&instruction.destination_register, &file_contents);

        println!(
            "Executed LOAD: r{} = \"{}\"",
            instruction.destination_register,
            self.registers
                .get_register(&instruction.destination_register)
        );
    }

    pub fn execute(&mut self, instruction: &Instruction) {
        match instruction {
            Instruction::Move(instruction) => self.execute_move(instruction),
            Instruction::Add(instruction) => self.execute_add(instruction),
            Instruction::Sub(instruction) => self.execute_subtract(instruction),
            Instruction::Similarity(instruction) => self.execute_similarity(instruction),
            Instruction::JumpCompare(instruction) => self.execute_jump_compare(instruction),
            Instruction::Output(instruction) => self.execute_output(instruction),
            Instruction::Load(instruction) => self.execute_load(instruction),
        }
    }
}
