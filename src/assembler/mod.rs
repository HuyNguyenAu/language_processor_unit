use std::collections::HashMap;

use crate::assembler::opcode::OpCode;
use crate::assembler::scanner::Scanner;
use crate::assembler::scanner::token::{Token, TokenType};

pub mod opcode;
mod scanner;

const HEADER_SIZE: u32 = 2;

struct UnresolvedLabel {
    indices: Vec<usize>,
    token: Token,
}

pub struct Assembler {
    data_segment: Vec<[u8; 4]>,
    text_segment: Vec<[u8; 4]>,

    source: &'static str,
    scanner: Scanner,

    previous: Option<Token>,
    current: Option<Token>,

    labels: HashMap<String, usize>,
    unresolved_labels: HashMap<String, UnresolvedLabel>,

    had_error: bool,
    panic_mode: bool,
}

impl Assembler {
    pub fn new(source: &'static str) -> Self {
        Assembler {
            data_segment: Vec::new(),
            text_segment: Vec::new(),
            source,
            scanner: Scanner::new(source),
            previous: None,
            current: None,
            labels: HashMap::new(),
            unresolved_labels: HashMap::new(),
            had_error: false,
            panic_mode: false,
        }
    }

    fn lexeme(&self, token: &Token) -> &str {
        &self.source[token.start()..token.end()]
    }

    fn error_at(&mut self, token: &Token, message: &str) {
        if self.panic_mode {
            return;
        }

        self.panic_mode = true;

        eprint!("[Line {}:{}] Error:", token.line(), token.column());

        if token.token_type() == &TokenType::Error
            && let Some(error) = token.error()
        {
            eprint!(" {}", error);
        }

        eprint!(" at '{}'.", self.lexeme(token));
        eprintln!(" {}", message);

        self.had_error = true;
    }

    fn error_at_current(&mut self, message: &str) {
        if let Some(token) = &self.current {
            let token = token.clone();
            self.error_at(&token, message);
        } else {
            panic!(
                "Failed to handle error at current token.\nError: {}",
                message
            );
        }
    }

    fn error_at_previous(&mut self, message: &str) {
        if let Some(token) = &self.previous {
            let token = token.clone();
            self.error_at(&token, message);
        } else {
            panic!(
                "Failed to handle error at previous token.\nError: {}",
                message
            );
        }
    }

    fn advance(&mut self) {
        self.previous = self.current.clone();

        let token = self.scanner.scan_token();
        self.current = Some(token.clone());

        if token.token_type() == &TokenType::Error {
            self.error_at_current("Failed to advance to next token due to scanning error.");
        }
    }

    fn previous_lexeme(&self) -> &str {
        if let Some(token) = &self.previous {
            return self.lexeme(token);
        }

        panic!("Expected previous token to be present, but it is None.");
    }

    fn consume(&mut self, token_type: &TokenType, message: &str) {
        if let Some(current_token) = &self.current
            && current_token.token_type() == token_type
        {
            self.advance();

            return;
        }

        self.error_at_current(message);
    }

    fn number(&mut self, message: &str) -> Result<u32, String> {
        self.consume(&TokenType::Number, message);

        match self.previous_lexeme().parse() {
            Ok(value) => Ok(value),
            Err(_) => Err(format!(
                "Failed to parse number from lexeme '{}'.",
                self.previous_lexeme()
            )),
        }
    }

    fn register(&mut self, message: &str) -> Result<u32, String> {
        self.consume(&TokenType::Identifier, message);

        let lexeme = self.previous_lexeme();

        // Ensure the lexeme starts with "x".
        if !lexeme.to_lowercase().starts_with("x") {
            return Err(format!(
                "Invalid register format: '{}'. Expected format: 'xN' where N is a number between 1 and 32.",
                lexeme
            ));
        }

        let register_number = match lexeme[1..].parse::<u32>() {
            Ok(value) => value,
            Err(_) => {
                return Err(format!(
                    "Failed to parse register number from lexeme '{}'. Expected format: 'xN' where N is a number between 1 and 32.",
                    lexeme
                ));
            }
        };

        if !(1..=32).contains(&register_number) {
            return Err(format!(
                "Register number out of range: '{}'. Expected format: 'xN' where N is a number between 1 and 32.",
                register_number
            ));
        }

        Ok(register_number)
    }

