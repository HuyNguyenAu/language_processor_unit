use crate::{
    instruction::{
        AddInstruction, ComparisonType, Instruction, JumpCompareInstruction, MoveInstruction,
        Operand, OperandType, OutputInstruction, SimilarityInstruction, SubInstruction,
    },
    opcode::OpCode,
    openai::{OpenAIChatRequest, OpenAIChatRequestText, OpenAIClient, OpenAIEmbeddingsRequest},
};

struct MemoryUnit {
    data: Vec<u8>,
}

impl MemoryUnit {
    fn new() -> Self {
        MemoryUnit { data: Vec::new() }
    }

    fn load(&mut self, bytecode: Vec<u8>) {
        println!("Loading bytecode of length {}", bytecode.len());

        self.data = bytecode;
    }

    fn read_byte(&self, address: &u8) -> &u8 {
        return match self.data.get(*address as usize) {
            Some(byte) => byte,
            None => panic!("Address out of bounds."),
        };
    }
}

struct Registers {
    register_1: String,
    register_2: String,
    register_3: String,
    register_4: String,
    register_5: String,
    register_6: String,
    register_7: String,
    register_8: String,
    instruction_pointer: u8,
}

impl Registers {
    fn new() -> Self {
        Registers {
            register_1: String::new(),
            register_2: String::new(),
            register_3: String::new(),
            register_4: String::new(),
            register_5: String::new(),
            register_6: String::new(),
            register_7: String::new(),
            register_8: String::new(),
            instruction_pointer: 0,
        }
    }

    fn set_register(&mut self, register_number: &u8, value: &str) {
        match register_number {
            1 => self.register_1 = value.to_string(),
            2 => self.register_2 = value.to_string(),
            3 => self.register_3 = value.to_string(),
            4 => self.register_4 = value.to_string(),
            5 => self.register_5 = value.to_string(),
            6 => self.register_6 = value.to_string(),
            7 => self.register_7 = value.to_string(),
            8 => self.register_8 = value.to_string(),
            _ => panic!("Invalid register number."),
        }
    }

    fn get_register(&self, register_number: &u8) -> &str {
        return match register_number {
            1 => &self.register_1,
            2 => &self.register_2,
            3 => &self.register_3,
            4 => &self.register_4,
            5 => &self.register_5,
            6 => &self.register_6,
            7 => &self.register_7,
            8 => &self.register_8,
            _ => panic!("Invalid register number."),
        };
    }

    pub fn set_instruction_pointer(&mut self, address: &u8) {
        self.instruction_pointer = *address;
    }

    fn advance_instruction_pointer(&mut self) {
        self.instruction_pointer += 1;
    }
}

struct SemanticLogicUnit {
    openai_client: OpenAIClient,
    model: &'static str,
    role: &'static str,
    stream: bool,
    temperature: f32,
    encoding_format: &'static str,
}

impl SemanticLogicUnit {
    pub fn new() -> Self {
        return SemanticLogicUnit {
            openai_client: OpenAIClient::new(),
            model: "granite-4.0-h-350m-Q8_0.gguf",
            role: "user",
            stream: false,
            temperature: 0.8,
            encoding_format: "float",
        };
    }

    fn clean_string(&self, value: &str) -> String {
        return value.trim().replace("\n", "").to_string();
    }

    fn chat(&self, content: &str) -> Result<String, String> {
        let request = OpenAIChatRequest {
            model: self.model.to_string(),
            stream: self.stream,
            messages: vec![OpenAIChatRequestText {
                role: self.role.to_string(),
                content: content.to_string(),
            }],
            temperature: self.temperature,
        };

        let response = &self.openai_client.chat(request);

        let choice = match response {
            Ok(response) => response.choices.iter().nth(0),
            Err(err) => {
                return Err(format!(
                    "Failed to get chat response from client. Error: {}",
                    err
                ));
            }
        };

        return match choice {
            Some(choice) => Ok(self.clean_string(&choice.message.content)),
            None => Err("No choices returned from client.".to_string()),
        };
    }

