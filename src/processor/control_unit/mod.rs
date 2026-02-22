use std::fs::read_to_string;

use crate::{
    assembler::{
        immediate::{Immediate, ImmediateType},
        opcode::OpCode,
    },
    processor::control_unit::{
        instruction::{
            BranchInstruction, BranchType, ExitInstruction, Instruction, LoadFileInstruction,
            LoadImmediateInstruction, MoveInstruction, OutputInstruction, RType, RTypeInstruction,
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
        self.registers.get_instruction_pointer() >= self.memory.length()
    }

    fn peek(&self) -> &[u8; 4] {
        match self.memory.read(self.registers.get_instruction_pointer()) {
            Ok(bytes) => bytes,
            Err(error) => panic!(
                "Failed to read byte code at instruction pointer during peek. Error: {}. Instruction pointer value: {}.",
                error,
                self.registers.get_instruction_pointer()
            ),
        }
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
            None => panic!(
                "Expected register byte code, but no current byte code found. {}",
                message
            ),
        };

        if !self.is_at_end() {
            // Consume register bytecode.
            self.advance();
        }

        u32::from_be_bytes(register_be_bytes)
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

        u32::from_be_bytes(number_be_bytes)
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

        match ImmediateType::from_be_bytes(be_bytes) {
            Ok(immediate_type) => immediate_type,
            Err(error) => panic!(
                "{} {}, Instruction Byte code: {:?}",
                message, error, be_bytes
            ),
        }
    }

    fn decode_immediate(
        &mut self,
        value_type_message: &str,
        value_number_message: &str,
        value_text_message: &str,
    ) -> Immediate {
        match self.decode_immediate_type(value_type_message) {
            ImmediateType::Number => {
                Immediate::Number(self.decode_number(true, value_number_message))
            }
            ImmediateType::Register => {
                Immediate::Register(self.decode_register(true, value_number_message))
            }
            ImmediateType::Text => Immediate::Text(self.decode_text(value_text_message)),
        }
    }

    fn decode_load_immediate(&mut self) -> LoadImmediateInstruction {
        // Consume LI opcode.
        self.decode_op_code(&OpCode::Li, "Failed to decode LI opcode.");

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

        LoadImmediateInstruction {
            destination_register,
            value,
        }
    }

    fn decode_load_file(&mut self) -> LoadFileInstruction {
        // Consume LF opcode.
        self.decode_op_code(&OpCode::Lf, "Failed to decode LF opcode.");

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

        LoadFileInstruction {
            destination_register,
            value,
        }
    }

    fn decode_move(&mut self) -> MoveInstruction {
        // Consume MOV opcode.
        self.decode_op_code(&OpCode::Mv, "Failed to decode MV opcode.");

        // Consume the destination register.
        let destination_register = self.decode_register(
            false,
            "Failed to read destination register for MOV instruction.",
        );

        // Consume the source register.
        let source_register =
            self.decode_register(false, "Failed to read source register for MOV instruction.");

        MoveInstruction {
            destination_register,
            source_register,
        }
    }

    fn decode_r_type(&mut self, op_code: OpCode) -> RTypeInstruction {
        // Consume opcode.
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

        // Decode first immediate operand.
        let immediate_1 = self.decode_immediate(
            "Failed to decode immediate type for R-type instruction.",
            "Failed to decode number for cognitive instruction.",
            "Failed to decode text for cognitive instruction.",
        );

        // Decode second immediate operand (HAL uses a dummy numeric 0).
        let immediate_2 = self.decode_immediate(
            "Failed to decode immediate type for cognitive instruction.",
            "Failed to decode number for cognitive instruction.",
            "Failed to decode text for cognitive instruction.",
        );

        let r_type = match op_code {
            // Generative operations.
            OpCode::Sum => RType::Sum,
            OpCode::Exp => RType::Exp,
            OpCode::Trn => RType::Trn,
            // Cognitive operations.
            OpCode::Cmp => RType::Cmp,
            OpCode::Syn => RType::Syn,
            OpCode::Flt => RType::Flt,
            OpCode::Prd => RType::Prd,
            // Guardrails operations.
            OpCode::Vfy => RType::Vfy,
            OpCode::Sim => RType::Sim,
            _ => panic!("Invalid opcode '{:?}' for R-type instruction.", op_code),
        };

        RTypeInstruction {
            r_type,
            destination_register,
            immediate_1,
            immediate_2,
        }
    }

    fn decode_branch(&mut self, op_code: OpCode) -> BranchInstruction {
        // Consume branch opcode.
        self.advance();

        // Decode the first immediate operand.
        let immediate_1 = self.decode_immediate(
            "Failed to decode immediate type for branch instruction.",
            "Failed to decode number for branch instruction.",
            "Failed to decode text for branch instruction.",
        );

        // Decode the second immediate operand.
        let immediate_2 = self.decode_immediate(
            "Failed to decode immediate type for branch instruction.",
            "Failed to decode number for branch instruction.",
            "Failed to decode text for branch instruction.",
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
            OpCode::Beq => BranchType::Eq,
            OpCode::Blt => BranchType::Lt,
            OpCode::Ble => BranchType::Le,
            OpCode::Bgt => BranchType::Gt,
            OpCode::Bge => BranchType::Ge,
            _ => panic!("Invalid opcode '{:?}' for branch instruction.", op_code),
        };

        BranchInstruction {
            branch_type,
            immediate_1,
            immediate_2,
            byte_code_index,
        }
    }

    fn decode_output(&mut self) -> OutputInstruction {
        // Consume OUT opcode.
        self.advance();

        // Decode the immediate operand for OUT.
        let immediate = self.decode_immediate(
            "Failed to decode immediate type for OUT instruction.",
            "Failed to decode number for OUT instruction.",
            "Failed to decode text for OUT instruction.",
        );

        OutputInstruction { immediate }
    }

    fn decode_exit(&mut self) -> ExitInstruction {
        // Consume EXIT opcode.
        self.decode_op_code(&OpCode::Exit, "Failed to decode EXIT opcode.");

        ExitInstruction
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
            OpCode::Li => Instruction::LoadImmediate(self.decode_load_immediate()),
            OpCode::Lf => Instruction::LoadFile(self.decode_load_file()),
            OpCode::Mv => Instruction::Move(self.decode_move()),
            // Control flow instructions.
            OpCode::Beq => Instruction::Branch(self.decode_branch(op_code)),
            OpCode::Blt => Instruction::Branch(self.decode_branch(op_code)),
            OpCode::Ble => Instruction::Branch(self.decode_branch(op_code)),
            OpCode::Bgt => Instruction::Branch(self.decode_branch(op_code)),
            OpCode::Bge => Instruction::Branch(self.decode_branch(op_code)),
            OpCode::Exit => Instruction::Exit(self.decode_exit()),
            // I/O instructions.
            OpCode::Out => Instruction::Output(self.decode_output()),
            // Generative operations.
            OpCode::Sum | OpCode::Exp | OpCode::Trn => {
                Instruction::RType(self.decode_r_type(op_code))
            }
            // Cognitive operations.
            OpCode::Cmp | OpCode::Syn | OpCode::Flt | OpCode::Prd => {
                Instruction::RType(self.decode_r_type(op_code))
            }
            // Guardrails operations.
            OpCode::Vfy | OpCode::Sim => Instruction::RType(self.decode_r_type(op_code)),
        };

        Some(instruction)
    }

    fn execute_load_immediate(&mut self, instruction: &LoadImmediateInstruction, debug: bool) {
        let value = match &instruction.value {
            Immediate::Text(text) => Value::Text(text.to_string()),
            Immediate::Number(number) => Value::Number(*number),
            Immediate::Register(register) => match self.registers.get_register(*register) {
                Ok(value) => value.to_owned(),
                Err(error) => panic!(
                    "Failed to read source register r{} for LI instruction. Error: {}",
                    register, error
                ),
            },
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

    fn execute_r_type(&mut self, instruction: &RTypeInstruction, debug: bool) {
        let value_a = match &instruction.immediate_1 {
            Immediate::Text(text) => Value::Text(text.to_string()),
            Immediate::Number(number) => Value::Number(*number),
            Immediate::Register(register) => match self.registers.get_register(*register) {
                Ok(value) => value.to_owned(),
                Err(error) => panic!(
                    "Failed to read source register r{} for R-type instruction. Error: {}",
                    register, error
                ),
            },
        };

        let value_b = match &instruction.immediate_2 {
            Immediate::Text(text) => Value::Text(text.to_string()),
            Immediate::Number(number) => Value::Number(*number),
            Immediate::Register(register) => match self.registers.get_register(*register) {
                Ok(value) => value.to_owned(),
                Err(error) => panic!(
                    "Failed to read source register r{} for R-type instruction. Error: {}",
                    register, error
                ),
            },
        };

        let result = match self
            .language_logic_unit
            .run(&instruction.r_type, &value_a, &value_b)
        {
            Ok(result) => result,
            Err(error) => panic!(
                "Failed to perform {:?}. Error: {}",
                instruction.r_type, error
            ),
        };

        if debug {
            println!(
                "Executed {:?}: {:?} {:?} {:?} -> r{} = \"{:?}\"",
                instruction.r_type,
                value_a,
                instruction.r_type,
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
                instruction.r_type, error
            ),
        };
    }

    fn execute_branch(&mut self, instruction: &BranchInstruction, debug: bool) {
        // Resolve immediates to numeric u32 values.
        let value_a: u32 = match &instruction.immediate_1 {
            Immediate::Number(number) => *number,
            Immediate::Register(register) => match self.registers.get_register(*register) {
                Ok(Value::Number(number)) => *number,
                Ok(_) => panic!(
                    "{:?} instruction requires numeric operands.",
                    instruction.branch_type
                ),
                Err(error) => panic!("Failed to execute branch instruction. Error: {}", error),
            },
            Immediate::Text(_) => panic!(
                "{:?} instruction requires numeric operands.",
                instruction.branch_type
            ),
        };

        let value_b: u32 = match &instruction.immediate_2 {
            Immediate::Number(number) => *number,
            Immediate::Register(register) => match self.registers.get_register(*register) {
                Ok(Value::Number(number)) => *number,
                Ok(_) => panic!(
                    "{:?} instruction requires numeric operands.",
                    instruction.branch_type
                ),
                Err(error) => panic!("Failed to execute branch instruction. Error: {}", error),
            },
            Immediate::Text(_) => panic!(
                "{:?} instruction requires numeric operands.",
                instruction.branch_type
            ),
        };

        let address = instruction.byte_code_index;
        let is_true = match instruction.branch_type {
            BranchType::Eq => value_a == value_b,
            BranchType::Lt => value_a < value_b,
            BranchType::Le => value_a <= value_b,
            BranchType::Gt => value_a > value_b,
            BranchType::Ge => value_a >= value_b,
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
                BranchType::Eq => {
                    println!(
                        "Executed {:?}: {:?} == {:?} -> {}, {}",
                        instruction.branch_type,
                        value_a,
                        value_b,
                        is_true,
                        instruction.byte_code_index
                    );
                }
                BranchType::Lt => {
                    println!(
                        "Executed {:?}: {:?} < {:?} -> {}, {}",
                        instruction.branch_type,
                        value_a,
                        value_b,
                        is_true,
                        instruction.byte_code_index
                    );
                }
                BranchType::Le => {
                    println!(
                        "Executed {:?}: {:?} <= {:?} -> {}, {}",
                        instruction.branch_type,
                        value_a,
                        value_b,
                        is_true,
                        instruction.byte_code_index
                    );
                }
                BranchType::Gt => {
                    println!(
                        "Executed {:?}: {:?} > {:?} -> {}, {}",
                        instruction.branch_type,
                        value_a,
                        value_b,
                        is_true,
                        instruction.byte_code_index
                    );
                }
                BranchType::Ge => println!(
                    "Executed {:?}: {:?} >= {:?} -> {}, {}",
                    instruction.branch_type, value_a, value_b, is_true, instruction.byte_code_index
                ),
            }
        }
    }

    fn execute_output(&mut self, instruction: &OutputInstruction, debug: bool) {
        let value_a = match &instruction.immediate {
            Immediate::Text(text) => text.to_string(),
            Immediate::Number(number) => number.to_string(),
            Immediate::Register(register) => match self.registers.get_register(*register) {
                Ok(value) => match value {
                    Value::Text(text) => text.to_string(),
                    Value::Number(number) => number.to_string(),
                    Value::None => panic!("Output register r{} is empty.", register),
                },
                Err(error) => panic!("Failed to execute OUT instruction. Error: {}", error),
            },
        };

        if debug {
            println!("Executed OUT: {}", value_a);
        } else {
            println!("{}", value_a);
        }
    }

    fn execute_exit(&mut self, _instruction: &ExitInstruction, debug: bool) {
        if debug {
            println!("Executed EXIT: Halting execution.");
        }

        // Set instruction pointer to memory length to indicate end of execution.
        self.registers.set_instruction_pointer(self.memory.length());
    }

    pub fn execute(&mut self, instruction: &Instruction, debug: bool) {
        match instruction {
            Instruction::LoadImmediate(instruction) => {
                self.execute_load_immediate(instruction, debug)
            }
            Instruction::LoadFile(instruction) => self.execute_load_file(instruction, debug),
            Instruction::Move(instruction) => self.execute_move(instruction, debug),
            Instruction::Branch(instruction) => self.execute_branch(instruction, debug),
            Instruction::Exit(instruction) => self.execute_exit(instruction, debug),
            Instruction::Output(instruction) => self.execute_output(instruction, debug),
            Instruction::RType(instruction) => self.execute_r_type(instruction, debug),
        }
    }
}
