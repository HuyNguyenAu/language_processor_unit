use crate::processor::control_unit::semantic_logic_unit::value::Value;

pub struct Registers {
    register_1: Value,
    register_2: Value,
    register_3: Value,
    register_4: Value,
    register_5: Value,
    register_6: Value,
    register_7: Value,
    register_8: Value,
    instruction_pointer: u8,
}

impl Registers {
    pub fn new() -> Self {
        Registers {
            register_1: Value::None,
            register_2: Value::None,
            register_3: Value::None,
            register_4: Value::None,
            register_5: Value::None,
            register_6: Value::None,
            register_7: Value::None,
            register_8: Value::None,
            instruction_pointer: 0,
        }
    }

    pub fn get_register(&self, register_number: &u8) -> &Value {
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

    pub fn set_register(&mut self, register_number: &u8, value: Value) {
        match register_number {
            1 => self.register_1 = value,
            2 => self.register_2 = value,
            3 => self.register_3 = value,
            4 => self.register_4 = value,
            5 => self.register_5 = value,
            6 => self.register_6 = value,
            7 => self.register_7 = value,
            8 => self.register_8 = value,
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