    fn embeddings(&self, content: &str) -> Result<Vec<f32>, String> {
        let request = OpenAIEmbeddingsRequest {
            model: self.model.to_string(),
            input: content.to_string(),
            encoding_format: self.encoding_format.to_string(),
        };

        let response = &self.openai_client.embeddings(request);

        let embeddings = match response {
            Ok(response) => response.data.iter().nth(0),
            Err(err) => {
                return Err(format!(
                    "Failed to get embeddings response from client. Error: {}",
                    err
                ));
            }
        };

        return match embeddings {
            Some(value) => Ok(value.embedding.clone()),
            None => Err("No embeddings returned from client.".to_string()),
        };
    }

    pub fn addition(&self, first_operand: &str, second_operand: &str) -> String {
        let content = format!(
            "Synthesize the attributes of the {} with the attributes of the {}. Locate the specific noun that represents the intersection of these two identities within the latent space. Output exactly one word.",
            first_operand, second_operand
        );

        return match &self.chat(content.as_str()) {
            Ok(choice) => choice.to_lowercase(),
            Err(error) => panic!("Failed to perform additionString. Error: {}", error),
        };
    }

    pub fn subtract(&self, first_operand: &str, second_operand: &str) -> String {
        let content = format!(
            "Synthesize the attributes of the {} without the attributes of the {}. Locate the specific noun that represents the intersection of these two identities within the latent space. Output exactly one word.",
            first_operand, second_operand,
        );

        return match &self.chat(content.as_str()) {
            Ok(choice) => choice.to_lowercase(),
            Err(error) => panic!("Failed to perform subtraction. Error: {}", error),
        };
    }

    pub fn similarity(&self, first_operand: &str, second_operand: &str) -> String {
        let first_embedding_result = self.embeddings(first_operand);
        let first_embedding = match &first_embedding_result {
            Ok(embedding) => embedding,
            Err(error) => panic!("Failed to get first embedding. Error: {}", error),
        };

        let second_embedding_result = self.embeddings(second_operand);
        let second_embedding = match &second_embedding_result {
            Ok(embedding) => embedding,
            Err(error) => panic!("Failed to get second embedding. Error: {}", error),
        };

        // Compute cosine similarity.
        let dot_product: f32 = first_embedding
            .iter()
            .zip(second_embedding.iter())
            .map(|(a, b)| a * b)
            .sum();
        let x_euclidean_length: f32 = first_embedding.iter().map(|x| x * x).sum::<f32>().sqrt();
        let y_euclidean_length: f32 = second_embedding.iter().map(|y| y * y).sum::<f32>().sqrt();
        let similarity = dot_product / (x_euclidean_length * y_euclidean_length);

        return ((similarity * 100.0).round()).to_string();
    }
}

struct ControlUnit {
    memory: MemoryUnit,
    registers: Registers,
    semantic_logic_unit: SemanticLogicUnit,

    previous_byte: Option<u8>,
    current_byte: Option<u8>,
}

impl ControlUnit {
    fn new() -> Self {
        ControlUnit {
            memory: MemoryUnit::new(),
            registers: Registers::new(),
            semantic_logic_unit: SemanticLogicUnit::new(),
            previous_byte: None,
            current_byte: None,
        }
    }

    fn is_at_end(&self) -> bool {
        return self.registers.instruction_pointer as usize >= self.memory.data.len() - 1;
    }

    fn advance(&mut self) {
        self.registers.advance_instruction_pointer();

        self.previous_byte = self.current_byte;

        let current_byte = self.memory.read_byte(&self.registers.instruction_pointer);
        self.current_byte = Some(*current_byte);
    }

