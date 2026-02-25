use std::sync::{Arc, Mutex, MutexGuard};

use crate::processor::{
    control_unit::{decoder::Decoder, instruction::Instruction},
    memory::Memory,
    registers::Registers,
};

mod decoder;
// mod executer;
mod instruction;

pub struct ControlUnit {
    memory: Arc<Mutex<Memory>>,
    registers: Arc<Mutex<Registers>>,
    decoder: Decoder,
    // executer: Executer,
}

impl ControlUnit {
    pub fn new(memory: &Arc<Mutex<Memory>>, registers: &Arc<Mutex<Registers>>) -> Self {
        ControlUnit {
            memory: Arc::clone(&memory),
            registers: Arc::clone(&registers),
            decoder: Decoder::new(memory, registers),
            // executer: Executer::new(),
        }
    }

    fn memory_lock(&self) -> MutexGuard<'_, Memory> {
        match self.memory.lock() {
            Ok(memory) => memory,
            Err(error) => {
                panic!("Failed to access memory: memory lock error: {}", error);
            }
        }
    }

    fn registers_lock(&self) -> MutexGuard<'_, Registers> {
        match self.registers.lock() {
            Ok(registers) => registers,
            Err(error) => {
                panic!(
                    "Failed to access registers: registers lock error: {}",
                    error
                );
            }
        }
    }

    fn read_instruction(&mut self) -> Result<[[u8; 4]; 4], String> {
        let memory = self.memory_lock();
        let registers = self.registers_lock();

        let mut instruction_bytes: [[u8; 4]; 4] = [[0; 4], [0; 4], [0; 4], [0; 4]];
        let mut address = registers.get_instruction_pointer();

        for i in 0..4 {
            match memory.read(address) {
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

        let mut memory = self.memory_lock();
        let mut registers = self.registers_lock();

        memory.load(byte_code);

        registers.set_instruction_pointer(instruction_section_pointer);
        registers.set_instruction(None);

        registers.set_instruction_section_pointer(instruction_section_pointer);
        registers.set_data_section_pointer(data_section_pointer);
    }

    pub fn fetch(&mut self) -> bool {
        let should_continue = {
            let registers = self.registers_lock();

            registers.get_instruction_pointer() < registers.get_data_section_pointer()
        };

        if !should_continue {
            return false;
        }

        let instruction_bytes = match self.read_instruction() {
            Ok(bytes) => bytes,
            Err(error) => panic!("Failed to fetch instruction: {}", error),
        };

        let mut registers = self.registers_lock();

        registers.set_instruction(Some(instruction_bytes));
        registers.advance_instruction_pointer(4);

        true
    }

    pub fn decode(&mut self) -> Instruction {
        let instruction_bytes = {
            let registers = self.registers_lock();

            match registers.get_instruction() {
                Some(bytes) => bytes,
                None => panic!("Failed to decode instruction: no instruction loaded."),
            }
        };

        self.decoder.decode(instruction_bytes)
    }

    pub fn execute(&mut self, instruction: Instruction) {}
}
