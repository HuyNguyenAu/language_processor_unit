use crate::instruction::{
    self, AddInstruction, Instruction, JumpLessThanInstruction, MoveInstruction, OpCode, Operand,
    OperandType, SimilarityInstruction, SubInstruction,
};

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

    fn read_byte(&self, address: u8) -> &u8 {
        return match self.data.get(address as usize) {
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
}

impl ControlUnit {
    fn new() -> Self {
        ControlUnit {
            memory: MemoryUnit::new(),
            registers: Registers::new(),
            previous_byte: None,
            current_byte: None,
        }
    }

    fn is_at_end(&self) -> bool {
        return self.registers.instruction_pointer as usize >= self.memory.data.len() - 1;
    }

    fn advance(&mut self) {
        self.registers.advance_instruction_pointer();

        self.previous_byte = self.current_byte;

        let current_byte = self.memory.read_byte(self.registers.instruction_pointer);
        self.current_byte = Some(*current_byte);
    }

    fn operand_type(&mut self, message: &str) -> OperandType {
        let operand_byte = match self.current_byte {
            Some(byte) => byte,
            None => panic!("No current byte to determine operand type."),
        };

        // Consume operand type byte.
        self.advance();

        let operand_type = match OperandType::from_byte(&operand_byte) {
            Ok(operand_type) => operand_type,
            Err(error) => panic!("{} {} Byte: {:02X}", message, error, operand_byte),
        };

        return operand_type;
    }

    fn text(&mut self, message: &str) -> String {
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
            Ok(text) => return text,
            Err(_) => panic!("{}", message),
        }
    }

    fn register(&mut self, length_byte: bool, message: &str) -> u8 {
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

        return register_byte;
    }

    fn number(&mut self, length_byte: bool, message: &str) -> u8 {
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

        return number_byte;
    }

    fn operand(
        &mut self,
        operand_type_message: &str,
        operand_number: &str,
        operand_text: &str,
        operand_register: &str,
    ) -> Operand {
        let operand_type = self.operand_type(operand_type_message);

        return match operand_type {
            OperandType::NUMBER => Operand::Number(self.number(true, operand_number)),
            OperandType::TEXT => Operand::Text(self.text(operand_text)),
            OperandType::REGISTER => Operand::Register(self.register(true, operand_register)),
        };
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

    fn _move(&mut self) -> MoveInstruction {
        // Consume MOV opcode.
        self.advance();

        // Consume the destination register.
        let destination_register = self.register(
            false,
            "Failed to read destination register for MOV instruction.",
        );

        // Consume value operand.
        let value = self.operand(
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

    fn subtract(&mut self) -> SubInstruction {
        // Consume SUB opcode.
        self.advance();

        // Consume the first operand.
        let first_operand = self.operand(
            "Failed to determine first operand type for SUB instruction.",
            "Failed to read first number operand for SUB instruction.",
            "Failed to read first text operand for SUB instruction.",
            "Failed to read first register operand for SUB instruction.",
        );

        // Consume the second operand.
        let second_operand = self.operand(
            "Failed to determine second operand type for SUB instruction.",
            "Failed to read second number operand for SUB instruction.",
            "Failed to read second text operand for SUB instruction.",
            "Failed to read second register operand for SUB instruction.",
        );

        // Consume the destination register.
        let destination_register = self.register(
            false,
            "Failed to read destination register for SUB instruction.",
        );

        return SubInstruction {
            destination_register,
            first_operand,
            second_operand,
        };
    }

    fn addition(&mut self) -> AddInstruction {
        // Consume ADD opcode.
        self.advance();

        // Consume the first operand.
        let first_operand = self.operand(
            "Failed to determine first operand type for ADD instruction.",
            "Failed to read first number operand for ADD instruction.",
            "Failed to read first text operand for ADD instruction.",
            "Failed to read first register operand for ADD instruction.",
        );

        // Consume the second operand.
        let second_operand = self.operand(
            "Failed to determine second operand type for ADD instruction.",
            "Failed to read second number operand for ADD instruction.",
            "Failed to read second text operand for ADD instruction.",
            "Failed to read second register operand for ADD instruction.",
        );

        // Consume the destination register.
        let destination_register = self.register(
            false,
            "Failed to read destination register for ADD instruction.",
        );

        return AddInstruction {
            destination_register,
            first_operand,
            second_operand,
        };
    }

    fn similarity(&mut self) -> SimilarityInstruction {
        // Consume SIM opcode.
        self.advance();

        // Consume the first operand.
        let first_operand = self.operand(
            "Failed to determine first operand type for SIM instruction.",
            "Failed to read first number operand for SIM instruction.",
            "Failed to read first text operand for SIM instruction.",
            "Failed to read first register operand for SIM instruction.",
        );

        // Consume the second operand.
        let second_operand = self.operand(
            "Failed to determine second operand type for SIM instruction.",
            "Failed to read second number operand for SIM instruction.",
            "Failed to read second text operand for SIM instruction.",
            "Failed to read second register operand for SIM instruction.",
        );

        // Consume the destination register.
        let destination_register = self.register(
            false,
            "Failed to read destination register for SIM instruction.",
        );

        return SimilarityInstruction {
            destination_register,
            first_operand,
            second_operand,
        };
    }

    fn jump_less_than(&mut self) -> JumpLessThanInstruction {
        // Consume JLT opcode.
        self.advance();

        // Consume the first operand.
        let first_operand = self.operand(
            "Failed to determine first operand type for JLT instruction.",
            "Failed to read first number operand for JLT instruction.",
            "Failed to read first text operand for JLT instruction.",
            "Failed to read first register operand for JLT instruction.",
        );

        // Consume the second operand.
        let second_operand = self.operand(
            "Failed to determine second operand type for JLT instruction.",
            "Failed to read second number operand for JLT instruction.",
            "Failed to read second text operand for JLT instruction.",
            "Failed to read second register operand for JLT instruction.",
        );

        // Consume the bytecode jump index.
        let bytecode_jump_index = self.number(
            false,
            "Failed to read byte code jump index for JLT instruction.",
        );

        return JumpLessThanInstruction {
            bytecode_jump_index,
            first_operand,
            second_operand,
        };
    }

    fn op_code(&mut self) -> Instruction {
        let current_byte = match self.current_byte {
            Some(byte) => byte,
            None => panic!("No current byte to determine opcode."),
        };
        let op_code = match OpCode::from_byte(&current_byte) {
            Ok(op_code) => op_code,
            Err(error) => panic!("{} Byte: {:02X}", error, current_byte),
        };

        return match op_code {
            OpCode::MOV => Instruction::Move(self._move()),
            OpCode::ADD => Instruction::Add(self.addition()),
            OpCode::SUB => Instruction::Sub(self.subtract()),
            OpCode::SIM => Instruction::Similarity(self.similarity()),
            OpCode::JLT => Instruction::JumpLessThan(self.jump_less_than()),
        };
    }

    fn fetch(&mut self) -> Option<Instruction> {
        // Initialise current byte.
        let current_byte = self.memory.read_byte(self.registers.instruction_pointer);
        self.current_byte = Some(*current_byte);

        if self.is_at_end() {
            return None;
        }

        return Some(self.op_code());
    }

    fn decode(&mut self, instruction: &Instruction) {
        match instruction {
            Instruction::Move(mov_instruction) => {
                println!(
                    "MOV: {:?} -> r{}",
                    mov_instruction.value, mov_instruction.destination_register
                );
            }
            Instruction::Add(add_instruction) => {
                println!(
                    "ADD: {:?} + {:?} -> r{}",
                    add_instruction.first_operand,
                    add_instruction.second_operand,
                    add_instruction.destination_register
                );
            }
            Instruction::Sub(sub_instruction) => {
                println!(
                    "SUB: {:?} - {:?} -> r{}",
                    sub_instruction.first_operand,
                    sub_instruction.second_operand,
                    sub_instruction.destination_register
                );
            }
            Instruction::Similarity(sim_instruction) => {
                println!(
                    "SIM: {:?} ~ {:?} -> r{}",
                    sim_instruction.first_operand,
                    sim_instruction.second_operand,
                    sim_instruction.destination_register
                );
            }
            Instruction::JumpLessThan(jlt_instruction) => {
                println!(
                    "JLT: {:?} < {:?} -> {}",
                    jlt_instruction.first_operand,
                    jlt_instruction.second_operand,
                    jlt_instruction.bytecode_jump_index
                );
            }
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
        while let Some(instruction) = self.control.fetch() {
            self.control.decode(&instruction);
        }
    }
}