    fn string(&mut self, message: &str) -> String {
        self.consume(&TokenType::String, message);

        let lexeme = self.previous_lexeme();

        // Remove surrounding quotes.
        lexeme
            .chars()
            .skip(1)
            .take(lexeme.chars().count() - 2)
            .collect::<String>()
            .replace("\\n", "\n")
            .replace("\\\"", "\"")
    }

    fn identifier(&mut self, message: &str) -> &str {
        self.consume(&TokenType::Identifier, message);

        self.previous_lexeme()
    }

    fn emit_number(&mut self, value: u32) {
        self.text_segment.push(value.to_be_bytes());
    }

    fn emit_opcode(&mut self, op_code: OpCode) {
        self.emit_number(op_code.into());
    }

    fn emit_string_bytecode(&mut self, value: &str) -> u32 {
        let nulled_value = format!("{}\0", value);
        let words: Vec<[u8; 4]> = nulled_value
            .bytes()
            .map(|b| u32::from(b).to_be_bytes())
            .collect();

        let address: u32 = match self.data_segment.len().try_into() {
            Ok(address) => address,
            Err(_) => {
                self.error_at_current(&format!(
                    "Failed to convert data segment length to u32. Data segment length exceeds {}. Found data segment length: {}.",
                    u32::MAX,
                    self.data_segment.len()
                ));
                return 0;
            }
        };

        self.data_segment.extend(words);

        address
    }

    fn upsert_unresolved_label(&mut self, key: String) -> Result<(), String> {
        let index = self.text_segment.len() - 1;

        if let Some(label) = self.unresolved_labels.get_mut(&key) {
            label.indices.push(index);
        } else {
            let previous_token = self
                .previous
                .clone()
                .ok_or_else(|| "Failed to get current token for unresolved label.".to_string())?;

            self.unresolved_labels.insert(
                key,
                UnresolvedLabel {
                    indices: vec![index],
                    token: previous_token,
                },
            );
        }

        Ok(())
    }

    fn emit_label_bytecode(&mut self, key: String) {
        // Placeholder for backpatching.
        self.emit_number(0);

        // Record the current bytecode index for backpatching later.
        if let Err(message) = self.upsert_unresolved_label(key) {
            self.error_at_current(&message);
        }
    }

    fn l_type(&mut self, token_type: &TokenType) {
        self.consume(
            token_type,
            format!("Expected '{:?}' keyword.", token_type).as_str(),
        );

        let opcode = match token_type {
            TokenType::LoadString => OpCode::LoadString,
            TokenType::LoadImmediate => OpCode::LoadImmediate,
            TokenType::LoadFile => OpCode::LoadFile,
            TokenType::Move => OpCode::Move,
            _ => {
                self.error_at_previous("Invalid l-type opcode instruction.");
                return;
            }
        };

        let destination_register = match self.register("Expected destination register.") {
            Ok(register) => register,
            Err(message) => {
                self.error_at_current(&message);
                return;
            }
        };

        self.consume(
            &TokenType::Comma,
            "Expected ',' after destination register.",
        );

        self.emit_opcode(opcode);
        self.emit_number(destination_register);

        match opcode {
            OpCode::LoadImmediate => {
                let immediate = match self.number("Expected immediate after ','.") {
                    Ok(value) => value,
                    Err(message) => {
                        self.error_at_current(&message);
                        return;
                    }
                };

                self.emit_number(immediate);
                self.emit_number(0); // Pad the instruction to 16 bytes.
            }
            OpCode::Move => {
                let source_register = match self.register("Expected source register after ','.") {
                    Ok(register) => register,
                    Err(message) => {
                        self.error_at_current(&message);
                        return;
                    }
                };

                self.emit_number(source_register);
                self.emit_number(0); // Pad the instruction to 16 bytes.
            }
            _ => {
                let value = self.string("Expected string after ','.");
                let pointer = self.emit_string_bytecode(&value);

                self.emit_number(pointer);
                self.emit_number(0); // Pad the instruction to 16 bytes.
            }
        }
    }

