use crate::processor::control_unit::instruction::RType;

pub fn true_values(r_type: &RType) -> Result<Vec<&'static str>, &'static str> {
    match r_type {
        RType::Audit => Ok(vec!["PASS"]),
        _ => Err("Unsupported r_type for true values."),
    }
}

pub fn search(r_type: &RType, value_a: &str, value_b: &str) -> Result<String, &'static str> {
    match r_type {
        // Generative operations.
        RType::Morph => Ok(format!(
            "Source: {}\nTarget Format: {}\nMorphed Data:",
            value_a, value_b
        )),
        RType::Project => Ok(format!(
            "Base Data: {}\nDirection/Trend: {}\nProjected Output:",
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
            "Claim: {}\nEvidence: {}\nAudit Status [Pass/Fail]:",
            value_a, value_b
        )),
        _ => Err("Unsupported r_type for micro prompt generation."),
    }
}