    fn decode_operand_type(&mut self, message: &str) -> OperandType {
        let operand_byte = match self.current_byte {
            Some(byte) => byte,
            None => panic!("No current byte to determine operand type."),
        };

        // Consume operand type byte.
        self.advance();

        let operand_type = match OperandType::from_byte(&operand_byte) {
            Ok(operand_type) => operand_type,
            Err(error) => panic!("{} {} Byte: {:02X}", message, error, operand_byte),
        };

        return operand_type;
    }

    fn decode_text(&mut self, message: &str) -> String {
        let mut text_length: usize = 0;

        if let Some(length_byte) = self.current_byte {
            // Consume text length byte.
            self.advance();

            text_length = length_byte as usize;
        }

        let mut text_bytes: Vec<u8> = Vec::new();

        while text_bytes.len() < text_length
            && let Some(text_byte) = self.current_byte
        {
            if !self.is_at_end() {
                // Consume text byte.
                self.advance();
            }

            text_bytes.push(text_byte);
        }

        if let Ok(text) = String::from_utf8(text_bytes) {
            return text;
        }

        panic!("{}", message);
    }

    fn decode_register(&mut self, length_byte: bool, message: &str) -> u8 {
        // Consume register length byte if needed.
        if length_byte {
            self.advance();
        }

        let register_byte = match self.current_byte {
            Some(byte) => byte,
            None => panic!("{}", message),
        };

        if !self.is_at_end() {
            // Consume register byte.
            self.advance();
        }

        return register_byte;
    }

    fn decode_number(&mut self, length_byte: bool, message: &str) -> u8 {
        // Consume number length byte if needed.
        if length_byte {
            self.advance();
        }

        let number_byte = match self.current_byte {
            Some(byte) => byte,
            None => panic!("{}", message),
        };

        if !self.is_at_end() {
            // Consume number byte.
            self.advance();
        }

        return number_byte;
    }

    fn decode_operand(
        &mut self,
        operand_type_message: &str,
        operand_number: &str,
        operand_text: &str,
        operand_register: &str,
    ) -> Operand {
        let operand_type = self.decode_operand_type(operand_type_message);

        return match operand_type {
            OperandType::NUMBER => Operand::Number(self.decode_number(true, operand_number)),
            OperandType::TEXT => Operand::Text(self.decode_text(operand_text)),
            OperandType::REGISTER => {
                Operand::Register(self.decode_register(true, operand_register))
            }
        };
    }

    // fn debug(&self, message: &str) {
    //     println!(
    //         "[{}] IP: {}, Prev Byte: {:02X}, Curr Byte: {:02X}",
    //         message,
    //         self.registers.instruction_pointer,
    //         match self.previous_byte {
    //             Some(value) => value as i32,
    //             None => -1,
    //         },
    //         match self.current_byte {
    //             Some(value) => value as i32,
    //             None => -1,
    //         }
    //     );
    // }

    fn decode_move(&mut self) -> MoveInstruction {
        // Consume MOV opcode.
        self.advance();

        // Consume the destination register.
        let destination_register = self.decode_register(
            false,
            "Failed to read destination register for MOV instruction.",
        );

        // Consume value operand.
        let value = self.decode_operand(
            "Failed to determine operand type for MOV instruction.",
            "Failed to read number for MOV instruction.",
            "Failed to read text for MOV instruction.",
            "Failed to read source register for MOV instruction.",
        );

        return MoveInstruction {
            destination_register,
            value,
        };
    }

    fn decode_subtract(&mut self) -> SubInstruction {
        // Consume SUB opcode.
        self.advance();

        // Consume the first operand.
        let first_operand = self.decode_operand(
            "Failed to determine first operand type for SUB instruction.",
            "Failed to read first number operand for SUB instruction.",
            "Failed to read first text operand for SUB instruction.",
            "Failed to read first register operand for SUB instruction.",
        );

        // Consume the second operand.
        let second_operand = self.decode_operand(
            "Failed to determine second operand type for SUB instruction.",
            "Failed to read second number operand for SUB instruction.",
            "Failed to read second text operand for SUB instruction.",
            "Failed to read second register operand for SUB instruction.",
        );

        // Consume the destination register.
        let destination_register = self.decode_register(
            false,
            "Failed to read destination register for SUB instruction.",
        );

        return SubInstruction {
            destination_register,
            first_operand,
            second_operand,
        };
    }

