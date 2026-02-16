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

    pub fn get_register(&self, register_number: u32) -> Result<&Value, String> {
        let register_number = match usize::try_from(register_number) {
            Ok(num) => num,
            Err(_) => {
                return Err(format!(
                    "Invalid register number: {}. Must be a non-negative integer.",
                    register_number
                ));
            }
        };

        if register_number < 1 || register_number > 32 {
            return Err(format!(
                "Invalid register number: {}. Valid register numbers are 1-32.",
                register_number
            ));
        }

        return match self.general_purpose_registers.get(register_number - 1) {
            Some(value) => Ok(value),
            None => Err(format!(
                "Invalid register number: {}. Valid register numbers are 1-32.",
                register_number
            )),
        };
    }

    pub fn set_register(&mut self, register_number: u32, value: &Value) -> Result<(), String> {
        let register_number = match usize::try_from(register_number) {
            Ok(num) => num,
            Err(_) => {
                return Err(format!(
                    "Invalid register number: {}. Must be a non-negative integer.",
                    register_number
                ));
            }
        };

        if register_number < 1 || register_number > 32 {
            return Err(format!(
                "Invalid register number: {}. Valid register numbers are 1-32.",
                register_number
            ));
        }

        match register_number - 1 {
            0..=31 => self.general_purpose_registers[register_number - 1] = value.to_owned(),
            _ => {
                return Err(format!(
                    "Invalid register number: {}. Valid register numbers are 1-32.",
                    register_number
                ));
            }
        }

        return Ok(());
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
