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
        return match self.data.get(address) {
            Some(byte) => byte,
            None => panic!("Address out of bounds."),
        };
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
    current_instruction: Option<Instruction>,
}

impl ControlUnit {
    fn new() -> Self {
        ControlUnit {
            memory: MemoryUnit::new(),
            registers: Registers::new(),
            previous_byte: None,
            current_byte: None,
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
        let operand_byte = match self.current_byte {
            Some(byte) => byte,
            None => return Err("No current byte to determine operand type."),
        };

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

    fn text(&mut self, message: &str) {
        let mut text_length: usize = 0;

        if let Some(length_byte) = self.current_byte {
            // Consume text length byte.
            self.advance();

            text_length = length_byte as usize;
        }

        let mut text_bytes: Vec<u8> = Vec::new();

        while text_bytes.len() < text_length
            && let Some(text_byte) = self.current_byte
        {
            if !self.is_at_end() {
                // Consume text byte.
                self.advance();
            }

            text_bytes.push(text_byte);
        }

        match String::from_utf8(text_bytes) {
            Ok(text) => print!("{} ", text),
            Err(_) => panic!("{}", message),
        }
    }

    fn register(&mut self, length_byte: bool, message: &str) {
        // Consume register length byte if needed.
        if length_byte {
            self.advance();
        }

        let register_byte = match self.current_byte {
            Some(byte) => byte,
            None => panic!("{}", message),
        };

        if !self.is_at_end() {
            // Consume register byte.
            self.advance();
        }

        print!("r{} ", register_byte);
    }

    fn number(&mut self, length_byte: bool, message: &str) {
        // Consume number length byte if needed.
        if length_byte {
            self.advance();
        }

        let number_byte = match self.current_byte {
            Some(byte) => byte,
            None => panic!("{}", message),
        };

        if !self.is_at_end() {
            // Consume number byte.
            self.advance();
        }

        print!("{} ", number_byte);
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

    // fn make_instruction(&mut self, op_code: OpCode) {
    //     self.current_instruction = match self.current_instruction {
    //         Some(_) => panic!("Current instruction should have been consumed."),
    //         None => Some(Instruction::new(op_code)),
    //     };
    // }

    fn _move(&mut self) {
        // Consume MOV opcode.
        self.advance();
        print!("MOV: ");

        // self.make_instruction(OpCode::MOV);

        // Consume the destination register.
        self.register(
            false,
            "Failed to read destination register for MOV instruction.",
        );

        // Consume value operand.
        let operand_type = match self.operand_type() {
            Ok(operand_type) => operand_type,
            Err(error) => panic!(
                "Failed to determine operand type for MOV instruction. {}",
                error
            ),
        };

        match operand_type {
            OperandType::NUMBER => self.number(true, "Failed to read number for MOV instruction."),
            OperandType::TEXT => self.text("Failed to read text for MOV instruction."),
            OperandType::REGISTER => {
                self.register(true, "Failed to read source register for MOV instruction.")
            }
        }

        println!();
    }

    fn subtract(&mut self) {
        // Consume SUB opcode.
        self.advance();
        print!("SUB: ");

        // Consume the first operand.
        let first_operand_type = match self.operand_type() {
            Ok(operand_type) => operand_type,
            Err(error) => panic!(
                "Failed to determine first operand type for SUB instruction. {}",
                error
            ),
        };

        match first_operand_type {
            OperandType::NUMBER => self.number(
                true,
                "Failed to read first number operand for SUB instruction.",
            ),
            OperandType::TEXT => {
                self.text("Failed to read first text operand for SUB instruction.")
            }
            OperandType::REGISTER => self.register(
                true,
                "Failed to read first register operand for SUB instruction.",
            ),
        }

        // Consume the second operand.
        let second_operand_type = match self.operand_type() {
            Ok(operand_type) => operand_type,
            Err(error) => panic!(
                "Failed to determine second operand type for SUB instruction. {}",
                error
            ),
        };

        match second_operand_type {
            OperandType::NUMBER => self.number(
                true,
                "Failed to read second number operand for SUB instruction.",
            ),
            OperandType::TEXT => {
                self.text("Failed to read second text operand for SUB instruction.")
            }
            OperandType::REGISTER => self.register(
                true,
                "Failed to read second register operand for SUB instruction.",
            ),
        }

        self.register(
            false,
            "Failed to read destination register for SUB instruction.",
        );

        println!();
    }

    fn addition(&mut self) {
        // Consume ADD opcode.
        self.advance();
        print!("ADD: ");

        // Consume the first operand.
        let first_operand_type = match self.operand_type() {
            Ok(operand_type) => operand_type,
            Err(error) => panic!(
                "Failed to determine first operand type for ADD instruction. {}",
                error
            ),
        };

        match first_operand_type {
            OperandType::NUMBER => self.number(
                true,
                "Failed to read first number operand for ADD instruction.",
            ),
            OperandType::TEXT => {
                self.text("Failed to read first text operand for ADD instruction.")
            }
            OperandType::REGISTER => self.register(
                true,
                "Failed to read first register operand for ADD instruction.",
            ),
        }

        // Consume the second operand.
        let second_operand_type = match self.operand_type() {
            Ok(operand_type) => operand_type,
            Err(error) => panic!(
                "Failed to determine second operand type for ADD instruction. {}",
                error
            ),
        };

        match second_operand_type {
            OperandType::NUMBER => self.number(
                true,
                "Failed to read second number operand for ADD instruction.",
            ),
            OperandType::TEXT => {
                self.text("Failed to read second text operand for ADD instruction.")
            }
            OperandType::REGISTER => self.register(
                true,
                "Failed to read second register operand for ADD instruction.",
            ),
        }

        self.register(
            false,
            "Failed to read destination register for ADD instruction.",
        );

        println!();
    }

    fn similarity(&mut self) {
        // Consume SIM opcode.
        self.advance();
        print!("SIM: ");

        // Consume the first operand.
        let first_operand_type = match self.operand_type() {
            Ok(operand_type) => operand_type,
            Err(error) => panic!(
                "Failed to determine first operand type for SIM instruction. {}",
                error
            ),
        };

        match first_operand_type {
            OperandType::NUMBER => self.number(
                true,
                "Failed to read first number operand for SIM instruction.",
            ),
            OperandType::TEXT => {
                self.text("Failed to read first text operand for SIM instruction.")
            }
            OperandType::REGISTER => self.register(
                true,
                "Failed to read first register operand for SIM instruction.",
            ),
        }

        // Consume the second operand.
        let second_operand_type = match self.operand_type() {
            Ok(operand_type) => operand_type,
            Err(error) => panic!(
                "Failed to determine second operand type for SIM instruction. {}",
                error
            ),
        };

        match second_operand_type {
            OperandType::NUMBER => self.number(
                true,
                "Failed to read second number operand for SIM instruction.",
            ),
            OperandType::TEXT => {
                self.text("Failed to read second text operand for SIM instruction.")
            }
            OperandType::REGISTER => self.register(
                true,
                "Failed to read second register operand for SIM instruction.",
            ),
        }

        self.register(
            false,
            "Failed to read destination register for SIM instruction.",
        );

        println!();
    }

    fn jump_less_than(&mut self) {
        // Consume JLT opcode.
        self.advance();
        print!("JLT: ");

        // Consume the first operand.
        let first_operand_type = match self.operand_type() {
            Ok(operand_type) => operand_type,
            Err(error) => panic!(
                "Failed to determine first operand type for JLT instruction. {}",
                error
            ),
        };

        match first_operand_type {
            OperandType::NUMBER => self.number(
                true,
                "Failed to read first number operand for JLT instruction.",
            ),
            OperandType::TEXT => {
                self.text("Failed to read first text operand for JLT instruction.")
            }
            OperandType::REGISTER => self.register(
                true,
                "Failed to read first register operand for JLT instruction.",
            ),
        }

        // Consume the second operand.
        let second_operand_type = match self.operand_type() {
            Ok(operand_type) => operand_type,
            Err(error) => panic!(
                "Failed to determine second operand type for JLT instruction. {}",
                error
            ),
        };

        match second_operand_type {
            OperandType::NUMBER => self.number(
                true,
                "Failed to read second number operand for JLT instruction.",
            ),
            OperandType::TEXT => {
                self.text("Failed to read second text operand for JLT instruction.")
            }
            OperandType::REGISTER => self.register(
                true,
                "Failed to read second register operand for JLT instruction.",
            ),
        }

        self.number(
            false,
            "Failed to read byte code jump index for JLT instruction.",
        );

        println!();
    }

    fn op_code(&mut self) {
        let current_byte = match self.current_byte {
            Some(byte) => byte,
            None => panic!("No current byte to determine opcode."),
        };

        match OpCode::from_byte(&current_byte) {
            Ok(op_code) => match op_code {
                OpCode::MOV => self._move(),
                OpCode::ADD => self.addition(),
                OpCode::SUB => self.subtract(),
                OpCode::SIM => self.similarity(),
                OpCode::JLT => self.jump_less_than(),
            },
            Err(error) => panic!("{} Byte: {:02X}", error, current_byte),
        }
    }

    fn fetch(&mut self) {
        // Initialise current byte.
        let current_byte = self
            .memory
            .read_byte(self.registers.instruction_pointer as usize);
        self.current_byte = Some(*current_byte);

        loop {
            if self.is_at_end() {
                break;
            }

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
