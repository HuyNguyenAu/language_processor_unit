use crate::instruction::{Instruction, OpCode, OperandType};

struct MemoryUnit {
    data: Option<Vec<u8>>,
}

impl MemoryUnit {
    fn new() -> Self {
        MemoryUnit { data: None }
    }

    fn load(&mut self, bytecode: Vec<u8>) {
        self.data = Some(bytecode);
    }

    fn read_byte(&self, address: usize) -> &u8 {
        if let Some(data) = &self.data {
            if let Some(byte) = data.get(address) {
                return byte;
            }

            panic!("Address out of bounds.");
        }

        panic!("No bytecode loaded in memory.");
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
    fn advance_instruction_pointer(&mut self) {
        self.instruction_pointer += 1;
    }
}

struct ControlUnit {
    memory: MemoryUnit,
    registers: Registers,
    previous: Option<u8>,
    current: Option<u8>,
}

impl ControlUnit {
    fn advance(&mut self) {
        self.previous = self.current;

        let current_byte = self
            .memory
            .read_byte(self.registers.instruction_pointer as usize);
        self.current = Some(*current_byte);

        self.registers.advance_instruction_pointer();
    }

    fn operand_type(&mut self) -> Result<OperandType, &'static str> {
        // Consume operand type byte.
        self.advance();

        if let Some(operand_byte) = &self.current {
            return match OperandType::from_byte(operand_byte) {
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

    fn text(&mut self) {
        // Consume text length byte.
        self.advance();

        if let Some(length_byte) = &self.current {
            let text_length = *length_byte as usize;
            let mut text_bytes: Vec<u8> = Vec::new();

            for _ in 0..text_length + 1 {
                self.advance();

                if let Some(current) = &self.current {
                    text_bytes.push(current.clone());
                } else {
                    panic!("Text byte expected but not found.");
                }
            }

            if let Ok(text) = String::from_utf8(text_bytes) {
                print!("{} ", text);
            } else {
                panic!("Failed to decode text from bytes.");
            }
        }
    }

    fn register(&mut self) {
        // Consume the register byte.
        if let Some(current) = &self.current {
            print!("r{} ", current);
        }
    }

    fn _move(&mut self) {
        // Consume MOV opcode.
        self.advance();
        print!("MOV: ");

        self.register();

        if let Ok(operand_type) = self.operand_type() {
            match operand_type {
                OperandType::NUMBER => {}
                OperandType::TEXT => self.text(),
                OperandType::REGISTER => self.register(),
            }
        } else {
            panic!("Failed to determine operand type for MOV instruction.");
        }

        println!();
    }

    fn op_code(&mut self) {
        if let Some(current_byte) = &self.current {
            match OpCode::from_byte(current_byte) {
                Ok(op_code) => match op_code {
                    OpCode::MOV => self._move(),
                    // OpCode::ADD => println!("ADD instruction"),
                    // OpCode::SUB => println!("SUB instruction"),
                    // OpCode::SIM => println!("SIM instruction"),
                    // OpCode::JLT => println!("JLT instruction"),
                    _ => panic!("Opcode not implemented yet. Byte: {:02X}", current_byte),
                },
                Err(error) => panic!("{} Byte: {:02X}", error, current_byte),
            }
        }
    }

    fn load_instruction(&mut self) {
        self.advance();

        loop {
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
            control: ControlUnit {
                memory: MemoryUnit { data: None },
                registers: Registers {
                    register_1: 0,
                    register_2: 0,
                    register_3: 0,
                    register_4: 0,
                    register_5: 0,
                    register_6: 0,
                    instruction_pointer: 0,
                },
                previous: None,
                current: None,
            },
            semantic_logic: SemanticLogicUnit {},
        }
    }

    pub fn load_bytecode(&mut self, bytecode: Vec<u8>) {
        self.control.memory.load(bytecode);
    }

    pub fn execute(&mut self) {
        self.control.load_instruction();
    }
}
