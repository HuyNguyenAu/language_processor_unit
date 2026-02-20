use crate::processor::control_unit::instruction::RType;

pub fn true_values(r_type: &RType) -> Result<Vec<&'static str>, &'static str> {
    match r_type {
        RType::Vfy => Ok(vec!["VERIFIED"]),
        _ => Err("Unsupported r_type for true values."),
    }
}

pub fn search(r_type: &RType, value_a: &str, value_b: &str) -> Result<String, &'static str> {
    match r_type {
        // Generative operations.
        RType::Sum => Ok(format!(
            "TASK: Summarize\nDATA: \"{}\"\nCONSTRAINT: \"{}\"\nRESULT:",
            value_a, value_b
        )),
        RType::Xpn => Ok(format!(
            "TOPIC: \"{}\"\nCONTEXT TO ADD: \"{}\"\nEXPANDED RESULT:",
            value_a, value_b
        )),
        RType::Trn => Ok(format!(
            "SOURCE: \"{}\"\nTARGET FORMAT: \"{}\"\nTRANSFORMED DATA:",
            value_a, value_b
        )),
        // Cognitive operations.
        RType::Cmp => Ok(format!(
            "ITEM A: \"{}\"\nITEM B: \"{}\"\nCOMPARISON POINTS:",
            value_a, value_b
        )),
        RType::Syn => Ok(format!(
            "INPUT 1: \"{}\"\nINPUT 2: \"{}\"\nSYNTHESIZED OUTPUT:",
            value_a, value_b
        )),
        RType::Flt => Ok(format!(
            "RAW DATA: \"{}\"\nFILTER CRITERIA: \"{}\"\nFILTERED DATA:",
            value_a, value_b
        )),
        RType::Prd => Ok(format!(
            "CURRENT STATE: \"{}\"\nTREND/RULE: \"{}\"\nPREDICTED NEXT:",
            value_a, value_b
        )),
        // Guardrails operations.
        RType::Vfy => Ok(format!(
            "CLAIM: \"{}\"\nEVIDENCE: \"{}\"\nSTATUS [VERIFIED/FAILED]:",
            value_b, value_a
        )),
        _ => Err("Unsupported r_type for micro prompt generation."),
    }
}
