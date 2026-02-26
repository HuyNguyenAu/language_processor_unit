use crate::processor::control_unit::instruction::RType;

pub fn true_values(r_type: &RType) -> Result<Vec<&'static str>, &'static str> {
    match r_type {
        RType::Audit => Ok(vec!["YES"]),
        _ => Err("Unsupported r_type for true values."),
    }
}

pub fn create(r_type: &RType, value_a: &str, value_b: &str) -> Result<String, &'static str> {
    match r_type {
        // Generative operations.
        RType::Morph => Ok(format!(
            "Take the following:\n{}\n\nAnd transform it into the following format:\n{}\n\nTransformed Output:",
            value_a, value_b
        )),
        RType::Project => Ok(format!(
            "Take the following data:\n{}\n\nProject how it might evolve based on this direction or trend:\n{}\n\nProjected Output:",
            value_a, value_b
        )),
        // Cognitive operations.
        RType::Distill => Ok(format!(
            "Input: {}\nGoal/Criteria: {}\nDistilled Result:",
            value_a, value_b
        )),
        RType::Correlate => Ok(format!(
            "Entity A: {}\nEntity B: {}\nRelational Analysis:",
            value_a, value_b
        )),
        // Guardrails operations.
        RType::Audit => Ok(format!(
            "Does the claim:\n{}\n\nComply with the evidence:\n{}\n\nYES/NO:",
            value_a, value_b
        )),
        _ => Err("Unsupported r_type for micro prompt generation."),
    }
}
