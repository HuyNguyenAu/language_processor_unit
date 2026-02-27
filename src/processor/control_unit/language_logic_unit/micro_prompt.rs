use crate::processor::control_unit::instruction::RType;

pub fn true_values(r_type: &RType) -> Result<Vec<&'static str>, &'static str> {
    match r_type {
        RType::Audit => Ok(vec!["YES"]),
        _ => Err("Unsupported r_type for true values."),
    }
}

pub fn create(r_type: &RType, value: &str) -> Result<String, &'static str> {
    match r_type {
        // Generative operations.
        RType::Morph => Ok(format!(
            "Transform it into the following format:\n{}\n\nTransformed Output:",
            value
        )),
        RType::Project => Ok(format!(
            "Project how it might evolve based on this direction or trend:\n{}\n\nProjected Output:",
            value
        )),
        // Cognitive operations.
        RType::Distill => Ok(format!(
            "Distill it down following the goal or criteria:\n{}\n\nDistilled Result:",
            value
        )),
        RType::Correlate => Ok(format!(
            "Find the correlation with:\n{}\n\nRelational Analysis:",
            value
        )),
        // Guardrails operations.
        RType::Audit => Ok(format!(
            "Does it comply with the evidence:\n{}\n\nYES/NO:",
            value
        )),
        _ => Err("Unsupported r_type for micro prompt generation."),
    }
}
