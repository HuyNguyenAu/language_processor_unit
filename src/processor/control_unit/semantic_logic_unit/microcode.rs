pub struct Microcode {}

impl Microcode {
    pub fn new() -> Self {
        return Microcode {};
    }

    pub fn addition(&self, first_operand: &str, second_operand: &str) -> String {
        return format!(
            "Synthesize the attributes of the {} with the attributes of the {}. Locate the specific word that represents the intersection of these two identities within the latent space. Output exactly one word.",
            first_operand, second_operand
        );
    }

    pub fn subtract(&self, first_operand: &str, second_operand: &str) -> String {
        return format!(
            "Isolate the unique attributes of the {} by removing the shared attributes with the {}. Locate the specific word that represents the difference of these two identities within the latent space. Output exactly one word.",
            first_operand, second_operand
        );
    }
}
