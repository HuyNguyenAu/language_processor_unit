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
                return [
                    chunk
                        .get(0)
                        .cloned()
                        .expect("Byte code chunk is missing the first byte"),
                    chunk
                        .get(1)
                        .cloned()
                        .expect("Byte code chunk is missing the second byte"),
                    chunk
                        .get(2)
                        .cloned()
                        .expect("Byte code chunk is missing the third byte"),
                    chunk
                        .get(3)
                        .cloned()
                        .expect("Byte code chunk is missing the fourth byte"),
                ];
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
