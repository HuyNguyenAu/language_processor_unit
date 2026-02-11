pub struct Microcode {}

impl Microcode {
    pub fn new() -> Self {
        return Microcode {};
    }

    pub fn addition(&self, value_a: &str, value_b: &str) -> String {
        return format!(
            "Merge {} into {}. Ensure logical continuity and smooth linguistic transitions. The resulting output must contain the complete factual density of both inputs without redundancy. Answer with a single word if appropriate, otherwise a single sentence.",
            value_a, value_b
        );
    }

    pub fn subtract(&self, value_a: &str, value_b: &str) -> String {
        return format!(
            "Identify any concepts, entities, or phrases that align with the definitions in {}. Remove them entirely from {}. Reconstruct the remaining concepts, entities, or phrases so it remains grammatically correct and coherent, but devoid of the subtracted elements. Answer with a single word if appropriate, otherwise a single sentence.",
            value_b, value_a
        );
    }
}
