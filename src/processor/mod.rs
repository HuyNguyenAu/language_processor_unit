use std::sync::{Arc, Mutex};

use crate::processor::{control_unit::ControlUnit, memory::Memory, registers::Registers};

mod control_unit;
mod memory;
mod registers;

pub struct Processor {
    memory: Arc<Mutex<Memory>>,
    registers: Arc<Mutex<Registers>>,
    control_unit: ControlUnit,
}

impl Processor {
    pub fn new() -> Self {
        let memory = Arc::new(Mutex::new(Memory::new()));
        let registers = Arc::new(Mutex::new(Registers::new()));

        Processor {
            memory: Arc::clone(&memory),
            registers: Arc::clone(&registers),
            control_unit: ControlUnit::new(&memory, &registers),
        }
    }

    pub fn load(&mut self, data: Vec<u8>) {
        if !data.len().is_multiple_of(4) {
            panic!(
                "Invalid bytecode length: {}. Bytecode must be a multiple of 4 bytes.",
                data.len()
            );
        }

        let byte_code: Vec<[u8; 4]> = data
            .chunks(4)
            .map(|chunk| {
                chunk
                    .try_into()
                    .expect("Byte code chunks must be exactly 4 bytes")
            })
            .collect();

        self.control_unit.load(byte_code);
    }

    pub fn run(&mut self, debug: bool) {
        while self.control_unit.fetch() {
            let instruction = self.control_unit.decode();
            println!("Fetched instruction: {:?}", instruction);
            // self.control_unit.execute(instruction);
        }
    }
}
