use std::fs::read_to_string;

use crate::{
    assembler::{
        immediate::{Immediate, ImmediateType},
        opcode::OpCode,
    },
    processor::control_unit::{
        instruction::{
            AddInstruction, BranchInstruction, BranchType, DivInstruction, Instruction,
            LoadFileInstruction, LoadImmediateInstruction, MoveInstruction, MulInstruction,
            OutputInstruction, SimilarityInstruction, SubInstruction,
        },
        memory_unit::MemoryUnit,
        registers::{Registers, Value},
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

    previous_be_bytes: Option<[u8; 4]>,
    current_be_bytes: Option<[u8; 4]>,
}

impl ControlUnit {
    pub fn new() -> Self {
        ControlUnit {
            memory: MemoryUnit::new(),
            registers: Registers::new(),
            semantic_logic_unit: SemanticLogicUnit::new(),
            previous_be_bytes: None,
            current_be_bytes: None,
        }
    }

    pub fn load_byte_code(&mut self, byte_code: Vec<[u8; 4]>) {
        self.memory.load(byte_code);
    }

    fn is_at_end(&self) -> bool {
        return self.registers.get_instruction_pointer() >= self.memory.length();
    }

    fn advance(&mut self) {
        self.registers.advance_instruction_pointer();

        self.previous_be_bytes = self.current_be_bytes;

        if self.is_at_end() {
            self.current_be_bytes = None;

            return;
        }

        let bytes = self.memory.read(self.registers.get_instruction_pointer());
        self.current_be_bytes = Some(bytes.clone());
    }

    fn decode_text(&mut self, message: &str) -> String {
        let mut text_length: usize = 0;

        if let Some(length_be_bytes) = self.current_be_bytes {
            // Consume text length bytecode.
            self.advance();

            text_length = match u32::from_be_bytes(length_be_bytes).try_into() {
                Ok(length) => length,
                _ => panic!(
                    "Failed to get text length from bytecode. Text length exceeds {}. Found text length byte code: {:?}.",
                    usize::MAX,
                    length_be_bytes
                ),
            };
        }

        let mut text_bytes: Vec<u8> = Vec::new();

        while text_bytes.len() < text_length
            && let Some(be_bytes) = self.current_be_bytes
        {
            if !self.is_at_end() {
                // Consume text bytecode.
                self.advance();
            }

            let value: u8 = match u32::from_be_bytes(be_bytes).try_into() {
                Ok(value) => value,
                _ => panic!(
                    "Failed to get text byte from bytecode. Text byte value exceeds {}. Found text byte code: {:?}.",
                    u8::MAX,
                    be_bytes
                ),
            };

            text_bytes.push(value);
        }

        if let Ok(text) = String::from_utf8(text_bytes) {
            return text;
        }

        panic!("{}", message);
    }

    fn decode_register(&mut self, length_byte: bool, message: &str) -> u32 {
        // Consume register length bytecode if needed.
        if length_byte {
            self.advance();
        }

        let register_be_bytes = match self.current_be_bytes {
            Some(be_bytes) => be_bytes,
            None => panic!("{}", message),
        };

        if !self.is_at_end() {
            // Consume register bytecode.
            self.advance();
        }

        return u32::from_be_bytes(register_be_bytes);
    }

    fn decode_number(&mut self, length_byte: bool, message: &str) -> u32 {
        // Consume number length bytecode if needed.
        if length_byte {
            self.advance();
        }

        let number_be_bytes = match self.current_be_bytes {
            Some(be_bytes) => be_bytes,
            None => panic!("{}", message),
        };

        if !self.is_at_end() {
            // Consume number bytecode.
            self.advance();
        }

        return u32::from_be_bytes(number_be_bytes);
    }

    fn decode_immediate_type(&mut self, message: &str) -> ImmediateType {
        let be_bytes = match self.current_be_bytes {
            Some(be_bytes) => be_bytes,
            None => panic!(
                "No current bytecode to determine immediate type. {}",
                message
            ),
        };

        // Consume value type bytecode.
        self.advance();

        return match ImmediateType::from_be_bytes(be_bytes) {
            Ok(immediate_type) => immediate_type,
            Err(error) => panic!("{} {} Byte code: {:?}", message, error, be_bytes),
        };
    }

    fn decode_immediate(
        &mut self,
        value_type_message: &str,
        value_number_message: &str,
        value_text_message: &str,
    ) -> Immediate {
        return match self.decode_immediate_type(value_type_message) {
            ImmediateType::NUMBER => {
                Immediate::Number(self.decode_number(true, value_number_message))
            }
            ImmediateType::TEXT => Immediate::Text(self.decode_text(value_text_message)),
        };
    }

    fn decode_load_immediate(&mut self) -> LoadImmediateInstruction {
        // Consume LI opcode.
        self.advance();

        // Consume the destination register.
        let destination_register = self.decode_register(
            false,
            "Failed to decode destination register for LI instruction.",
        );

        // Consume the immediate value.
        let value = self.decode_immediate(
            "Failed to decode immediate type for LI instruction.",
            "Failed to decode number for LI instruction.",
            "Failed to decode text for LI instruction.",
        );

        return LoadImmediateInstruction {
            destination_register,
            value,
        };
    }

    fn decode_load_file(&mut self) -> LoadFileInstruction {
        // Consume LF opcode.
        self.advance();

        // Consume the destination register.
        let destination_register = self.decode_register(
            false,
            "Failed to decode destination register for LF instruction.",
        );

        // Consume the immediate value.
        let value = match self.decode_immediate(
            "Failed to decode immediate type for LF instruction.",
            "Failed to decode number for LF instruction.",
            "Failed to decode text for LF instruction.",
        ) {
            Immediate::Text(text) => text,
            _ => panic!("LF instruction requires a text immediate for the file path."),
        };

        return LoadFileInstruction {
            destination_register,
            value,
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

        // Consume the source register.
        let source_register =
            self.decode_register(false, "Failed to read source register for MOV instruction.");

        return MoveInstruction {
            destination_register,
            source_register,
        };
    }

    fn decode_addition(&mut self) -> AddInstruction {
        // Consume ADD opcode.
        self.advance();

        // Consume the destination register.
        let destination_register = self.decode_register(
            false,
            "Failed to read destination register for ADD instruction.",
        );

        // Consume the source register 1.
        let source_register_1 = self.decode_register(
            false,
            "Failed to read source register 1 for ADD instruction.",
        );

        // Consume the source register 2.
        let source_register_2 = self.decode_register(
            false,
            "Failed to read source register 2 for ADD instruction.",
        );

        return AddInstruction {
            destination_register,
            source_register_1,
            source_register_2,
        };
    }

    fn decode_subtract(&mut self) -> SubInstruction {
        // Consume SUB opcode.
        self.advance();

        // Consume the destination register.
        let destination_register = self.decode_register(
            false,
            "Failed to read destination register for SUB instruction.",
        );

        // Consume the source register 1.
        let source_register_1 = self.decode_register(
            false,
            "Failed to read source register 1 for SUB instruction.",
        );

        // Consume the source register 2.
        let source_register_2 = self.decode_register(
            false,
            "Failed to read source register 2 for SUB instruction.",
        );

        return SubInstruction {
            destination_register,
            source_register_1,
            source_register_2,
        };
    }

    fn decode_multiply(&mut self) -> MulInstruction {
        // Consume MUL opcode.
        self.advance();

        // Consume the destination register.
        let destination_register = self.decode_register(
            false,
            "Failed to read destination register for MUL instruction.",
        );

        // Consume the source register 1.
        let source_register_1 = self.decode_register(
            false,
            "Failed to read source register 1 for MUL instruction.",
        );

        // Consume the source register 2.
        let source_register_2 = self.decode_register(
            false,
            "Failed to read source register 2 for MUL instruction.",
        );

        return MulInstruction {
            destination_register,
            source_register_1,
            source_register_2,
        };
    }

    fn decode_divide(&mut self) -> DivInstruction {
        // Consume DIV opcode.
        self.advance();

        // Consume the destination register.
        let destination_register = self.decode_register(
            false,
            "Failed to read destination register for DIV instruction.",
        );

        // Consume the source register 1.
        let source_register_1 = self.decode_register(
            false,
            "Failed to read source register 1 for DIV instruction.",
        );

        // Consume the source register 2.
        let source_register_2 = self.decode_register(
            false,
            "Failed to read source register 2 for DIV instruction.",
        );

        return DivInstruction {
            destination_register,
            source_register_1,
            source_register_2,
        };
    }

    fn decode_similarity(&mut self) -> SimilarityInstruction {
        // Consume SIM opcode.
        self.advance();

        // Consume the destination register.
        let destination_register = self.decode_register(
            false,
            "Failed to read destination register for SIM instruction.",
        );

        // Consume the source register 1.
        let source_register_1 = self.decode_register(
            false,
            "Failed to read source register 1 for SIM instruction.",
        );

        // Consume the source register 2.
        let source_register_2 = self.decode_register(
            false,
            "Failed to read source register 2 for SIM instruction.",
        );

        return SimilarityInstruction {
            destination_register,
            source_register_1,
            source_register_2,
        };
    }

    fn decode_branch(&mut self, op_code: OpCode) -> BranchInstruction {
        // Consume branch opcode.
        self.advance();

        // Consume the source register 1.
        let source_register_1 = self.decode_register(
            false,
            "Failed to read source register 1 for branch instruction.",
        );

        // Consume the source register 2.
        let source_register_2 = self.decode_register(
            false,
            "Failed to read source register 2 for branch instruction.",
        );
        // Consume the branch jump index.
        let byte_code_index = self.decode_number(
            false,
            format!(
                "Failed to read branch jump index for {:?} instruction.",
                op_code
            )
            .as_str(),
        );

        let branch_type = match op_code {
            OpCode::BEQ => BranchType::Equal,
            OpCode::BLT => BranchType::LessThan,
            OpCode::BLE => BranchType::LessThanOrEqual,
            OpCode::BGT => BranchType::GreaterThan,
            OpCode::BGE => BranchType::GreaterThanOrEqual,
            _ => panic!("Invalid opcode for branch instruction."),
        };

        return BranchInstruction {
            branch_type,
            source_register_1,
            source_register_2,
            byte_code_index,
        };
    }

    fn decode_output(&mut self) -> OutputInstruction {
        // Consume OUT opcode.
        self.advance();

        // Consume the source register.
        let source_register =
            self.decode_register(false, "Failed to read source register for OUT instruction.");

        return OutputInstruction { source_register };
    }

    fn decode_op_code(&mut self) -> Instruction {
        let be_bytes = match self.current_be_bytes {
            Some(be_bytes) => be_bytes,
            None => panic!("No current bytecode to determine opcode."),
        };
        let op_code = match OpCode::from_be_bytes(be_bytes) {
            Ok(op_code) => op_code,
            Err(error) => panic!("{} Byte: {:?}", error, be_bytes),
        };

        return match op_code {
            OpCode::LI => Instruction::LoadImmediate(self.decode_load_immediate()),
            OpCode::LF => Instruction::LoadFile(self.decode_load_file()),
            OpCode::MV => Instruction::Move(self.decode_move()),
            OpCode::ADD => Instruction::Add(self.decode_addition()),
            OpCode::SUB => Instruction::Sub(self.decode_subtract()),
            OpCode::MUL => Instruction::Mul(self.decode_multiply()),
            OpCode::DIV => Instruction::Div(self.decode_divide()),
            OpCode::SIM => Instruction::Similarity(self.decode_similarity()),
            OpCode::BEQ => Instruction::Branch(self.decode_branch(op_code)),
            OpCode::BLT => Instruction::Branch(self.decode_branch(op_code)),
            OpCode::BLE => Instruction::Branch(self.decode_branch(op_code)),
            OpCode::BGT => Instruction::Branch(self.decode_branch(op_code)),
            OpCode::BGE => Instruction::Branch(self.decode_branch(op_code)),
            OpCode::OUT => Instruction::Output(self.decode_output()),
        };
    }

    pub fn fetch_and_decode(&mut self) -> Option<Instruction> {
        if self.is_at_end() {
            return None;
        }

        // Initialise current bytecode.
        self.current_be_bytes = Some(
            self.memory
                .read(self.registers.get_instruction_pointer())
                .clone(),
        );

        return Some(self.decode_op_code());
    }

    fn get_value(&self, value: &Immediate) -> Value {
        return match value {
            Immediate::Text(text) => Value::Text(text.clone()),
            Immediate::Number(number) => Value::Number(*number),
        };
    }

    fn execute_load_immediate(&mut self, instruction: &LoadImmediateInstruction, debug: bool) {
        self.registers.set_register(
            instruction.destination_register,
            self.get_value(&instruction.value),
        );

        if debug {
            println!(
                "Executed LI: r{} = \"{:?}\"",
                instruction.destination_register,
                self.registers
                    .get_register(instruction.destination_register)
            );
        }
    }

    fn execute_load_file(&mut self, instruction: &LoadFileInstruction, debug: bool) {
        let file_contents = match read_to_string(&instruction.value) {
            Ok(value) => value,
            Err(error) => panic!("Run failed. Error: {}", error),
        };

        self.registers
            .set_register(instruction.destination_register, Value::Text(file_contents));

        if debug {
            println!(
                "Executed LF: r{} = \"{:?}\"",
                instruction.destination_register,
                self.registers
                    .get_register(instruction.destination_register)
            );
        }
    }

    fn execute_move(&mut self, instruction: &MoveInstruction, debug: bool) {
        let value = self
            .registers
            .get_register(instruction.source_register)
            .clone();
        self.registers
            .set_register(instruction.destination_register, value);

        if debug {
            println!(
                "Executed MOV: r{} = \"{:?}\"",
                instruction.destination_register,
                self.registers
                    .get_register(instruction.destination_register)
            );
        }
    }

    fn execute_add(&mut self, instruction: &AddInstruction, debug: bool) {
        let value_a = self
            .registers
            .get_register(instruction.source_register_1)
            .clone();
        let value_b = self
            .registers
            .get_register(instruction.source_register_2)
            .clone();

        let result = self.semantic_logic_unit.addition(&value_a, &value_b);

        self.registers
            .set_register(instruction.destination_register, Value::Text(result));

        if debug {
            println!(
                "Executed ADD: {:?} + {:?} -> r{} = \"{:?}\"",
                value_a,
                value_b,
                instruction.destination_register,
                self.registers
                    .get_register(instruction.destination_register)
            );
        }
    }

    fn execute_subtract(&mut self, instruction: &SubInstruction, debug: bool) {
        let value_a = self
            .registers
            .get_register(instruction.source_register_1)
            .clone();
        let value_b = self
            .registers
            .get_register(instruction.source_register_2)
            .clone();

        let result = self.semantic_logic_unit.subtract(&value_a, &value_b);

        self.registers
            .set_register(instruction.destination_register, Value::Text(result));

        if debug {
            println!(
                "Executed SUB: {:?} - {:?} -> r{} = \"{:?}\"",
                value_a,
                value_b,
                instruction.destination_register,
                self.registers
                    .get_register(instruction.destination_register)
            );
        }
    }

    fn execute_multiply(&mut self, instruction: &MulInstruction, debug: bool) {
        let value_a = self
            .registers
            .get_register(instruction.source_register_1)
            .clone();
        let value_b = self
            .registers
            .get_register(instruction.source_register_2)
            .clone();

        let result = self.semantic_logic_unit.multiply(&value_a, &value_b);

        self.registers
            .set_register(instruction.destination_register, Value::Text(result));

        if debug {
            println!(
                "Executed MUL: {:?} * {:?} -> r{} = \"{:?}\"",
                value_a,
                value_b,
                instruction.destination_register,
                self.registers
                    .get_register(instruction.destination_register)
            );
        }
    }

    fn execute_divide(&mut self, instruction: &DivInstruction, debug: bool) {
        let value_a = self
            .registers
            .get_register(instruction.source_register_1)
            .clone();
        let value_b = self
            .registers
            .get_register(instruction.source_register_2)
            .clone();

        let result = self.semantic_logic_unit.divide(&value_a, &value_b);

        self.registers
            .set_register(instruction.destination_register, Value::Text(result));

        if debug {
            println!(
                "Executed DIV: {:?} / {:?} -> r{} = \"{:?}\"",
                value_a,
                value_b,
                instruction.destination_register,
                self.registers
                    .get_register(instruction.destination_register)
            );
        }
    }

    fn execute_similarity(&mut self, instruction: &SimilarityInstruction, debug: bool) {
        let value_a = self
            .registers
            .get_register(instruction.source_register_1)
            .clone();
        let value_b = self
            .registers
            .get_register(instruction.source_register_2)
            .clone();

        let result = self.semantic_logic_unit.similarity(&value_a, &value_b);

        self.registers
            .set_register(instruction.destination_register, Value::Number(result));

        if debug {
            println!(
                "Executed SIM: {:?} ~ {:?} -> r{} = \"{:?}\"",
                value_a,
                value_b,
                instruction.destination_register,
                self.registers
                    .get_register(instruction.destination_register)
            );
        }
    }

    fn execute_branch(&mut self, instruction: &BranchInstruction, debug: bool) {
        let value_a = match self
            .registers
            .get_register(instruction.source_register_1)
            .clone()
        {
            Value::Number(num) => num,
            _ => panic!(
                "{:?} instruction requires numeric operands.",
                instruction.branch_type
            ),
        };
        let value_b = match self
            .registers
            .get_register(instruction.source_register_2)
            .clone()
        {
            Value::Number(num) => num,
            _ => panic!(
                "{:?} instruction requires numeric operands.",
                instruction.branch_type
            ),
        };
        let address = instruction.byte_code_index.clone();
        let is_true = match instruction.branch_type {
            BranchType::Equal => value_a == value_b,
            BranchType::LessThan => value_a < value_b,
            BranchType::LessThanOrEqual => value_a <= value_b,
            BranchType::GreaterThan => value_a > value_b,
            BranchType::GreaterThanOrEqual => value_a >= value_b,
        };

        if is_true {
            let address = match usize::try_from(address) {
                Ok(address) => address,
                Err(_) => panic!(
                    "Failed to convert address to usize for branch instruction. Address value: {}. Address value must be between 0 and {}.",
                    address,
                    usize::MAX
                ),
            };

            self.registers.set_instruction_pointer(address);
        }

        if debug {
            match instruction.branch_type {
                BranchType::Equal => {
                    println!(
                        "Executed JEQ: {:?} == {:?} -> {}, {}",
                        value_a, value_b, is_true, instruction.byte_code_index
                    );
                }
                BranchType::LessThan => {
                    println!(
                        "Executed JLT: {:?} < {:?} -> {}, {}",
                        value_a, value_b, is_true, instruction.byte_code_index
                    );
                }
                BranchType::LessThanOrEqual => {
                    println!(
                        "Executed JLE: {:?} <= {:?} -> {}, {}",
                        value_a, value_b, is_true, instruction.byte_code_index
                    );
                }
                BranchType::GreaterThan => {
                    println!(
                        "Executed JGT: {:?} > {:?} -> {}, {}",
                        value_a, value_b, is_true, instruction.byte_code_index
                    );
                }
                BranchType::GreaterThanOrEqual => println!(
                    "Executed JGE: {:?} >= {:?} -> {}, {}",
                    value_a, value_b, is_true, instruction.byte_code_index
                ),
            }
        }
    }

    fn execute_output(&mut self, instruction: &OutputInstruction, debug: bool) {
        let value_a = match self.registers.get_register(instruction.source_register) {
            Value::Text(text) => text.clone(),
            Value::Number(number) => number.to_string(),
            _ => panic!("OUT instruction requires text or number operands."),
        };

        if debug {
            println!("Executed OUT: {}", value_a);
        } else {
            println!("{}", value_a);
        }
    }

    pub fn execute(&mut self, instruction: &Instruction, debug: bool) {
        match instruction {
            Instruction::LoadImmediate(instruction) => {
                self.execute_load_immediate(instruction, debug)
            }
            Instruction::LoadFile(instruction) => self.execute_load_file(instruction, debug),
            Instruction::Move(instruction) => self.execute_move(instruction, debug),
            Instruction::Add(instruction) => self.execute_add(instruction, debug),
            Instruction::Sub(instruction) => self.execute_subtract(instruction, debug),
            Instruction::Mul(instruction) => self.execute_multiply(instruction, debug),
            Instruction::Div(instruction) => self.execute_divide(instruction, debug),
            Instruction::Similarity(instruction) => self.execute_similarity(instruction, debug),
            Instruction::Branch(instruction) => self.execute_branch(instruction, debug),
            Instruction::Output(instruction) => self.execute_output(instruction, debug),
        }
    }
}
