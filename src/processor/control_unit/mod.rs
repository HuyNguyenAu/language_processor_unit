use std::fs::read_to_string;

use crate::{
    assembler::{
        immediate::{Immediate, ImmediateType},
        opcode::OpCode,
    },
    processor::control_unit::{
        instruction::{
            BranchInstruction, BranchType, HeuristicInstruction, HeuristicType, Instruction,
            LoadFileInstruction, LoadImmediateInstruction, MoveInstruction, OutputInstruction,
            SemanticInstruction, SemanticType,
        },
        language_logic_unit::LanguageLogicUnit,
        memory_unit::MemoryUnit,
        registers::{Registers, Value},
    },
};

mod instruction;
mod language_logic_unit;
mod memory_unit;
mod registers;

pub struct ControlUnit {
    memory: MemoryUnit,
    registers: Registers,
    language_logic_unit: LanguageLogicUnit,

    previous_be_bytes: Option<[u8; 4]>,
    current_be_bytes: Option<[u8; 4]>,
}

impl ControlUnit {
    pub fn new() -> Self {
        ControlUnit {
            memory: MemoryUnit::new(),
            registers: Registers::new(),
            language_logic_unit: LanguageLogicUnit::new(),
            previous_be_bytes: None,
            current_be_bytes: None,
        }
    }

    fn is_at_end(&self) -> bool {
        return self.registers.get_instruction_pointer() >= self.memory.length();
    }

    fn peek(&self) -> &[u8; 4] {
        return match self.memory.read(self.registers.get_instruction_pointer()) {
            Ok(bytes) => bytes,
            Err(error) => panic!(
                "Failed to read byte code at instruction pointer during peek. Error: {}. Instruction pointer value: {}.",
                error,
                self.registers.get_instruction_pointer()
            ),
        };
    }

    fn advance(&mut self) {
        self.registers.advance_instruction_pointer();

        self.previous_be_bytes = self.current_be_bytes;

        if self.is_at_end() {
            self.current_be_bytes = None;

            return;
        }

        let bytes = match self.memory.read(self.registers.get_instruction_pointer()) {
            Ok(bytes) => *bytes,
            Err(error) => panic!(
                "Failed to read byte code at instruction pointer. Error: {}. Instruction pointer value: {}.",
                error,
                self.registers.get_instruction_pointer()
            ),
        };
        self.current_be_bytes = Some(bytes);
    }

