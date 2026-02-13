#[derive(Debug, Clone)]
pub enum Value {
    Text(String),
    Number(u32),
    None,
}

pub struct Registers {
    general_purpose_registers: [Value; 32],
    instruction_pointer: usize,
}

impl Registers {
    pub fn new() -> Self {
        Registers {
            general_purpose_registers: [const { Value::None }; 32],
            instruction_pointer: 0,
        }
    }

    pub fn get_register(&self, register_number: u32) -> &Value {
        let register_number =
            usize::try_from(register_number).expect("Failed to convert register number to usize.");

        if register_number -1 > 32 {
            panic!(
                "Invalid register number: {}. Valid register numbers are 0-32.",
                register_number
            );
        }

        return match self.general_purpose_registers.get(register_number - 1) {
            Some(value) => value,
            None => panic!(
                "Invalid register number: {}. Valid register numbers are 0-32.",
                register_number
            ),
        };
    }

    pub fn set_register(&mut self, register_number: u32, value: Value) {
        let register_number =
            usize::try_from(register_number).expect("Failed to convert register number to usize.");

        if register_number - 1 > 31 {
            panic!(
                "Invalid register number: {}. Valid register numbers are 0-32.",
                register_number
            );
        }

        match register_number {
            0..=31 => self.general_purpose_registers[register_number - 1] = value,
            _ => panic!(
                "Invalid register number: {}. Valid register numbers are 0-32.",
                register_number
            ),
        }
    }

    pub fn get_instruction_pointer(&self) -> usize {
        self.instruction_pointer
    }

    pub fn set_instruction_pointer(&mut self, address: usize) {
        self.instruction_pointer = address;
    }

    pub fn advance_instruction_pointer(&mut self) {
        self.instruction_pointer += 1;
    }
}
