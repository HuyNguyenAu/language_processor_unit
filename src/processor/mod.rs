use crate::processor::control_unit::ControlUnit;

mod control_unit;

pub struct Processor {
    control: ControlUnit,
}

impl Processor {
    pub fn new() -> Self {
        Processor {
            control: ControlUnit::new(),
        }
    }

    pub fn load(&mut self, bytecode: Vec<u8>) {
        self.control.load_bytecode(bytecode);
    }

    pub fn run(&mut self) {
        while let Some(instruction) = self.control.fetch_and_decode() {
            self.control.execute(&instruction);
        }
    }
}