    fn decode_op_code(&mut self, expected_op_code: &OpCode, message: &str) -> OpCode {
        if let Some(current_be_bytes) = &self.current_be_bytes
            && let Ok(current_op_code) = OpCode::from_be_bytes(*current_be_bytes)
            && current_op_code == *expected_op_code
        {
            self.advance();

            return current_op_code;
        }

        panic!(
            "{} Expected opcode: {:?}. Found byte code: {:?}.",
            message, expected_op_code, self.current_be_bytes
        );
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
            Err(error) => panic!(
                "{} {}, Instruction Byte code: {:?}",
                message, error, be_bytes
            ),
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
        self.decode_op_code(&OpCode::LI, "Failed to decode LI opcode.");

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
        self.decode_op_code(&OpCode::LF, "Failed to decode LF opcode.");

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
        self.decode_op_code(&OpCode::MV, "Failed to decode MV opcode.");

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

    fn decode_semantic(&mut self, op_code: OpCode) -> SemanticInstruction {
        // Consume semantic opcode.
        self.decode_op_code(
            &op_code,
            format!("Failed to decode {:?} opcode.", op_code).as_str(),
        );

        // Consume the destination register.
        let destination_register = self.decode_register(
            false,
            format!(
                "Failed to read destination register for {:?} instruction.",
                op_code
            )
            .as_str(),
        );

        // Consume the source register 1.
        let source_register_1 = self.decode_register(
            false,
            format!(
                "Failed to read source register 1 for {:?} instruction.",
                op_code
            )
            .as_str(),
        );

        // Consume the source register 2.
        let source_register_2 = self.decode_register(
            false,
            format!(
                "Failed to read source register 2 for {:?} instruction.",
                op_code
            )
            .as_str(),
        );

        let semantic_type = match op_code {
            OpCode::ADD => SemanticType::ADD,
            OpCode::SUB => SemanticType::SUB,
            OpCode::MUL => SemanticType::MUL,
            OpCode::DIV => SemanticType::DIV,
            OpCode::INF => SemanticType::INF,
            OpCode::ADT => SemanticType::ADT,
            _ => panic!("Invalid opcode '{:?}' for semantic instruction.", op_code),
        };

        return SemanticInstruction {
            semantic_type,
            destination_register,
            source_register_1,
            source_register_2,
        };
    }

    fn decode_heuristic(&mut self, op_code: OpCode) -> HeuristicInstruction {
        // Consume heuristic opcode.
        self.decode_op_code(
            &op_code,
            format!("Failed to decode {:?} opcode.", op_code).as_str(),
        );

        // Consume the destination register.
        let destination_register = self.decode_register(
            false,
            format!(
                "Failed to read destination register for {:?} instruction.",
                op_code
            )
            .as_str(),
        );

        // Consume the source register 1.
        let source_register_1 = self.decode_register(
            false,
            format!(
                "Failed to read source register 1 for {:?} instruction.",
                op_code
            )
            .as_str(),
        );

        let source_register_2 = self.decode_register(
            false,
            format!(
                "Failed to read source register 2 for {:?} instruction.",
                op_code
            )
            .as_str(),
        );

        let heuristic_type = match op_code {
            OpCode::EQV => HeuristicType::EQV,
            OpCode::INT => HeuristicType::INT,
            OpCode::HAL => HeuristicType::HAL,
            OpCode::SIM => HeuristicType::SIM,
            _ => panic!("Invalid opcode '{:?}' for heuristic instruction.", op_code),
        };

        return HeuristicInstruction {
            heuristic_type,
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
            OpCode::BEQ => BranchType::EQ,
            OpCode::BLT => BranchType::LT,
            OpCode::BLE => BranchType::LE,
            OpCode::BGT => BranchType::GT,
            OpCode::BGE => BranchType::GE,
            _ => panic!("Invalid opcode '{:?}' for branch instruction.", op_code),
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

    pub fn load_byte_code(&mut self, byte_code: Vec<[u8; 4]>) {
        self.memory.load(byte_code);

        // Reset instruction pointer and byte code tracking.
        self.registers.set_instruction_pointer(0);
        self.previous_be_bytes = None;
        self.current_be_bytes = Some(self.peek().to_owned());
    }

    pub fn fetch_and_decode(&mut self) -> Option<Instruction> {
        if self.is_at_end() {
            return None;
        }

        let current_be_bytes = match self.current_be_bytes {
            Some(be_bytes) => be_bytes,
            None => panic!(
                "No current byte code to fetch and decode. Instruction pointer value: {}.",
                self.registers.get_instruction_pointer()
            ),
        };
        let op_code = match OpCode::from_be_bytes(current_be_bytes) {
            Ok(op_code) => op_code,
            Err(error) => panic!(
                "Failed to decode opcode from byte code. Error: {}. Byte code: {:?}.",
                error, current_be_bytes
            ),
        };
        let instruction = match op_code {
            // Data movement instructions.
            OpCode::LI => Instruction::LoadImmediate(self.decode_load_immediate()),
            OpCode::LF => Instruction::LoadFile(self.decode_load_file()),
            OpCode::MV => Instruction::Move(self.decode_move()),
            // Semantic instructions.
            OpCode::ADD => Instruction::Semantic(self.decode_semantic(OpCode::ADD)),
            OpCode::SUB => Instruction::Semantic(self.decode_semantic(OpCode::SUB)),
            OpCode::MUL => Instruction::Semantic(self.decode_semantic(OpCode::MUL)),
            OpCode::DIV => Instruction::Semantic(self.decode_semantic(OpCode::DIV)),
            OpCode::INF => Instruction::Semantic(self.decode_semantic(OpCode::INF)),
            OpCode::ADT => Instruction::Semantic(self.decode_semantic(OpCode::ADT)),
            // Heuristic instructions.
            OpCode::EQV => Instruction::Heuristic(self.decode_heuristic(OpCode::EQV)),
            OpCode::INT => Instruction::Heuristic(self.decode_heuristic(OpCode::INT)),
            OpCode::HAL => Instruction::Heuristic(self.decode_heuristic(OpCode::HAL)),
            OpCode::SIM => Instruction::Heuristic(self.decode_heuristic(OpCode::SIM)),
            // Branch instructions.
            OpCode::BEQ => Instruction::Branch(self.decode_branch(op_code)),
            OpCode::BLT => Instruction::Branch(self.decode_branch(op_code)),
            OpCode::BLE => Instruction::Branch(self.decode_branch(op_code)),
            OpCode::BGT => Instruction::Branch(self.decode_branch(op_code)),
            OpCode::BGE => Instruction::Branch(self.decode_branch(op_code)),
            // I/O instructions.
            OpCode::OUT => Instruction::Output(self.decode_output()),
        };

        return Some(instruction);
    }

    fn execute_load_immediate(&mut self, instruction: &LoadImmediateInstruction, debug: bool) {
        let value = match &instruction.value {
            Immediate::Text(text) => Value::Text(text.to_string()),
            Immediate::Number(number) => Value::Number(*number),
        };

        match self
            .registers
            .set_register(instruction.destination_register, &value)
        {
            Ok(_) => (),
            Err(error) => panic!(
                "Failed to set register for LI instruction. Error: {}",
                error
            ),
        };

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

        match self.registers.set_register(
            instruction.destination_register,
            &Value::Text(file_contents),
        ) {
            Ok(_) => (),
            Err(error) => panic!(
                "Failed to set register for LF instruction. Error: {}",
                error
            ),
        };

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
        let value = match self.registers.get_register(instruction.source_register) {
            Ok(value) => value.to_owned(),
            Err(error) => panic!("Failed to execute MOV instruction. Error: {}", error),
        };

        match self
            .registers
            .set_register(instruction.destination_register, &value)
        {
            Ok(_) => (),
            Err(error) => panic!(
                "Failed to set register for MOV instruction. Error: {}",
                error
            ),
        };

        if debug {
            println!(
                "Executed MOV: r{} = \"{:?}\"",
                instruction.destination_register,
                self.registers
                    .get_register(instruction.destination_register)
            );
        }
    }

    fn execute_semantic(&mut self, instruction: &SemanticInstruction, debug: bool) {
        let value_a = match self.registers.get_register(instruction.source_register_1) {
            Ok(value) => value,
            Err(error) => panic!(
                "Failed to execute {:?} instruction. Error: {}",
                instruction.semantic_type, error
            ),
        };
        let value_b = match self.registers.get_register(instruction.source_register_2) {
            Ok(value) => value,
            Err(error) => panic!(
                "Failed to execute {:?} instruction. Error: {}",
                instruction.semantic_type, error
            ),
        };

        let opcode: OpCode = match instruction.semantic_type {
            SemanticType::ADD => OpCode::ADD,
            SemanticType::SUB => OpCode::SUB,
            SemanticType::MUL => OpCode::MUL,
            SemanticType::DIV => OpCode::DIV,
            SemanticType::INF => OpCode::INF,
            SemanticType::ADT => OpCode::ADT,
        };

        let result = match self.language_logic_unit.run(&opcode, value_a, value_b) {
            Ok(result) => result,
            Err(error) => panic!(
                "Failed to perform {:?}. Error: {}",
                instruction.semantic_type, error
            ),
        };

        if debug {
            println!(
                "Executed {:?}: {:?} {} {:?} -> r{} = \"{:?}\"",
                instruction.semantic_type,
                value_a,
                match instruction.semantic_type {
                    SemanticType::ADD => "+",
                    SemanticType::SUB => "-",
                    SemanticType::MUL => "*",
                    SemanticType::DIV => "/",
                    SemanticType::INF => "->",
                    SemanticType::ADT => "<->",
                },
                value_b,
                instruction.destination_register,
                result
            );
        }

        match self
            .registers
            .set_register(instruction.destination_register, &result)
        {
            Ok(_) => {}
            Err(error) => panic!(
                "Failed to set register for {:?} instruction. Error: {}",
                instruction.semantic_type, error
            ),
        };
    }

    fn execute_heuristic(&mut self, instruction: &HeuristicInstruction, debug: bool) {
        let value_a = match self.registers.get_register(instruction.source_register_1) {
            Ok(value) => value,
            Err(error) => panic!(
                "Failed to execute {:?} instruction. Error: {}",
                instruction.heuristic_type, error
            ),
        };
        let value_b = match self.registers.get_register(instruction.source_register_2) {
            Ok(value) => value,
            Err(error) => panic!(
                "Failed to execute {:?} instruction. Error: {}",
                instruction.heuristic_type, error
            ),
        };

        let opcode: OpCode = match instruction.heuristic_type {
            HeuristicType::EQV => OpCode::EQV,
            HeuristicType::INT => OpCode::INT,
            HeuristicType::HAL => OpCode::HAL,
            HeuristicType::SIM => OpCode::SIM,
        };

        let result = match self.language_logic_unit.run(&opcode, value_a, value_b) {
            Ok(result) => result,
            Err(error) => panic!(
                "Failed to perform {:?}. Error: {}",
                instruction.heuristic_type, error
            ),
        };

        if debug {
            println!(
                "Executed {:?}: {:?} {} {:?} -> r{} = \"{:?}\"",
                instruction.heuristic_type,
                value_a,
                match instruction.heuristic_type {
                    HeuristicType::EQV => "EQV",
                    HeuristicType::INT => "INT",
                    HeuristicType::HAL => "HAL",
                    HeuristicType::SIM => "SIM",
                },
                value_b,
                instruction.destination_register,
                result
            );
        }

        match self
            .registers
            .set_register(instruction.destination_register, &result)
        {
            Ok(_) => {}
            Err(error) => panic!(
                "Failed to set register for {:?} instruction. Error: {}",
                instruction.heuristic_type, error
            ),
        };
    }

    fn execute_branch(&mut self, instruction: &BranchInstruction, debug: bool) {
        let value_a = match self.registers.get_register(instruction.source_register_1) {
            Ok(value) => match value {
                Value::Number(number) => *number,
                _ => panic!(
                    "{:?} instruction requires numeric operands.",
                    instruction.branch_type
                ),
            },
            Err(error) => panic!("Failed to execute branch instruction. Error: {}", error),
        };
        let value_b = match self.registers.get_register(instruction.source_register_2) {
            Ok(value) => match value {
                Value::Number(number) => *number,
                _ => panic!(
                    "{:?} instruction requires numeric operands.",
                    instruction.branch_type
                ),
            },
            Err(error) => panic!("Failed to execute branch instruction. Error: {}", error),
        };
        let address = instruction.byte_code_index;
        let is_true = match instruction.branch_type {
            BranchType::EQ => value_a == value_b,
            BranchType::LT => value_a < value_b,
            BranchType::LE => value_a <= value_b,
            BranchType::GT => value_a > value_b,
            BranchType::GE => value_a >= value_b,
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
            self.current_be_bytes = Some(self.peek().to_owned());
        }

        if debug {
            match instruction.branch_type {
                BranchType::EQ => {
                    println!(
                        "Executed {:?}: {:?} == {:?} -> {}, {}",
                        instruction.branch_type,
                        value_a,
                        value_b,
                        is_true,
                        instruction.byte_code_index
                    );
                }
                BranchType::LT => {
                    println!(
                        "Executed {:?}: {:?} < {:?} -> {}, {}",
                        instruction.branch_type,
                        value_a,
                        value_b,
                        is_true,
                        instruction.byte_code_index
                    );
                }
                BranchType::LE => {
                    println!(
                        "Executed {:?}: {:?} <= {:?} -> {}, {}",
                        instruction.branch_type,
                        value_a,
                        value_b,
                        is_true,
                        instruction.byte_code_index
                    );
                }
                BranchType::GT => {
                    println!(
                        "Executed {:?}: {:?} > {:?} -> {}, {}",
                        instruction.branch_type,
                        value_a,
                        value_b,
                        is_true,
                        instruction.byte_code_index
                    );
                }
                BranchType::GE => println!(
                    "Executed {:?}: {:?} >= {:?} -> {}, {}",
                    instruction.branch_type, value_a, value_b, is_true, instruction.byte_code_index
                ),
            }
        }
    }

    fn execute_output(&mut self, instruction: &OutputInstruction, debug: bool) {
        let value_a = match self.registers.get_register(instruction.source_register) {
            Ok(value) => match value {
                Value::Text(text) => text.to_string(),
                Value::Number(number) => number.to_string(),
                _ => panic!("OUT instruction requires text or number operands."),
            },
            Err(error) => panic!("Failed to execute OUT instruction. Error: {}", error),
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
            Instruction::Semantic(instruction) => self.execute_semantic(instruction, debug),
            Instruction::Heuristic(instruction) => self.execute_heuristic(instruction, debug),
            Instruction::Branch(instruction) => self.execute_branch(instruction, debug),
            Instruction::Output(instruction) => self.execute_output(instruction, debug),
        }
    }
}
