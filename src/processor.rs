use crate::instruction::{Instruction, OpCode, OperandType};

struct MemoryUnit {
    data: Vec<u8>,
}

impl MemoryUnit {
    fn new() -> Self {
        MemoryUnit { data: Vec::new() }
    }

    fn load(&mut self, bytecode: Vec<u8>) {
        println!("Loading bytecode of length {}", bytecode.len());
        self.data = bytecode;
    }

    fn read_byte(&self, address: usize) -> &u8 {
        if let Some(byte) = self.data.get(address) {
            return byte;
        }

        panic!("Address out of bounds.");
    }
}

struct Registers {
    register_1: u8,
    register_2: u8,
    register_3: u8,
    register_4: u8,
    register_5: u8,
    register_6: u8,
    instruction_pointer: u8,
}

impl Registers {
    fn new() -> Self {
        Registers {
            register_1: 0,
            register_2: 0,
            register_3: 0,
            register_4: 0,
            register_5: 0,
            register_6: 0,
            instruction_pointer: 0,
        }
    }

    fn advance_instruction_pointer(&mut self) {
        self.instruction_pointer += 1;
    }
}

struct ControlUnit {
    memory: MemoryUnit,
    registers: Registers,
    previous_byte: Option<u8>,
    current_byte: Option<u8>,
    previous_instruction: Option<Instruction>,
    current_instruction: Option<Instruction>,
}

impl ControlUnit {
    fn new() -> Self {
        ControlUnit {
            memory: MemoryUnit::new(),
            registers: Registers::new(),
            previous_byte: None,
            current_byte: None,
            previous_instruction: None,
            current_instruction: None,
        }
    }

    fn is_at_end(&self) -> bool {
        return self.registers.instruction_pointer as usize >= self.memory.data.len() - 1;
    }

    fn advance(&mut self) {
        self.registers.advance_instruction_pointer();

        self.previous_byte = self.current_byte;

        let current_byte = self
            .memory
            .read_byte(self.registers.instruction_pointer as usize);
        self.current_byte = Some(*current_byte);
    }

    fn operand_type(&mut self) -> Result<OperandType, &'static str> {
        if let Some(operand_byte) = self.current_byte {
            // Consume operand type byte.
            self.advance();

            return match OperandType::from_byte(&operand_byte) {
                Ok(operand_type) => match operand_type {
                    OperandType::NUMBER => {
                        print!("NUM=");
                        return Ok(OperandType::NUMBER);
                    }
                    OperandType::TEXT => {
                        print!("TXT=");
                        return Ok(OperandType::TEXT);
                    }
                    OperandType::REGISTER => {
                        print!("REG=");
                        return Ok(OperandType::REGISTER);
                    }
                },
                Err(error) => Err(error),
            };
        }

