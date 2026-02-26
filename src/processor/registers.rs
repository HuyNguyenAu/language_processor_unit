use std::fmt;

#[derive(Debug, Clone)]
pub enum Value {
    Text(String),
    Number(u32),
    None,
}

impl fmt::Display for Value {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Text(text) => write!(formatter, "{}", text),
            Value::Number(number) => write!(formatter, "{}", number),
            Value::None => write!(formatter, ""),
        }
    }
}

pub struct Registers {
    general_purpose: [Value; 32],
    instruction_pointer: usize,
    instruction: Option<[[u8; 4]; 4]>,
    data_section_pointer: usize,
}

impl Registers {
    pub fn new() -> Self {
        Registers {
            general_purpose: [const { Value::None }; 32],
            instruction_pointer: 0,
            instruction: None,
            data_section_pointer: 0,
        }
    }

    fn to_index(register_number: u32) -> Result<usize, String> {
        let idx = usize::try_from(register_number).map_err(|_| {
            format!(
                "Invalid register number: {}. Must be a non-negative integer.",
                register_number
            )
        })?;

        if !(1..=32).contains(&idx) {
            return Err(format!(
                "Invalid register number: {}. Valid register numbers are 1-32.",
                register_number
            ));
        }

        Ok(idx - 1)
    }

    pub fn get_register(&self, register_number: u32) -> Result<&Value, String> {
        let idx = Self::to_index(register_number)?;

        Ok(&self.general_purpose[idx])
    }

    pub fn set_register(&mut self, register_number: u32, value: &Value) -> Result<(), String> {
        let idx = Self::to_index(register_number)?;
        self.general_purpose[idx] = value.clone();

        Ok(())
    }

    pub fn get_instruction_pointer(&self) -> usize {
        self.instruction_pointer
    }

    pub fn set_instruction_pointer(&mut self, address: usize) {
        self.instruction_pointer = address;
    }

    pub fn advance_instruction_pointer(&mut self, offset: usize) {
        self.instruction_pointer += offset;
    }

    pub fn read_text(&self, register_number: u32) -> Result<&String, String> {
        match self.get_register(register_number)? {
            Value::Text(text) => Ok(text),
            other => Err(format!(
                "Register r{} contains {:?}, expected text.",
                register_number, other
            )),
        }
    }

    pub fn read_number(&self, register_number: u32) -> Result<u32, String> {
        match self.get_register(register_number)? {
            Value::Number(number) => Ok(*number),
            other => Err(format!(
                "Register r{} contains {:?}, expected number.",
                register_number, other
            )),
        }
    }

    pub fn get_instruction(&self) -> Option<[[u8; 4]; 4]> {
        self.instruction
    }

    pub fn set_instruction(&mut self, be_bytes: Option<[[u8; 4]; 4]>) {
        self.instruction = be_bytes;
    }

    pub fn get_data_section_pointer(&self) -> usize {
        self.data_section_pointer
    }

    pub fn set_data_section_pointer(&mut self, address: usize) {
        self.data_section_pointer = address;
    }
}
