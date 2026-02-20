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

    pub fn load(&mut self, data: Vec<u8>) {
        let byte_code: Vec<[u8; 4]> = data
            .chunks(4)
            .map(|chunk| {
                chunk
                    .try_into()
                    .expect("Byte code chunks must be exactly 4 bytes")
            })
            .collect();

        self.control.load_byte_code(byte_code);
    }

    pub fn run(&mut self, debug: bool) {
        while let Some(instruction) = self.control.fetch_and_decode() {
            self.control.execute(&instruction, debug);
        }
    }
}
