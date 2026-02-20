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
            "Read \"{}\". Extract only info about \"{}\". Be short. Bullet points only.",
            value_a, value_b
        )),
        RType::Xpn => Ok(format!(
            "Write a detailed report about \"{}\". Use the facts in \"{}\" to add detail. No fluff.",
            value_a, value_b
        )),
        RType::Trn => Ok(format!(
            "Convert \"{}\" into \"{}\" format. Do not add text. Output only the code/data.",
            value_a, value_b
        )),
        // Cognitive operations.
        RType::Cmp => Ok(format!(
            "Compare \"{}\" and \"{}\". List what is different. List what is the same.",
            value_a, value_b
        )),
        RType::Syn => Ok(format!(
            "Combine \"{}\" and \"{}\" into one new text. Keep the meaning of both. Be seamless.",
            value_a, value_b
        )),
        RType::Flt => Ok(format!(
            "Copy-paste only the parts of \"{}\" that mention \"{}\". Delete everything else.",
            value_a, value_b
        )),
        RType::Prd => Ok(format!(
            "Current state: \"{}\". Action: \"{}\". What happens next? List the result.",
            value_a, value_b
        )),
        // Guardrails operations.
        RType::Vfy => Ok(format!(
            "Check \"{}\" against \"{}\". List any lies or errors. If none, say \"Verified\".",
            value_a, value_b
        )),
        _ => Err("Unsupported r_type for micro prompt generation."),
    }
}
