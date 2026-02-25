use crate::processor::control_unit::decoder::Decoder;
use crate::processor::control_unit::executer::Executer;
use crate::processor::{memory::Memory, registers::Registers};

use crate::processor::control_unit::instruction::Instruction;

mod decoder;
mod executer;
mod instruction;
mod language_logic_unit;

#[derive(Debug)]
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
        let mut instruction_bytes: [[u8; 4]; 4] = [[0; 4]; 4];
        let mut address = self.registers.get_instruction_pointer();

        for slot in instruction_bytes.iter_mut() {
            match self.memory.read(address) {
                Ok(bytes) => *slot = *bytes,
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

    fn header_pointer(&self, index: usize, byte_code: &[[u8; 4]]) -> usize {
        let pointer_bytes = byte_code
            .get(index)
            .unwrap_or_else(|| panic!("Failed to read header pointer from memory."));

        u32::from_be_bytes(*pointer_bytes)
            .try_into()
            .unwrap_or_else(|error| {
                panic!(
                    "Failed to decode header pointer from byte code. Error: {}. Byte code: {:?}.",
                    error, pointer_bytes
                )
            })
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
        Executer::execute(&mut self.memory, &mut self.registers, &instruction, debug);
    }
}