    fn decode_addition(&mut self) -> AddInstruction {
        // Consume ADD opcode.
        self.advance();

        // Consume the first operand.
        let first_operand = self.decode_operand(
            "Failed to determine first operand type for ADD instruction.",
            "Failed to read first number operand for ADD instruction.",
            "Failed to read first text operand for ADD instruction.",
            "Failed to read first register operand for ADD instruction.",
        );

        // Consume the second operand.
        let second_operand = self.decode_operand(
            "Failed to determine second operand type for ADD instruction.",
            "Failed to read second number operand for ADD instruction.",
            "Failed to read second text operand for ADD instruction.",
            "Failed to read second register operand for ADD instruction.",
        );

        // Consume the destination register.
        let destination_register = self.decode_register(
            false,
            "Failed to read destination register for ADD instruction.",
        );

        return AddInstruction {
            destination_register,
            first_operand,
            second_operand,
        };
    }

    fn decode_similarity(&mut self) -> SimilarityInstruction {
        // Consume SIM opcode.
        self.advance();

        // Consume the first operand.
        let first_operand = self.decode_operand(
            "Failed to determine first operand type for SIM instruction.",
            "Failed to read first number operand for SIM instruction.",
            "Failed to read first text operand for SIM instruction.",
            "Failed to read first register operand for SIM instruction.",
        );

        // Consume the second operand.
        let second_operand = self.decode_operand(
            "Failed to determine second operand type for SIM instruction.",
            "Failed to read second number operand for SIM instruction.",
            "Failed to read second text operand for SIM instruction.",
            "Failed to read second register operand for SIM instruction.",
        );

        // Consume the destination register.
        let destination_register = self.decode_register(
            false,
            "Failed to read destination register for SIM instruction.",
        );

        return SimilarityInstruction {
            destination_register,
            first_operand,
            second_operand,
        };
    }

    fn decode_jump_compare(&mut self, op_code: OpCode) -> JumpCompareInstruction {
        // Consume jump comparison opcode.
        self.advance();

        // Consume the first operand.
        let first_operand = self.decode_operand(
            &format!(
                "Failed to determine first operand type for {:?} instruction.",
                op_code
            ),
            &format!(
                "Failed to read first number operand for {:?} instruction.",
                op_code
            ),
            &format!(
                "Failed to read first text operand for {:?} instruction.",
                op_code
            ),
            &format!(
                "Failed to read first register operand for {:?} instruction.",
                op_code
            ),
        );

        // Consume the second operand.
        let second_operand = self.decode_operand(
            &format!(
                "Failed to determine second operand type for {:?} instruction.",
                op_code
            ),
            &format!(
                "Failed to read second number operand for {:?} instruction.",
                op_code
            ),
            &format!(
                "Failed to read second text operand for {:?} instruction.",
                op_code
            ),
            &format!(
                "Failed to read second register operand for {:?} instruction.",
                op_code
            ),
        );

        // Consume the bytecode jump index.
        let bytecode_jump_index = self.decode_number(
            false,
            format!(
                "Failed to read byte code jump index for {:?} instruction.",
                op_code
            )
            .as_str(),
        );
        let comparison_type = match op_code {
            OpCode::JEQ => ComparisonType::Equal,
            OpCode::JLT => ComparisonType::LessThan,
            OpCode::JLE => ComparisonType::LessThanOrEqual,
            OpCode::JGT => ComparisonType::GreaterThan,
            OpCode::JGE => ComparisonType::GreaterThanOrEqual,
            _ => panic!("Invalid opcode for jump compare instruction."),
        };

        return JumpCompareInstruction {
            comparison_type,
            bytecode_jump_index,
            first_operand,
            second_operand,
        };
    }

