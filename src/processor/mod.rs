use crate::processor::control_unit::ControlUnit;

mod control_unit;
mod memory;
mod registers;

pub struct Processor {
    control_unit: ControlUnit,
}

impl Processor {
    pub fn new() -> Self {
        Processor {
            control_unit: ControlUnit::new(),
        }
    }

    pub fn load(&mut self, data: Vec<u8>) -> Result<(), String> {
        if !data.len().is_multiple_of(4) {
            return Err(format!(
                "Invalid bytecode length: {}. Bytecode must be a multiple of 4 bytes.",
                data.len()
            ));
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
        Ok(())
    }

    pub fn run(&mut self, debug: bool) {
        while self.control_unit.fetch() {
            let instruction = self.control_unit.decode();
            self.control_unit.execute(instruction, debug);
        }
    }
}
