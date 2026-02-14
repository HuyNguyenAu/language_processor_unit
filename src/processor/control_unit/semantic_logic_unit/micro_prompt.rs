use crate::assembler::opcode::OpCode;

pub struct MicroPrompt {}

impl MicroPrompt {
    pub fn new() -> Self {
        return Self {};
    }

    pub fn search(
        &self,
        opcode: &OpCode,
        value_a: &str,
        value_b: &str,
    ) -> Result<String, &'static str> {
        return match opcode {
            OpCode::ADD => Ok(format!(
                "Merge the essence, attributes, and presence of \"{}\" and \"{}\" into a single form.",
                value_a, value_b
            )),
            OpCode::SUB => Ok(format!(
                "Strip the essence, attributes, and presence of \"{}\" away from \"{}\", leaving only the remainder.",
                value_b, value_a
            )),
            OpCode::INF => Ok(format!(
                "Identify the pattern, sequence, or narrative trajectory in \"{}\". Project this trajectory forward by the amount specified in \"{}\".",
                value_a, value_b
            )),
            OpCode::DIV => Ok(format!(
                "Deconstruct the complex concept \"{}\" into the specific units of \"{}\". List only the resulting components.",
                value_a, value_b
            )),
            _ => Err("Unsupported opcode for micro prompt generation."),
        };
    }
}