    fn decode_output(&mut self) -> OutputInstruction {
        // Consume OUT opcode.
        self.advance();

        // Consume the source operand.
        let source_operand = self.decode_operand(
            "Failed to determine source operand type for OUT instruction.",
            "Failed to read source number operand for OUT instruction.",
            "Failed to read source text operand for OUT instruction.",
            "Failed to read source register operand for OUT instruction.",
        );

        return OutputInstruction { source_operand };
    }

    fn decode_op_code(&mut self) -> Instruction {
        let current_byte = match self.current_byte {
            Some(byte) => byte,
            None => panic!("No current byte to determine opcode."),
        };
        let op_code = match OpCode::from_byte(&current_byte) {
            Ok(op_code) => op_code,
            Err(error) => panic!("{} Byte: {:02X}", error, current_byte),
        };

        return match op_code {
            OpCode::MOV => Instruction::Move(self.decode_move()),
            OpCode::ADD => Instruction::Add(self.decode_addition()),
            OpCode::SUB => Instruction::Sub(self.decode_subtract()),
            OpCode::SIM => Instruction::Similarity(self.decode_similarity()),
            OpCode::JEQ => Instruction::JumpCompare(self.decode_jump_compare(op_code)),
            OpCode::JLT => Instruction::JumpCompare(self.decode_jump_compare(op_code)),
            OpCode::JLE => Instruction::JumpCompare(self.decode_jump_compare(op_code)),
            OpCode::JGT => Instruction::JumpCompare(self.decode_jump_compare(op_code)),
            OpCode::JGE => Instruction::JumpCompare(self.decode_jump_compare(op_code)),
            OpCode::OUT => Instruction::Output(self.decode_output()),
        };
    }

    fn fetch_and_decode(&mut self) -> Option<Instruction> {
        // Initialise current byte.
        let current_byte = self.memory.read_byte(&self.registers.instruction_pointer);
        self.current_byte = Some(*current_byte);

        if self.is_at_end() {
            return None;
        }

        return Some(self.decode_op_code());
    }

    fn get_value(&self, operand: &Operand) -> String {
        return match operand {
            Operand::Number(number) => number.to_string(),
            Operand::Text(text) => text.clone(),
            Operand::Register(register_number) => {
                self.registers.get_register(register_number).to_string()
            }
        };
    }

    fn execute_move(&mut self, instruction: &MoveInstruction) {
        let value = self.get_value(&instruction.value);

        self.registers
            .set_register(&instruction.destination_register, &value);

        println!(
            "Executed MOV: r{} = \"{}\"",
            instruction.destination_register,
            self.registers
                .get_register(&instruction.destination_register)
        );
    }

    fn execute_add(&mut self, instruction: &AddInstruction) {
        let first_operand_value = self.get_value(&instruction.first_operand);
        let second_operand_value = self.get_value(&instruction.second_operand);

        let result = self
            .semantic_logic_unit
            .addition(&first_operand_value, &second_operand_value);

        self.registers
            .set_register(&instruction.destination_register, &result);

        println!(
            "Executed ADD: {:?} + {:?} -> r{} = \"{}\"",
            first_operand_value,
            second_operand_value,
            instruction.destination_register,
            self.registers
                .get_register(&instruction.destination_register)
        );
    }

    fn execute_subtract(&mut self, instruction: &SubInstruction) {
        let first_operand_value = self.get_value(&instruction.first_operand);
        let second_operand_value = self.get_value(&instruction.second_operand);

        let result = self
            .semantic_logic_unit
            .subtract(&first_operand_value, &second_operand_value);

        self.registers
            .set_register(&instruction.destination_register, &result);

        println!(
            "Executed SUB: {:?} - {:?} -> r{} = \"{}\"",
            first_operand_value,
            second_operand_value,
            instruction.destination_register,
            self.registers
                .get_register(&instruction.destination_register)
        );
    }