        return Err("No current byte to determine operand type.");
    }

    fn text(&mut self, message: &str) {
        if let Some(length_byte) = self.current_byte {
            // Consume text length byte.
            self.advance();

            let text_length = length_byte as usize;
            let mut text_bytes: Vec<u8> = Vec::new();

            while text_bytes.len() < text_length {
                if let Some(text_byte) = self.current_byte {
                    if !self.is_at_end() {
                        // Consume text byte.
                        self.advance();
                    }

                    text_bytes.push(text_byte);
                } else {
                    panic!("{}", message);
                }
            }

            if let Ok(text) = String::from_utf8(text_bytes) {
                print!("{} ", text);
            } else {
                panic!("{}", message);
            }
        }
    }

    fn register(&mut self, length_byte: bool, message: &str) {
        // Consume register length byte if needed.
        if length_byte {
            self.advance();
        }

        if let Some(register_byte) = self.current_byte {
            if !self.is_at_end() {
                // Consume register byte.
                self.advance();
            }

            print!("r{} ", register_byte);

            return;
        }

        panic!("{}", message);
    }

    fn number(&mut self, message: &str) {
        if let Some(number_byte) = self.current_byte {
            // Consume the number byte.
            self.advance();

            print!("{} ", number_byte);
        } else {
            panic!("{}", message);
        }
    }

    fn debug(&self, message: &str) {
        println!(
            "[{}] IP: {}, Prev Byte: {:02X}, Curr Byte: {:02X}",
            message,
            self.registers.instruction_pointer,
            match self.previous_byte {
                Some(value) => value as i32,
                None => -1,
            },
            match self.current_byte {
                Some(value) => value as i32,
                None => -1,
            }
        );
    }

    fn _move(&mut self) {
        // Consume MOV opcode.
        self.advance();
        print!("MOV: ");

        // Consume the destination register.
        self.register(
            false,
            "Failed to read destination register for MOV instruction.",
        );

        // Consume value operand.
        if let Ok(operand_type) = self.operand_type() {
            match operand_type {
                OperandType::NUMBER => self.number("Failed to read number for MOV instruction."),
                OperandType::TEXT => self.text("Failed to read text for MOV instruction."),
                OperandType::REGISTER => {
                    self.register(true, "Failed to read source register for MOV instruction.")
                }
            }
        } else {
            panic!("Failed to determine operand type for MOV instruction.");
        }

        println!();
    }

    fn subtract(&mut self) {
        // Consume SUB opcode.
        self.advance();
        print!("SUB: ");

        // Consume the first operand.
        if let Ok(operand_type) = self.operand_type() {
            match operand_type {
                OperandType::NUMBER => self.number("Failed to read first number operand for SUB instruction."),
                OperandType::TEXT => self.text("Failed to read first text operand for SUB instruction."),
                OperandType::REGISTER => self.register(
                    true,
                    "Failed to read first register operand for SUB instruction.",
                ),
            }
        } else {
            panic!("Failed to determine first operand type for SUB instruction.");
        }

        if let Ok(operand_type) = self.operand_type() {
            match operand_type {
                OperandType::NUMBER => self.number("Failed to read second number operand for SUB instruction."),
                OperandType::TEXT => self.text("Failed to read second text operand for SUB instruction."),
                OperandType::REGISTER => self.register(
                    true,
                    "Failed to read second register operand for SUB instruction.",
                ),
            }
        } else {
            panic!("Failed to determine second operand type for SUB instruction.");
        }

        self.register(false, "Failed to read destination register for SUB instruction.");

        println!();
    }

    fn addition(&mut self) {
        // Consume ADD opcode.
        self.advance();
        print!("ADD: ");

        if let Ok(operand_type) = self.operand_type() {
            match operand_type {
                OperandType::NUMBER => self.number("Failed to read first number operand for ADD instruction."),
                OperandType::TEXT => self.text("Failed to read first text operand for ADD instruction."),
                OperandType::REGISTER => self.register(true, "Failed to read first register operand for ADD instruction."),
            }
        } else {
            panic!("Failed to determine first operand type for ADD instruction.");
        }

        if let Ok(operand_type) = self.operand_type() {
            match operand_type {
                OperandType::NUMBER => self.number("Failed to read second number operand for ADD instruction."),
                OperandType::TEXT => self.text("Failed to read second text operand for ADD instruction."),
                OperandType::REGISTER => self.register(true, "Failed to read second register operand for ADD instruction."),
            }
        } else {
            panic!("Failed to determine second operand type for ADD instruction.");
        }

        self.register(false, "Failed to read destination register for ADD instruction.");

        println!();
    }

    fn similarity(&mut self) {
        // Consume SIM opcode.
        self.advance();
        print!("SIM: ");

        if let Ok(operand_type) = self.operand_type() {
            match operand_type {
                OperandType::NUMBER => self.number("Failed to read first number operand for SIM instruction."),
                OperandType::TEXT => self.text("Failed to read first text operand for SIM instruction."),
                OperandType::REGISTER => self.register(true, "Failed to read first register operand for SIM instruction."),
            }
        } else {
            panic!("Failed to determine first operand type for SIM instruction.");
        }

        if let Ok(operand_type) = self.operand_type() {
            match operand_type {
                OperandType::NUMBER => self.number("Failed to read second number operand for SIM instruction."),
                OperandType::TEXT => self.text("Failed to read second text operand for SIM instruction."),
                OperandType::REGISTER => self.register(true, "Failed to read second register operand for SIM instruction."),
            }
        } else {
            panic!("Failed to determine second operand type for SIM instruction.");
        }

        self.register(false, "Failed to read destination register for SIM instruction.");

        println!();
    }

    fn jump_less_than(&mut self) {
        // Consume JLT opcode.
        self.advance();
        print!("JLT: ");

        if let Ok(operand_type) = self.operand_type() {
            match operand_type {
                OperandType::NUMBER => self.number("Failed to read first number operand for JLT instruction."),
                OperandType::TEXT => self.text("Failed to read first text operand for JLT instruction."),
                OperandType::REGISTER => self.register(true, "Failed to read first register operand for JLT instruction."),
            }
        } else {
            panic!("Failed to determine first operand type for JLT instruction.");
        }

        if let Ok(operand_type) = self.operand_type() {
            match operand_type {
                OperandType::NUMBER => self.number("Failed to read second number operand for JLT instruction."),
                OperandType::TEXT => self.text("Failed to read second text operand for JLT instruction."),
                OperandType::REGISTER => self.register(true, "Failed to read second register operand for JLT instruction."),
            }
        } else {
            panic!("Failed to determine second operand type for JLT instruction.");
        }

        self.register(false, "Failed to read destination register for JLT instruction.");

        println!();
    }

    fn op_code(&mut self) {
        if let Some(current_byte) = &self.current_byte {
            match OpCode::from_byte(current_byte) {
                Ok(op_code) => match op_code {
                    OpCode::MOV => self._move(),
                    OpCode::ADD => self.addition(),
                    OpCode::SUB => self.subtract(),
                    OpCode::SIM => self.similarity(),
                    OpCode::JLT => self.jump_less_than(),
                    _ => panic!("Opcode not implemented yet. Byte: {:02X}", current_byte),
                },
                Err(error) => panic!("{} Byte: {:02X}", error, current_byte),
            }
        }
    }

    fn fetch(&mut self) {
        let current_byte = self
            .memory
            .read_byte(self.registers.instruction_pointer as usize);
        self.current_byte = Some(*current_byte);

        while !self.is_at_end() {
            self.op_code();
        }
    }
}

struct SemanticLogicUnit {}

pub struct Processor {
    control: ControlUnit,
    semantic_logic: SemanticLogicUnit,
}

impl Processor {
    pub fn new() -> Self {
        Processor {
            control: ControlUnit::new(),
            semantic_logic: SemanticLogicUnit {},
        }
    }

    pub fn load_bytecode(&mut self, bytecode: Vec<u8>) {
        self.control.memory.load(bytecode);
    }

    pub fn execute(&mut self) {
        self.control.fetch();
    }
}
