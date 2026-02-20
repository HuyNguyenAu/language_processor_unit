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
            "Distill {} into its most essential points, focusing specifically on the themes/dimensions requested in {}. Ignore all other data.",
            value_a, value_b
        )),
        RType::Xpn => Ok(format!(
            "Elaborate on the seed concept {} by applying the specific context or technical depth defined in {}. Maintain logical consistency with the seed.",
            value_a, value_b
        )),
        RType::Trn => Ok(format!(
            "Map the data in {} into the specific structure, schema, or language defined by {}. Ensure 1:1 data integrity. Output only the result.",
            value_a, value_b
        )),
        // Cognitive operations.
        RType::Cmp => Ok(format!(
            "Perform a delta analysis between {} and {}. Identify and list unique elements, shared overlaps, and direct contradictions between the two.",
            value_a, value_b
        )),
        RType::Syn => Ok(format!(
            "Integrate the logic/content of {} with the constraints or stylistic requirements of {}. Create a unified output that satisfies both inputs.",
            value_a, value_b
        )),
        RType::Flt => Ok(format!(
            "Scan {} and extract only the segments that satisfy the criteria defined in {}. Discard all irrelevant or non-matching information.",
            value_a, value_b
        )),
        RType::Prd => Ok(format!(
            "Given the state {}, simulate the consequences of action {}. Describe the immediate resulting environment and any secondary side effects.",
            value_a, value_b
        )),
        // Guardrails operations.
        RType::Vfy => Ok(format!(
            "Audit {} against the source of truth {}. Identify any claims that are unsupported or false. If 100% accurate, return 'Verified'.",
            value_a, value_b
        )),
        _ => Err("Unsupported r_type for micro prompt generation."),
    }
}
