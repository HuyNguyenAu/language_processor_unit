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
            "SOURCE: \"{}\"\nTARGET FORMAT: \"{}\"\nMORPHED DATA:",
            value_a, value_b
        )),
        RType::Project => Ok(format!(
            "BASE DATA: \"{}\"\nDIRECTION/TREND: \"{}\"\nPROJECTED OUTPUT:",
            value_a, value_b
        )),
        // Cognitive operations.
        RType::Distill => Ok(format!(
            "INPUT: \"{}\"\nGOAL/CRITERIA: \"{}\"\nDISTILLED RESULT:",
            value_a, value_b
        )),
        RType::Correlate => Ok(format!(
            "ENTITY A: \"{}\"\nENTITY B: \"{}\"\nRELATIONAL ANALYSIS:",
            value_a, value_b
        )),
        // Guardrails operations.
        RType::Audit => Ok(format!(
            "CLAIM: \"{}\"\nEVIDENCE: \"{}\"\nAUDIT STATUS [PASS/FAIL]:",
            value_a, value_b
        )),
        _ => Err("Unsupported r_type for micro prompt generation."),
    }
}
