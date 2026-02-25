use crate::processor::control_unit::{
    decoder::Decoder, instruction::Instruction, memory::Memory, registers::Registers,
};

mod decoder;
// mod executer;
mod instruction;
mod language_logic_unit;
mod memory;
mod registers;

pub struct ControlUnit {
    memory: Memory,
    registers: Registers,
    decoder: Decoder,
    // execution_unit: ExecutionUnit,
}

impl ControlUnit {
    pub fn new() -> Self {
        ControlUnit {
            memory: Memory::new(),
            registers: Registers::new(),
            decoder: Decoder::new(),
            // execution_unit: ExecutionUnit::new(registers, memory),
        }
    }

    fn read_instruction(&mut self) -> Result<[[u8; 4]; 4], String> {
        let mut instruction_bytes: [[u8; 4]; 4] = [[0; 4], [0; 4], [0; 4], [0; 4]];
        let mut address = self.registers.get_instruction_pointer();

        for i in 0..4 {
            match self.memory.read(address) {
                Ok(bytes) => instruction_bytes[i] = *bytes,
                Err(error) => {
                    return Err(format!(
                        "Failed to read instruction: memory read error at address {}: {}",
                        address, error
                    ));
                }
            }

            address += 1;
        }

        Ok(instruction_bytes)
    }

    fn header_pointer(&mut self, index: usize, byte_code: &Vec<[u8; 4]>) -> usize {
        let pointer_bytes = match byte_code.get(index) {
            Some(bytes) => bytes,
            None => panic!("Failed to read header pointer from memory."),
        };

        match u32::from_be_bytes(*pointer_bytes).try_into() {
            Ok(pointer) => pointer,
            Err(error) => panic!(
                "Failed to decode header pointer from byte code. Error: {}. Byte code: {:?}.",
                error, pointer_bytes
            ),
        }
    }

    pub fn load(&mut self, byte_code: Vec<[u8; 4]>) {
        let instruction_section_pointer = self.header_pointer(0, &byte_code);
        let data_section_pointer = self.header_pointer(1, &byte_code);

        println!(
            "Loading byte code. Data section starts at address {}, instruction section starts at address {}.",
            data_section_pointer, instruction_section_pointer
        );

        self.memory.load(byte_code);

        self.registers
            .set_instruction_pointer(instruction_section_pointer);
        self.registers.set_instruction(None);

        self.registers
            .set_instruction_section_pointer(instruction_section_pointer);
        self.registers
            .set_data_section_pointer(data_section_pointer);
    }

    pub fn fetch(&mut self) -> bool {
        if self.registers.get_instruction_pointer() >= self.registers.get_data_section_pointer() {
            return false;
        }

        let instruction_bytes = match self.read_instruction() {
            Ok(bytes) => bytes,
            Err(error) => panic!("Failed to fetch instruction: {}", error),
        };

        self.registers.set_instruction(Some(instruction_bytes));
        self.registers
            .set_instruction_pointer(self.registers.get_instruction_pointer() + 4);

        true
    }

    pub fn decode(&mut self) -> Instruction {
        let instruction_bytes = match self.registers.get_instruction() {
            Some(bytes) => bytes,
            None => panic!("Failed to decode instruction: no instruction loaded."),
        };

        self.decoder.decode(instruction_bytes)
    }
}
