use crate::processor::control_unit::decoder::Decoder;
use crate::processor::control_unit::executor::Executor;
use crate::processor::{memory::Memory, registers::Registers};

use crate::processor::control_unit::instruction::Instruction;

mod decoder;
mod executor;
mod instruction;
mod language_logic_unit;
mod utils;
mod roles;

pub struct ControlUnit {
    memory: Memory,
    registers: Registers,
}

impl ControlUnit {
    pub fn new() -> Self {
        ControlUnit {
            memory: Memory::new(),
            registers: Registers::new(),
        }
    }

    fn read_instruction(&self) -> Result<[[u8; 4]; 4], String> {
        let instruction_pointer = self.registers.get_instruction_pointer();
        let mut buffer = [[0u8; 4]; 4];

        for (i, slot) in buffer.iter_mut().enumerate() {
            *slot = *self.memory.read(instruction_pointer + i).map_err(|error| {
                format!(
                    "Failed to read instruction at {}: {}",
                    instruction_pointer + i,
                    error
                )
            })?;
        }

        Ok(buffer)
    }

    fn header_pointer(&self, index: usize, byte_code: &[[u8; 4]]) -> usize {
        let pointer_bytes = byte_code.get(index).expect("Missing header pointer");
        u32::from_be_bytes(*pointer_bytes)
            .try_into()
            .expect("Header pointer did not fit in usize")
    }

    pub fn load(&mut self, byte_code: Vec<[u8; 4]>) {
        let instruction_section_pointer = self.header_pointer(0, &byte_code);
        let data_section_pointer = self.header_pointer(1, &byte_code);

        self.memory.load(byte_code);

        self.registers
            .set_instruction_pointer(instruction_section_pointer);
        self.registers.set_instruction(None);
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
        self.registers.advance_instruction_pointer(4);

        true
    }

    pub fn decode(&self) -> Instruction {
        let bytes = self
            .registers
            .get_instruction()
            .expect("Failed to decode instruction: no instruction loaded.");

        Decoder::decode(&self.memory, &self.registers, bytes)
    }

    pub fn execute(&mut self, instruction: Instruction, debug: bool) {
        Executor::execute(&mut self.memory, &mut self.registers, &instruction, debug);
    }
}
