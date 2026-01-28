pub struct Registers {
    register_1: String,
    register_2: String,
    register_3: String,
    register_4: String,
    register_5: String,
    register_6: String,
    register_7: String,
    register_8: String,
    instruction_pointer: u8,
}

impl Registers {
    pub fn new() -> Self {
        Registers {
            register_1: String::new(),
            register_2: String::new(),
            register_3: String::new(),
            register_4: String::new(),
            register_5: String::new(),
            register_6: String::new(),
            register_7: String::new(),
            register_8: String::new(),
            instruction_pointer: 0,
        }
    }

    pub fn get_register(&self, register_number: &u8) -> &str {
        return match register_number {
            1 => &self.register_1,
            2 => &self.register_2,
            3 => &self.register_3,
            4 => &self.register_4,
            5 => &self.register_5,
            6 => &self.register_6,
            7 => &self.register_7,
            8 => &self.register_8,
            _ => panic!("Invalid register number."),
        };
    }

    pub fn set_register(&mut self, register_number: &u8, value: &str) {
        match register_number {
            1 => self.register_1 = value.to_string(),
            2 => self.register_2 = value.to_string(),
            3 => self.register_3 = value.to_string(),
            4 => self.register_4 = value.to_string(),
            5 => self.register_5 = value.to_string(),
            6 => self.register_6 = value.to_string(),
            7 => self.register_7 = value.to_string(),
            8 => self.register_8 = value.to_string(),
            _ => panic!("Invalid register number."),
        }
    }

    pub fn get_instruction_pointer(&self) -> u8 {
        self.instruction_pointer
    }

    pub fn set_instruction_pointer(&mut self, address: &u8) {
        self.instruction_pointer = *address;
    }

    pub fn advance_instruction_pointer(&mut self) {
        self.instruction_pointer += 1;
    }
}