    fn execute_similarity(&mut self, instruction: &SimilarityInstruction) {
        let first_operand_value = self.get_value(&instruction.first_operand);
        let second_operand_value = self.get_value(&instruction.second_operand);

        let result = self
            .semantic_logic_unit
            .similarity(&first_operand_value, &second_operand_value);

        self.registers
            .set_register(&instruction.destination_register, &result);

        println!(
            "Executed SIM: {:?} ~ {:?} -> r{} = \"{}\"",
            first_operand_value,
            second_operand_value,
            instruction.destination_register,
            self.registers
                .get_register(&instruction.destination_register)
        );
    }

    fn execute_jump_compare(&mut self, instruction: &JumpCompareInstruction) {
        let first_operand_value = match self.get_value(&instruction.first_operand).parse::<u8>() {
            Ok(value) => value,
            _ => panic!(
                "{:?} instruction requires numeric operands.",
                instruction.comparison_type
            ),
        };
        let second_operand_value = match self.get_value(&instruction.second_operand).parse::<u8>() {
            Ok(value) => value,
            _ => panic!(
                "{:?} instruction requires numeric operands.",
                instruction.comparison_type
            ),
        };
        let address = instruction.bytecode_jump_index.clone();

        match instruction.comparison_type {
            ComparisonType::Equal => {
                if first_operand_value == second_operand_value {
                    self.registers.set_instruction_pointer(&address);
                }

                println!(
                    "Executed JEQ: {:?} == {:?} -> {}",
                    first_operand_value, second_operand_value, instruction.bytecode_jump_index
                );
            }
            ComparisonType::LessThan => {
                if first_operand_value < second_operand_value {
                    self.registers.set_instruction_pointer(&address);
                }

                println!(
                    "Executed JLT: {:?} < {:?} -> {}",
                    first_operand_value, second_operand_value, instruction.bytecode_jump_index
                );
            }
            ComparisonType::LessThanOrEqual => {
                if first_operand_value <= second_operand_value {
                    self.registers.set_instruction_pointer(&address);
                }

                println!(
                    "Executed JLE: {:?} <= {:?} -> {}",
                    first_operand_value, second_operand_value, instruction.bytecode_jump_index
                );
            }
            ComparisonType::GreaterThan => {
                if first_operand_value > second_operand_value {
                    self.registers.set_instruction_pointer(&address);
                }

                println!(
                    "Executed JGT: {:?} > {:?} -> {}",
                    first_operand_value, second_operand_value, instruction.bytecode_jump_index
                );
            }
            ComparisonType::GreaterThanOrEqual => {
                if first_operand_value >= second_operand_value {
                    self.registers.set_instruction_pointer(&address);
                }

                println!(
                    "Executed JGE: {:?} >= {:?} -> {}",
                    first_operand_value, second_operand_value, instruction.bytecode_jump_index
                );
            }
        }
    }

    fn execute_output(&mut self, instruction: &OutputInstruction) {
        let source_operand_value = self.get_value(&instruction.source_operand);

        println!("Executed OUT: {}", source_operand_value);
    }

    fn execute(&mut self, instruction: &Instruction) {
        match instruction {
            Instruction::Move(instruction) => self.execute_move(instruction),
            Instruction::Add(instruction) => self.execute_add(instruction),
            Instruction::Sub(instruction) => self.execute_subtract(instruction),
            Instruction::Similarity(instruction) => self.execute_similarity(instruction),
            Instruction::JumpCompare(instruction) => self.execute_jump_compare(instruction),
            Instruction::Output(instruction) => self.execute_output(instruction),
        }
    }
}

pub struct Processor {
    control: ControlUnit,
}

impl Processor {
    pub fn new() -> Self {
        Processor {
            control: ControlUnit::new(),
        }
    }

    pub fn load_bytecode(&mut self, bytecode: Vec<u8>) {
        self.control.memory.load(bytecode);
    }

    pub fn run(&mut self) {
        while let Some(instruction) = self.control.fetch_and_decode() {
            self.control.execute(&instruction);
        }
    }
}