    fn label(&mut self) {
        self.consume(&TokenType::Label, "Expected label name.");

        let label_name = self.previous_lexeme();
        let value = label_name.trim_end_matches(':').to_string();
        let byte_code_index = self.text_segment.len();

        self.labels.insert(value, byte_code_index);
    }

    fn r_type(&mut self, token_type: &TokenType) {
        self.consume(
            token_type,
            format!("Expected '{:?}' keyword.", token_type).as_str(),
        );

        let opcode = match token_type {
            // Generative operations.
            TokenType::Morph => OpCode::Morph,
            TokenType::Project => OpCode::Project,
            // Cognitive operations.
            TokenType::Distill => OpCode::Distill,
            TokenType::Correlate => OpCode::Correlate,
            // Guardrails operations.
            TokenType::Audit => OpCode::Audit,
            TokenType::Similarity => OpCode::Similarity,
            _ => {
                self.error_at_previous("Invalid r-type opcode instruction.");
                return;
            }
        };

        let destination_register = match self.register(
            format!(
                "Expected destination register after '{:?}' keyword.",
                token_type
            )
            .as_str(),
        ) {
            Ok(register) => register,
            Err(message) => {
                self.error_at_current(&message);
                return;
            }
        };

        self.consume(
            &TokenType::Comma,
            "Expected ',' after destination register.",
        );

        let source_register_1 = match self.register("Expected source register 1 after ','.") {
            Ok(register) => register,
            Err(message) => {
                self.error_at_current(&message);
                return;
            }
        };

        self.consume(&TokenType::Comma, "Expected ',' after source register 1.");

        let source_register_2 = match self.register("Expected source register 2 after ','.") {
            Ok(register) => register,
            Err(message) => {
                self.error_at_current(&message);
                return;
            }
        };

        self.emit_opcode(opcode);
        self.emit_number(destination_register);
        self.emit_number(source_register_1);
        self.emit_number(source_register_2);
    }

    fn b_type(&mut self, token_type: &TokenType) {
        self.consume(
            token_type,
            format!("Expected '{:?}' keyword.", token_type).as_str(),
        );

        let opcode = match token_type {
            TokenType::BranchEqual => OpCode::BranchEqual,
            TokenType::BranchLess => OpCode::BranchLess,
            TokenType::BranchLessEqual => OpCode::BranchLessEqual,
            TokenType::BranchGreater => OpCode::BranchGreater,
            TokenType::BranchGreaterEqual => OpCode::BranchGreaterEqual,
            _ => {
                self.error_at_previous("Invalid b-type opcode instruction.");
                return;
            }
        };

        let source_register_1 = match self.register(
            format!(
                "Expected source register 1 after '{:?}' keyword.",
                token_type
            )
            .as_str(),
        ) {
            Ok(register) => register,
            Err(message) => {
                self.error_at_current(&message);
                return;
            }
        };

        self.consume(&TokenType::Comma, "Expected ',' after source register 1.");

        let source_register_2 = match self.register("Expected source register 2 after ','.") {
            Ok(register) => register,
            Err(message) => {
                self.error_at_current(&message);
                return;
            }
        };

        self.consume(&TokenType::Comma, "Expected ',' after source register 2.");

        let label_name = self
            .identifier("Expected label name after ','.")
            .to_string();
        let key = label_name;

        self.emit_opcode(opcode);
        self.emit_number(source_register_1);
        self.emit_number(source_register_2);
        self.emit_label_bytecode(key);
    }

    fn output(&mut self) {
        self.consume(&TokenType::Out, "Expected 'out' keyword.");

        let source_register = match self.register("Expected source register after 'out'.") {
            Ok(register) => register,
            Err(message) => {
                self.error_at_current(&message);
                return;
            }
        };

        self.emit_opcode(OpCode::Out);
        self.emit_number(source_register);
        self.emit_number(0); // Pad the instruction to 16 bytes.
        self.emit_number(0); // Pad the instruction to 16 bytes.
    }

    fn exit(&mut self) {
        self.consume(&TokenType::Exit, "Expected 'exit' keyword.");

        self.emit_opcode(OpCode::Exit);
        self.emit_number(0); // Pad the instruction to 16 bytes.
        self.emit_number(0); // Pad the instruction to 16 bytes.
        self.emit_number(0); // Pad the instruction to 16 bytes.
    }

