use std::collections::HashMap;
use std::hash::{DefaultHasher, Hash, Hasher};

use crate::assembler::opcode::OpCode;
use crate::assembler::scanner::Scanner;
use crate::assembler::scanner::token::{Token, TokenType};

pub mod opcode;
mod scanner;

struct UnresolvedLabel {
    bytecode_indices: Vec<usize>,
    token: Token,
}

pub struct Assembler {
    data_segment: Vec<[u8; 4]>,
    text_segment: Vec<[u8; 4]>,

    source: &'static str,
    scanner: Scanner,

    previous: Option<Token>,
    current: Option<Token>,

    byte_code_indices: HashMap<u64, usize>,
    unresolved_labels: HashMap<u64, UnresolvedLabel>,

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
            byte_code_indices: HashMap::new(),
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

    fn emit_number_bytecode(&mut self, value: u32) {
        self.text_segment.push(value.to_be_bytes());
    }

    fn emit_op_code_bytecode(&mut self, op_code: OpCode) {
        let op_code_be_bytes = match op_code.to_be_bytes() {
            Ok(bytes) => bytes,
            Err(message) => {
                self.error_at_current(message);
                return;
            }
        };

        self.text_segment.push(op_code_be_bytes);
    }

    fn emit_string_bytecode(&mut self, value: &str) {
        let value_be_bytes = format!("{}\0", value)
            .bytes()
            .map(|byte| u32::from(byte).to_be_bytes())
            .collect::<Vec<[u8; 4]>>();
        let value_be_bytes_address: u32 = match self.data_segment.len().try_into() {
            Ok(length) => length,
            _ => {
                self.error_at_current(&format!(
                    "Failed to convert data segment length to u32. Data segment length exceeds {}. Found data segment length: {}.",
                    u32::MAX,
                    self.data_segment.len()
                ));
                return;
            }
        };

        self.data_segment.extend(value_be_bytes);
        self.text_segment.push(value_be_bytes_address.to_be_bytes());
    }

    fn upsert_unresolved_label(&mut self, key: u64) -> Result<(), String> {
        let bytecode_index = self.text_segment.len() - 1;

        if let Some(label) = self.unresolved_labels.get_mut(&key) {
            label.bytecode_indices.push(bytecode_index);
        } else {
            let previous_token = match &self.previous {
                Some(token) => token.clone(),
                None => {
                    return Err("Failed to get current token for unresolved label.".to_string());
                }
            };

            self.unresolved_labels.insert(
                key,
                UnresolvedLabel {
                    bytecode_indices: vec![bytecode_index],
                    token: previous_token,
                },
            );
        }

        Ok(())
    }

    fn emit_label_bytecode(&mut self, key: u64) {
        if let Some(index) = self.byte_code_indices.get(&key) {
            let value: u32 = match (*index).try_into() {
                Ok(value) => value,
                Err(_) => {
                    self.error_at_current(format!(
                    "Failed to convert bytecode index to u32 for jump compare. Bytecode index exceeds {}. Found bytecode index: {}.",
                    u32::MAX,
                    index
                ).as_str());
                    return;
                }
            };
            self.emit_number_bytecode(value);
        } else {
            // Placeholder for backpatching.
            self.emit_number_bytecode(0);
            // Record the current bytecode index for backpatching later.
            match self.upsert_unresolved_label(key) {
                Ok(()) => (),
                Err(message) => {
                    self.error_at_current(&message);
                }
            }
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

        if matches!(opcode, OpCode::LoadImmediate) {
            let immediate = match self.number("Expected immediate after ','.") {
                Ok(immediate) => immediate,
                Err(message) => {
                    self.error_at_current(&message);
                    return;
                }
            };

            self.emit_op_code_bytecode(opcode);
            self.emit_number_bytecode(destination_register);
            self.emit_number_bytecode(immediate);
            self.emit_number_bytecode(0); // Padding for uniform instruction size.
        } else if matches!(opcode, OpCode::Move) {
            let source_register = match self.register("Expected source register after ','.") {
                Ok(register) => register,
                Err(message) => {
                    self.error_at_current(&message);
                    return;
                }
            };

            self.emit_op_code_bytecode(opcode);
            self.emit_number_bytecode(destination_register);
            self.emit_number_bytecode(source_register);
            self.emit_number_bytecode(0); // Padding for uniform instruction size.
        } else {
            let string_value = self.string("Expected string after ','.");

            self.emit_op_code_bytecode(opcode);
            self.emit_number_bytecode(destination_register);
            self.emit_string_bytecode(&string_value);
            self.emit_number_bytecode(0); // Padding for uniform instruction size.
        }
    }

    fn hash(value: &str) -> u64 {
        let mut hasher = DefaultHasher::new();
        value.hash(&mut hasher);

        hasher.finish()
    }

    fn label(&mut self) {
        self.consume(&TokenType::Label, "Expected label name.");

        let label_name = self.previous_lexeme();
        let value = label_name.trim_end_matches(':');
        let key = Self::hash(value);
        let jump_destination_byte_code_index = self.text_segment.len();

        // Backpatch any unresolved labels.
        if let Some(unresolved) = self.unresolved_labels.remove(&key) {
            let value: u32 = match jump_destination_byte_code_index.try_into() {
                Ok(value) => value,
                Err(_) => {
                    self.error_at_current(&format!("Failed to convert bytecode index to u32 for backpatching. Bytecode index exceeds {}. Found bytecode index: {}.", u32::MAX, jump_destination_byte_code_index ));

                    return;
                }
            };

            let bytes = value.to_be_bytes();

            for idx in unresolved.bytecode_indices {
                self.text_segment[idx] = bytes;
            }
        }

        self.byte_code_indices
            .insert(key, jump_destination_byte_code_index);
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

        self.emit_op_code_bytecode(opcode);
        self.emit_number_bytecode(destination_register);
        self.emit_number_bytecode(source_register_1);
        self.emit_number_bytecode(source_register_2);
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

        let label_name = self.identifier("Expected label name after ','.");
        let key = Self::hash(label_name);

        self.emit_op_code_bytecode(opcode);
        self.emit_number_bytecode(source_register_1);
        self.emit_number_bytecode(source_register_2);
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

        self.emit_op_code_bytecode(OpCode::Out);
        self.emit_number_bytecode(source_register);
        self.emit_number_bytecode(0); // Padding for uniform instruction size.
        self.emit_number_bytecode(0); // Padding for uniform instruction size.
    }

    fn exit(&mut self) {
        self.consume(&TokenType::Exit, "Expected 'exit' keyword.");

        self.emit_op_code_bytecode(OpCode::Exit);
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

        if let Some((_, unresolved_label)) = self.unresolved_labels.iter().nth(0) {
            let token = unresolved_label.token.clone();

            self.error_at(&token, "Undefined label referenced here.");

            return Err("Assembly failed due to errors.");
        }

        let header_size = 2_u32;
        let mut byte_code: Vec<[u8; 4]> = Vec::new();

        // Text segment starts after the header.
        byte_code.push(header_size.to_be_bytes());

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

        byte_code.push((header_size + text_segment_size).to_be_bytes());

        // Append the text segment.
        byte_code.extend(&self.text_segment);

        // Append the data segment after the text segment.
        byte_code.extend(&self.data_segment);

        Ok(byte_code.into_iter().flatten().collect())
    }
}
