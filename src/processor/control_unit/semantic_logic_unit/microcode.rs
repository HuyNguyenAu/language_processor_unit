pub struct Microcode {}

impl Microcode {
    pub fn new() -> Self {
        return Microcode {};
    }

    pub fn addition(&self, value_a: &str, value_b: &str) -> String {
        return format!(
            "Synthesize the attributes of the {} with the attributes of the {}. Locate the specific word that represents the intersection of these two identities. Output exactly one word.",
            value_a, value_b
        );
    }

    pub fn subtract(&self, value_a: &str, value_b: &str) -> String {
        return format!(
            "Isolate the unique attributes of the {} by removing the shared attributes with the {}. Locate the specific word that represents the difference of these two identities. Output exactly one word.",
            value_a, value_b
        );
    }
}
