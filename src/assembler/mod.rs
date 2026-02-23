use std::collections::HashMap;
use std::hash::{DefaultHasher, Hash, Hasher};

use crate::assembler::immediate::{Immediate, ImmediateType};
use crate::assembler::opcode::OpCode;
use crate::assembler::scanner::Scanner;
use crate::assembler::scanner::token::{Token, TokenType};

pub mod immediate;
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

    fn immediate(&mut self, message: &str) -> Result<Immediate, String> {
        let current_type = match self.current {
            Some(ref token) => token.token_type(),
            None => {
                return Err(format!(
                    "Failed to parse immediate value. Current token is None. {}",
                    message
                ));
            }
        };

        match current_type {
            TokenType::String => Ok(Immediate::Text(self.string(message).to_string())),
            TokenType::Number => {
                let number = self.number(message);

                if let Ok(number) = number {
                    return Ok(Immediate::Number(number));
                }

                Err(format!("Failed to parse immediate number. {}", message))
            }
            TokenType::Identifier => {
                let reg = self.register(message);
                if let Ok(register) = reg {
                    return Ok(Immediate::Register(register));
                }

                Err(format!("Failed to parse immediate register. {}", message))
            }
            _ => Err(format!(
                "Invalid immediate value. Expected number, string or register. Found token type: {:?}. {}",
                current_type, message
            )),
        }
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

    fn emit_register_bytecode(&mut self, register: u32) {
        self.text_segment.push(register.to_be_bytes());
    }

    fn emit_immediate_bytecode(&mut self, immediate: &Immediate) {
        let (immediate_type, value_be_bytes): (ImmediateType, Vec<[u8; 4]>) = match immediate {
            Immediate::Number(value) => (ImmediateType::Number, vec![value.to_be_bytes()]),
            Immediate::Register(value) => (ImmediateType::Register, vec![value.to_be_bytes()]),
            Immediate::Text(value) => (
                ImmediateType::Text,
                format!("{}\0", value)
                    .bytes()
                    .map(|byte| u32::from(byte).to_be_bytes())
                    .collect::<Vec<[u8; 4]>>(),
            ),
        };

        let immediate_type_be_bytes = match immediate_type.to_be_bytes() {
            Ok(bytes) => bytes,
            Err(message) => {
                self.error_at_current(message);

                return;
            }
        };

        self.text_segment.push(immediate_type_be_bytes);

        if let ImmediateType::Text = immediate_type {
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

            self.text_segment.push(value_be_bytes_address.to_be_bytes());

            self.data_segment.extend(value_be_bytes);
        } else {
            self.text_segment.extend(value_be_bytes);
        }
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

    fn load_immediate(&mut self) {
        self.consume(&TokenType::LoadImmediate, "Expected 'li' keyword.");

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

        let immediate = match self.immediate("Expected immediate after ','.") {
            Ok(immediate) => immediate,
            Err(message) => {
                self.error_at_current(&message);
                return;
            }
        };

        self.emit_op_code_bytecode(OpCode::LoadImmediate);
        self.emit_register_bytecode(destination_register);
        self.emit_immediate_bytecode(&immediate);
    }

    fn load_file(&mut self) {
        self.consume(&TokenType::LoadFile, "Expected 'lf' keyword.");

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

        let file_path = self
            .string("Expected file path string after ','.")
            .to_string();

        self.emit_op_code_bytecode(OpCode::LoadFile);
        self.emit_register_bytecode(destination_register);
        self.emit_immediate_bytecode(&Immediate::Text(file_path));
    }

    fn _move(&mut self) {
        self.consume(&TokenType::Move, "Expected 'mv' keyword.");

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

        let source_register = match self.register("Expected source register after ','.") {
            Ok(register) => register,
            Err(message) => {
                self.error_at_current(&message);
                return;
            }
        };

        self.emit_op_code_bytecode(OpCode::Move);
        self.emit_register_bytecode(destination_register);
        self.emit_register_bytecode(source_register);
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
                self.error_at_previous("Invalid opcode instruction.");
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

        let immediate_1 = match self.immediate("Expected immediate 1 after ','.") {
            Ok(immediate) => immediate,
            Err(message) => {
                self.error_at_current(&message);
                return;
            }
        };

        self.consume(&TokenType::Comma, "Expected ',' after immediate 1.");

        let immediate_2 = match self.immediate("Expected immediate 2 after ','.") {
            Ok(immediate) => immediate,
            Err(message) => {
                self.error_at_current(&message);
                return;
            }
        };

        self.emit_op_code_bytecode(opcode);
        self.emit_register_bytecode(destination_register);
        self.emit_immediate_bytecode(&immediate_1);
        self.emit_immediate_bytecode(&immediate_2);
    }

    fn branch(&mut self, token_type: &TokenType) {
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
                self.error_at_previous("Invalid branch instruction.");
                return;
            }
        };

        let immediate_1 = match self
            .immediate(format!("Expected immediate 1 after '{:?}' keyword.", token_type).as_str())
        {
            Ok(immediate) => immediate,
            Err(message) => {
                self.error_at_current(&message);
                return;
            }
        };

        self.consume(&TokenType::Comma, "Expected ',' after immediate 1.");

        let immediate_2 = match self.immediate("Expected immediate 2 after ','.") {
            Ok(immediate) => immediate,
            Err(message) => {
                self.error_at_current(&message);
                return;
            }
        };

        self.consume(&TokenType::Comma, "Expected ',' after source immediate 2.");

        let label_name = self.identifier("Expected label name after ','.");
        let key = Self::hash(label_name);

        self.emit_op_code_bytecode(opcode);
        self.emit_immediate_bytecode(&immediate_1);
        self.emit_immediate_bytecode(&immediate_2);
        self.emit_label_bytecode(key);
    }

    fn output(&mut self) {
        self.consume(&TokenType::Out, "Expected 'out' keyword.");

        let immediate = match self.immediate("Expected immediate after 'out'.") {
            Ok(immediate) => immediate,
            Err(message) => {
                self.error_at_current(&message);
                return;
            }
        };

        self.emit_op_code_bytecode(OpCode::Out);
        self.emit_immediate_bytecode(&immediate);
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
                    TokenType::LoadImmediate => self.load_immediate(),
                    TokenType::LoadFile => self.load_file(),
                    TokenType::Move => self._move(),
                    // Control flow.
                    TokenType::BranchEqual => self.branch(&TokenType::BranchEqual),
                    TokenType::BranchLess => self.branch(&TokenType::BranchLess),
                    TokenType::BranchLessEqual => self.branch(&TokenType::BranchLessEqual),
                    TokenType::BranchGreater => self.branch(&TokenType::BranchGreater),
                    TokenType::BranchGreaterEqual => self.branch(&TokenType::BranchGreaterEqual),
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

        let mut byte_code: Vec<[u8; 4]> = Vec::new();

        // Append the data segment size.
        let data_segment_size: u32 = match self.data_segment.len().try_into() {
            Ok(size) => size,
            Err(_) => {
                self.error_at_current(&format!(
                    "Failed to convert data segment size to u32. Data segment size exceeds {}. Found data segment size: {}.",
                    u32::MAX,
                    self.data_segment.len()
                ));
                return Err("Assembly failed due to errors.");
            }
        };
        byte_code.push(data_segment_size.to_be_bytes());

        // Append the text segment size.
        let text_segment_size: u32 = match self.text_segment.len().try_into() {
            Ok(size) => size,
            Err(_) => {
                self.error_at_current(&format!(
                    "Failed to convert text segment size to u32. Text segment size exceeds {}. Found text segment size: {}.",
                    u32::MAX,
                    self.text_segment.len()
                ));
                return Err("Assembly failed due to errors.");
            }
        };
        byte_code.push(text_segment_size.to_be_bytes());

        // Append the data segment.
        byte_code.extend(&self.data_segment);

        // Append the text segment.
        byte_code.extend(&self.text_segment);

        Ok(byte_code.into_iter().flatten().collect())
    }
}
