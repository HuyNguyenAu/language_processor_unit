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
    byte_code: Vec<[u8; 4]>,

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
            byte_code: Vec::new(),
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

        if token.token_type() == &TokenType::ERROR
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

        loop {
            let token = self.scanner.scan_token();
            self.current = Some(token);

            if self.current.as_ref().unwrap().token_type() != &TokenType::ERROR {
                return;
            }

            self.error_at_current("");
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
        self.consume(&TokenType::NUMBER, message);

        return match self.previous_lexeme().parse() {
            Ok(value) => Ok(value),
            Err(_) => Err(format!(
                "Failed to parse number from lexeme '{}'.",
                self.previous_lexeme()
            )),
        };
    }

    fn register(&mut self, message: &str) -> Result<u32, String> {
        self.consume(&TokenType::IDENTIFIER, message);

        let lexeme = self.previous_lexeme();

        // Ensure the lexeme starts with "x".
        if !lexeme.to_lowercase().starts_with("x") {
            return Err(format!(
                "Invalid register format: '{}'. Expected format: 'xN' where N is a number between 1 and 32.",
                lexeme
            ));
        }

        let register_number = match u32::from_str_radix(&lexeme[1..], 10) {
            Ok(value) => value,
            Err(_) => {
                return Err(format!(
                    "Failed to parse register number from lexeme '{}'. Expected format: 'xN' where N is a number between 1 and 32.",
                    lexeme
                ));
            }
        };

        if register_number < 1 || register_number > 32 {
            return Err(format!(
                "Register number out of range: '{}'. Expected format: 'xN' where N is a number between 1 and 32.",
                register_number
            ));
        }

        return Ok(register_number);
    }

    fn string(&mut self, message: &str) -> String {
        self.consume(&TokenType::STRING, message);

        let lexeme = self.previous_lexeme();

        // Remove surrounding quotes.
        return lexeme
            .chars()
            .skip(1)
            .take(lexeme.chars().count() - 2)
            .collect::<String>()
            .replace("\\n", "\n");
    }

    fn identifier(&mut self, message: &str) -> &str {
        self.consume(&TokenType::IDENTIFIER, message);

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

        return match current_type {
            TokenType::STRING => Ok(Immediate::Text(self.string(message).to_string())),
            TokenType::NUMBER => {
                let number = self.number(message);

                if let Ok(number) = number {
                    return Ok(Immediate::Number(number));
                }

                return Err(format!("Failed to parse immediate number. {}", message));
            }
            TokenType::IDENTIFIER => {
                let reg = self.register(message);
                if let Ok(register) = reg {
                    return Ok(Immediate::Register(register));
                }

                return Err(format!("Failed to parse immediate register. {}", message));
            }
            _ => {
                return Err(format!(
                    "Invalid immediate value. Expected number, string or register. Found token type: {:?}. {}",
                    current_type, message
                ));
            }
        };
    }

    fn emit_number_bytecode(&mut self, value: u32) {
        self.byte_code.push(value.to_be_bytes());
    }

    fn emit_op_code_bytecode(&mut self, op_code: OpCode) {
        let op_code_be_bytes = match op_code.to_be_bytes() {
            Ok(bytes) => bytes,
            Err(message) => {
                self.error_at_current(&message);
                return;
            }
        };

        self.byte_code.push(op_code_be_bytes);
    }

    fn emit_register_bytecode(&mut self, register: u32) {
        self.byte_code.push(register.to_be_bytes());
    }

    fn emit_immediate_bytecode(&mut self, immediate: &Immediate) {
        let (immediate_type, value_be_bytes): (ImmediateType, Vec<[u8; 4]>) = match immediate {
            Immediate::Number(value) => (ImmediateType::NUMBER, vec![value.to_be_bytes()]),
            Immediate::Register(value) => (ImmediateType::REGISTER, vec![value.to_be_bytes()]),
            Immediate::Text(value) => (
                ImmediateType::TEXT,
                value
                    .bytes()
                    .map(|byte| u32::from(byte).to_be_bytes())
                    .collect::<Vec<[u8; 4]>>(),
            ),
        };

        let immediate_type_be_bytes = match immediate_type.to_be_bytes() {
            Ok(bytes) => bytes,
            Err(message) => {
                self.error_at_current(&message);

                return;
            }
        };

        self.byte_code.push(immediate_type_be_bytes);

        let value_be_bytes_len: u32 = match value_be_bytes.len().try_into() {
            Ok(length) => length,
            _ => {
                self.error_at_current(&format!(
                    "Failed to convert immediate byte length to u32. Byte length exceeds {}. Found byte length: {}.",
                    u32::MAX,
                    value_be_bytes.len()
                ));

                return;
            }
        };

        self.byte_code.push(value_be_bytes_len.to_be_bytes());
        self.byte_code.extend(value_be_bytes);
    }

    fn upsert_unresolved_label(&mut self, key: u64) -> Result<(), String> {
        let bytecode_index = self.byte_code.len() - 1;

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
                    return;
                }
            }
        }
    }

    fn load_immediate(&mut self) {
        self.consume(&TokenType::LI, "Expected 'li' keyword.");

        let destination_register = match self.register("Expected destination register.") {
            Ok(register) => register,
            Err(message) => {
                self.error_at_current(&message);
                return;
            }
        };

        self.consume(
            &TokenType::COMMA,
            "Expected ',' after destination register.",
        );

        let immediate = match self.immediate("Expected immediate after ','.") {
            Ok(immediate) => immediate,
            Err(message) => {
                self.error_at_current(&message);
                return;
            }
        };

        self.emit_op_code_bytecode(OpCode::LI);
        self.emit_register_bytecode(destination_register);
        self.emit_immediate_bytecode(&immediate);
    }

    fn load_file(&mut self) {
        self.consume(&TokenType::LF, "Expected 'lf' keyword.");

        let destination_register = match self.register("Expected destination register.") {
            Ok(register) => register,
            Err(message) => {
                self.error_at_current(&message);
                return;
            }
        };

        self.consume(
            &TokenType::COMMA,
            "Expected ',' after destination register.",
        );

        let file_path = self
            .string("Expected file path string after ','.")
            .to_string();

        self.emit_op_code_bytecode(OpCode::LF);
        self.emit_register_bytecode(destination_register);
        self.emit_immediate_bytecode(&Immediate::Text(file_path));
    }

    fn move_value(&mut self) {
        self.consume(&TokenType::MV, "Expected 'mv' keyword.");

        let destination_register = match self.register("Expected destination register.") {
            Ok(register) => register,
            Err(message) => {
                self.error_at_current(&message);
                return;
            }
        };

        self.consume(
            &TokenType::COMMA,
            "Expected ',' after destination register.",
        );

        let source_register = match self.register("Expected source register after ','.") {
            Ok(register) => register,
            Err(message) => {
                self.error_at_current(&message);
                return;
            }
        };

        self.emit_op_code_bytecode(OpCode::MV);
        self.emit_register_bytecode(destination_register);
        self.emit_register_bytecode(source_register);
    }

    fn hash(value: &str) -> u64 {
        let mut hasher = DefaultHasher::new();
        value.hash(&mut hasher);

        return hasher.finish();
    }

    fn label(&mut self) {
        self.consume(&TokenType::LABEL, "Expected label name.");

        let label_name = self.previous_lexeme();
        let value = label_name.trim_end_matches(':');
        let key = Self::hash(value);
        let jump_destination_byte_code_index = self.byte_code.len();

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
                self.byte_code[idx] = bytes;
            }
        }

        self.byte_code_indices
            .insert(key, jump_destination_byte_code_index);
    }

    fn semantic_heuristic(&mut self, token_type: &TokenType) {
        self.consume(
            token_type,
            format!("Expected '{:?}' keyword.", token_type).as_str(),
        );

        let opcode = match token_type {
            // Semantic operations.
            TokenType::ADD => OpCode::ADD,
            TokenType::SUB => OpCode::SUB,
            TokenType::MUL => OpCode::MUL,
            TokenType::DIV => OpCode::DIV,
            TokenType::INF => OpCode::INF,
            TokenType::ADT => OpCode::ADT,
            // Heuristic operations.
            TokenType::EQV => OpCode::EQV,
            TokenType::INT => OpCode::INT,
            TokenType::HAL => OpCode::HAL,
            TokenType::SIM => OpCode::SIM,
            _ => {
                self.error_at_previous("Invalid semantic instruction.");
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
            &TokenType::COMMA,
            "Expected ',' after destination register.",
        );

        let immediate_1 = match self.immediate("Expected immediate 1 after ','.") {
            Ok(immediate) => immediate,
            Err(message) => {
                self.error_at_current(&message);
                return;
            }
        };

        let immediate_2 = if matches!(token_type, TokenType::HAL) {
            // HAL only takes one source operand; use numeric 0 as a dummy immediate.
            Immediate::Number(0)
        } else {
            self.consume(&TokenType::COMMA, "Expected ',' after immediate 1.");

            match self.immediate("Expected immediate 2 after ','.") {
                Ok(immediate) => immediate,
                Err(message) => {
                    self.error_at_current(&message);
                    return;
                }
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
            TokenType::BEQ => OpCode::BEQ,
            TokenType::BLT => OpCode::BLT,
            TokenType::BLE => OpCode::BLE,
            TokenType::BGT => OpCode::BGT,
            TokenType::BGE => OpCode::BGE,
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

        self.consume(&TokenType::COMMA, "Expected ',' after immediate 1.");

        let immediate_2 = match self.immediate("Expected immediate 2 after ','.") {
            Ok(immediate) => immediate,
            Err(message) => {
                self.error_at_current(&message);
                return;
            }
        };

        self.consume(&TokenType::COMMA, "Expected ',' after source immediate 2.");

        let label_name = self.identifier("Expected label name after ','.");
        let key = Self::hash(&label_name);

        self.emit_op_code_bytecode(opcode);
        self.emit_immediate_bytecode(&immediate_1);
        self.emit_immediate_bytecode(&immediate_2);
        self.emit_label_bytecode(key);
    }

    fn output(&mut self) {
        self.consume(&TokenType::OUT, "Expected 'out' keyword.");

        let immediate = match self.immediate("Expected immediate after 'out'.") {
            Ok(immediate) => immediate,
            Err(message) => {
                self.error_at_current(&message);
                return;
            }
        };

        self.emit_op_code_bytecode(OpCode::OUT);
        self.emit_immediate_bytecode(&immediate);
    }

    fn exit(&mut self) {
        self.consume(&TokenType::EXIT, "Expected 'exit' keyword.");

        self.emit_op_code_bytecode(OpCode::EXIT);
    }

    pub fn assemble(&mut self) -> Result<Vec<u8>, &'static str> {
        self.advance();

        while !self.panic_mode {
            if let Some(current_token) = &self.current {
                match current_token.token_type() {
                    // Data movement.
                    TokenType::LI => self.load_immediate(),
                    TokenType::LF => self.load_file(),
                    TokenType::MV => self.move_value(),
                    // Semantic operations.
                    TokenType::ADD => self.semantic_heuristic(&TokenType::ADD),
                    TokenType::SUB => self.semantic_heuristic(&TokenType::SUB),
                    TokenType::MUL => self.semantic_heuristic(&TokenType::MUL),
                    TokenType::DIV => self.semantic_heuristic(&TokenType::DIV),
                    TokenType::INF => self.semantic_heuristic(&TokenType::INF),
                    TokenType::ADT => self.semantic_heuristic(&TokenType::ADT),
                    // Heuristic operations.
                    TokenType::EQV => self.semantic_heuristic(&TokenType::EQV),
                    TokenType::INT => self.semantic_heuristic(&TokenType::INT),
                    TokenType::HAL => self.semantic_heuristic(&TokenType::HAL),
                    TokenType::SIM => self.semantic_heuristic(&TokenType::SIM),
                    // Control flow.
                    TokenType::BEQ => self.branch(&TokenType::BEQ),
                    TokenType::BLT => self.branch(&TokenType::BLT),
                    TokenType::BLE => self.branch(&TokenType::BLE),
                    TokenType::BGT => self.branch(&TokenType::BGT),
                    TokenType::BGE => self.branch(&TokenType::BGE),
                    TokenType::LABEL => self.label(),
                    // I/O.
                    TokenType::OUT => self.output(),
                    // Misc.
                    TokenType::EXIT => self.exit(),
                    TokenType::EOF => break,
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

        Ok(self
            .byte_code
            .iter()
            .flat_map(|bytes| bytes.iter())
            .copied()
            .collect())
    }
}
