use crate::assembler::opcode::OpCode;

pub fn true_values(opcode: &OpCode) -> Result<Vec<&'static str>, &'static str> {
    match opcode {
        OpCode::EQV => Ok(vec!["IDENTICAL", "SYNONYMOUS", "RELATED"]),
        OpCode::INT => Ok(vec!["TRUE"]),
        OpCode::HAL => Ok(vec!["REAL"]),
        _ => Err("Unsupported opcode for true values."),
    }
}

pub fn search(opcode: &OpCode, value_a: &str, value_b: &str) -> Result<String, &'static str> {
    match opcode {
        // Semantic operations.
        OpCode::ADD => Ok(format!(
            "Merge the essence, attributes, and presence of \"{}\" and \"{}\" into a single form.",
            value_a, value_b
        )),
        OpCode::SUB => Ok(format!(
            "Strip the essence, attributes, and presence of \"{}\" away from \"{}\", leaving only the remainder.",
            value_b, value_a
        )),
        OpCode::MUL => Ok(format!(
            "Magnify the intensity, scale, and influence of \"{}\" using the defining traits of \"{}\".",
            value_a, value_b
        )),
        OpCode::DIV => Ok(format!(
            "Deconstruct the complex concept \"{}\" into the specific units of \"{}\". List only the resulting components.",
            value_a, value_b
        )),
        OpCode::INF => Ok(format!(
            "Identify the pattern, sequence, or narrative trajectory in \"{}\". Project this trajectory forward by the amount specified in \"{}\".",
            value_a, value_b
        )),
        OpCode::ADT => Ok(format!(
            "Hold the data in \"{}\" against the sacred light of the criteria \"{}\". List any fractures where the data fails to comply.",
            value_a, value_b
        )),
        // Heuristic operations.
        OpCode::EQV => Ok(format!(
            "Relation: \"{}\" vs \"{}\". Label: [IDENTICAL, SYNONYMOUS, RELATED, DISPARATE]. Result:",
            value_a, value_b
        )),
        OpCode::INT => Ok(format!(
            "Does the hidden intent behind \"{}\" align with the goal of \"{}\"? Answer TRUE or FALSE.",
            value_a, value_b
        )),
        OpCode::HAL => Ok(format!(
            "Does \"{}\" ring true with reality, or is it a hollow hallucination? Answer REAL or HOLLOW.",
            value_a
        )),
        _ => Err("Unsupported opcode for micro prompt generation."),
    }
}