    fn backpatch_labels(&mut self) {
        let mut resolved_labels: Vec<String> = Vec::new();

        for (key, unresolved) in &self.unresolved_labels {
            if let Some(byte_code_index) = self.labels.get(key) {
                let index: u32 = match (*byte_code_index).try_into() {
                    Ok(value) => value,
                    Err(_) => {
                        self.error_at_current(format!(
                            "Failed to convert bytecode index to u32 for backpatching. Bytecode index exceeds {}. Found bytecode index: {}.",
                            u32::MAX,
                            byte_code_index
                        ).as_str());
                        return;
                    }
                };

                let bytes = (HEADER_SIZE + index).to_be_bytes();

                for idx in &unresolved.indices {
                    self.text_segment[*idx] = bytes;
                }

                resolved_labels.push(key.clone());
            }
        }

        for key in resolved_labels {
            self.unresolved_labels.remove(&key);
        }
    }

    pub fn assemble(&mut self) -> Result<Vec<u8>, &'static str> {
        self.advance();

        while !self.panic_mode {
            if let Some(current_token) = &self.current {
                match current_token.token_type() {
                    // Data movement.
                    TokenType::LoadString => self.l_type(&TokenType::LoadString),
                    TokenType::LoadImmediate => self.l_type(&TokenType::LoadImmediate),
                    TokenType::LoadFile => self.l_type(&TokenType::LoadFile),
                    TokenType::Move => self.l_type(&TokenType::Move),
                    // Control flow.
                    TokenType::BranchEqual => self.b_type(&TokenType::BranchEqual),
                    TokenType::BranchLess => self.b_type(&TokenType::BranchLess),
                    TokenType::BranchLessEqual => self.b_type(&TokenType::BranchLessEqual),
                    TokenType::BranchGreater => self.b_type(&TokenType::BranchGreater),
                    TokenType::BranchGreaterEqual => self.b_type(&TokenType::BranchGreaterEqual),
                    TokenType::Exit => self.exit(),
                    TokenType::Label => self.label(),
                    // I/O.
                    TokenType::Out => self.output(),
                    // Generative operations.
                    TokenType::Morph => self.r_type(&TokenType::Morph),
                    TokenType::Project => self.r_type(&TokenType::Project),
                    // Cognitive operations.
                    TokenType::Distill => self.r_type(&TokenType::Distill),
                    TokenType::Correlate => self.r_type(&TokenType::Correlate),
                    // Guardrails operations.
                    TokenType::Audit => self.r_type(&TokenType::Audit),
                    TokenType::Similarity => self.r_type(&TokenType::Similarity),
                    // Misc.
                    TokenType::Eof => break,
                    _ => self.error_at_current("Unexpected keyword."),
                }
            } else {
                self.error_at_current("Unexpected end of input. Expected more tokens.");
            }
        }

        if self.had_error {
            return Err("Assembly failed due to errors.");
        }

        self.backpatch_labels();

        if let Some((_, unresolved_label)) = self.unresolved_labels.iter().next() {
            let token = unresolved_label.token.clone();

            self.error_at(&token, "Undefined label referenced here.");

            return Err("Assembly failed due to errors.");
        }

        let mut byte_code: Vec<[u8; 4]> = Vec::new();

        // Text segment starts after the header.
        byte_code.push(HEADER_SIZE.to_be_bytes());

        // Data segment starts after the header and text segment.
        let text_segment_size: u32 = match self.text_segment.len().try_into() {
            Ok(size) => size,
            Err(_) => {
                self.error_at_current(&format!(
                    "Failed to convert text segment size to u32. Text segment size exceeds {}. Found text segment size: {}",
                    u32::MAX,
                    self.text_segment.len()
                ));
                return Err("Assembly failed due to errors.");
            }
        };

        byte_code.push((HEADER_SIZE + text_segment_size).to_be_bytes());

        // Append the text segment.
        byte_code.extend(&self.text_segment);

        // Append the data segment after the text segment.
        byte_code.extend(&self.data_segment);

        Ok(byte_code.into_iter().flatten().collect())
    }
}
