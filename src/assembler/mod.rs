use std::collections::HashMap;

use crate::assembler::opcode::OpCode;
use crate::assembler::scanner::Scanner;
use crate::assembler::scanner::token::{Token, TokenType};

pub mod opcode;
mod scanner;

const HEADER_SIZE: u32 = 2;

impl TryFrom<TokenType> for OpCode {
    type Error = String;

    fn try_from(token_type: TokenType) -> Result<Self, Self::Error> {
        match token_type {
            // Data movement.
            TokenType::LoadString => Ok(OpCode::LoadString),
            TokenType::LoadImmediate => Ok(OpCode::LoadImmediate),
            TokenType::LoadFile => Ok(OpCode::LoadFile),
            TokenType::Move => Ok(OpCode::Move),
            // Control flow.
            TokenType::BranchEqual => Ok(OpCode::BranchEqual),
            TokenType::BranchLessEqual => Ok(OpCode::BranchLessEqual),
            TokenType::BranchLess => Ok(OpCode::BranchLess),
            TokenType::BranchGreaterEqual => Ok(OpCode::BranchGreaterEqual),
            TokenType::BranchGreater => Ok(OpCode::BranchGreater),
            TokenType::Exit => Ok(OpCode::Exit),
            // I/O.
            TokenType::Out => Ok(OpCode::Out),
            // Generative operations.
            TokenType::Morph => Ok(OpCode::Morph),
            TokenType::Project => Ok(OpCode::Project),
            // Cognitive operations.
            TokenType::Distill => Ok(OpCode::Distill),
            TokenType::Correlate => Ok(OpCode::Correlate),
            // Guardrails operations.
            TokenType::Audit => Ok(OpCode::Audit),
            TokenType::Similarity => Ok(OpCode::Similarity),
            // Context operations.
            TokenType::ContextClear => Ok(OpCode::ContextClear),
            TokenType::ContextSnapshot => Ok(OpCode::ContextSnapshot),
            TokenType::ContextRestore => Ok(OpCode::ContextRestore),
            TokenType::ContextPush => Ok(OpCode::ContextPush),
            TokenType::ContextPop => Ok(OpCode::ContextPop),
            TokenType::ContextDrop => Ok(OpCode::ContextDrop),
            _ => Err(format!(
                "Cannot convert token type '{:?}' into opcode.",
                token_type
            )),
        }
    }
}

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

        if !lexeme.to_lowercase().starts_with('x') {
            return Err(format!(
                "Invalid register format: '{}'. Expected xN (1-32).",
                lexeme
            ));
        }

        let num = lexeme[1..]
            .parse::<u32>()
            .map_err(|_| format!("Failed to parse register number from '{}'.", lexeme))?;

        if !(1..=32).contains(&num) {
            return Err(format!("Register number {} out of range (1-32).", num));
        }

        Ok(num)
    }

    fn string(&mut self, message: &str) -> String {
        self.consume(&TokenType::String, message);

        let lexeme = self.previous_lexeme();

        // Strip quotes.
        let inner = &lexeme[1..lexeme.len() - 1];

        inner.replace("\\n", "\n").replace("\\\"", "\"")
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
            .map(|byte| u32::from(byte).to_be_bytes())
            .collect();

        let address: u32 = match self.data_segment.len().try_into() {
            Ok(address) => address,
            Err(_) => {
                self.error_at_current(&format!(
                    "Failed to convert data segment length to u32. Data segment length exceeds {}. Found data segment length: {}.",
                    u32::MAX,
                    self.data_segment.len()
                ));
                return 0; // Return a default value on error, though the error handling above should prevent this from being used.
            }
        };

        self.data_segment.extend(words);

        address
    }

    fn upsert_unresolved_label(&mut self, key: String) -> Result<(), String> {
        let index = self.text_segment.len().saturating_sub(1);

        if let Some(label) = self.unresolved_labels.get_mut(&key) {
            label.indices.push(index);
        } else {
            let previous_token = self
                .previous
                .clone()
                .ok_or_else(|| "Missing token for unresolved label".to_string())?;

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
        self.emit_number(0); // Placeholder, will be replaced in backpatch.

        if let Err(msg) = self.upsert_unresolved_label(key) {
            self.error_at_current(&msg);
        }
    }

    fn expect_register(&mut self, message: &str) -> Option<u32> {
        match self.register(message) {
            Ok(r) => Some(r),
            Err(msg) => {
                self.error_at_current(&msg);
                None
            }
        }
    }

    fn expect_number(&mut self, message: &str) -> Option<u32> {
        match self.number(message) {
            Ok(n) => Some(n),
            Err(msg) => {
                self.error_at_current(&msg);
                None
            }
        }
    }

    fn expect_string(&mut self, message: &str) -> Option<String> {
        if let Some(tok) = &self.current {
            if tok.token_type() == &TokenType::String {
                return Some(self.string(message));
            }
        }

        self.error_at_current(message);
        None
    }

    fn emit_padding(&mut self, words: usize) {
        for _ in 0..words {
            self.emit_number(0);
        }
    }

    fn l_type(&mut self, token_type: &TokenType, op_code: OpCode) {
        self.consume(
            token_type,
            format!("Expected '{:?}' keyword.", token_type).as_str(),
        );

        let destination_register = match self.expect_register("Expected destination register.") {
            Some(register) => register,
            None => return,
        };

        self.consume(
            &TokenType::Comma,
            "Expected ',' after destination register.",
        );

        self.emit_opcode(op_code);
        self.emit_number(destination_register);

        match op_code {
            OpCode::LoadImmediate => {
                if let Some(immediate) = self.expect_number("Expected immediate after ','.") {
                    self.emit_number(immediate);
                }

                self.emit_padding(1);
            }
            OpCode::Move => {
                if let Some(source_register) =
                    self.expect_register("Expected source register after ','.")
                {
                    self.emit_number(source_register);
                }

                self.emit_padding(1);
            }
            _ => {
                if let Some(string) = self.expect_string("Expected string after ','.") {
                    let pointer = self.emit_string_bytecode(&string);
                    self.emit_number(pointer);
                }

                self.emit_padding(1);
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

    fn r_type(&mut self, token_type: &TokenType, op_code: OpCode) {
        self.consume(
            token_type,
            format!("Expected '{:?}' keyword.", token_type).as_str(),
        );

        let destination_register =
            match self.expect_register("Expected destination register after r-type keyword.") {
                Some(register) => register,
                None => return,
            };

        self.consume(
            &TokenType::Comma,
            "Expected ',' after destination register.",
        );

        let source_register_1 = match self.expect_register("Expected source register 1 after ','.")
        {
            Some(register) => register,
            None => return,
        };

        self.consume(&TokenType::Comma, "Expected ',' after source register 1.");

        let source_register_2 = match self.expect_register("Expected source register 2 after ','.")
        {
            Some(register) => register,
            None => return,
        };

        self.emit_opcode(op_code);
        self.emit_number(destination_register);
        self.emit_number(source_register_1);
        self.emit_number(source_register_2);
    }

    fn b_type(&mut self, token_type: &TokenType, op_code: OpCode) {
        self.consume(
            token_type,
            format!("Expected '{:?}' keyword.", token_type).as_str(),
        );

        let source_register_1 =
            match self.expect_register("Expected source register 1 after branch keyword.") {
                Some(register) => register,
                None => return,
            };
        self.consume(&TokenType::Comma, "Expected ',' after source register 1.");

        let source_register_2 = match self.expect_register("Expected source register 2 after ','.")
        {
            Some(register) => register,
            None => return,
        };
        self.consume(&TokenType::Comma, "Expected ',' after source register 2.");

        let label_name = self
            .identifier("Expected label name after ','.")
            .to_string();

        self.emit_opcode(op_code);
        self.emit_number(source_register_1);
        self.emit_number(source_register_2);
        self.emit_label_bytecode(label_name);
    }

    fn zero_operand(&mut self, token_type: &TokenType, op_code: OpCode) {
        self.consume(
            token_type,
            format!("Expected '{:?}' keyword.", token_type).as_str(),
        );

        self.emit_opcode(op_code);
        self.emit_padding(3);
    }

    fn single_operand(&mut self, token_type: &TokenType, op_code: OpCode) {
        self.consume(
            token_type,
            format!("Expected '{:?}' keyword.", token_type).as_str(),
        );

        if let Some(source_register) = self.expect_register("Expected source register after 'out'.")
        {
            self.emit_opcode(op_code);
            self.emit_number(source_register);
        }

        self.emit_padding(2);
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
                let token_type = current_token.token_type().clone();
                let op_code: OpCode = match token_type.clone().try_into() {
                    Ok(op_code) => op_code,
                    Err(error) => {
                        self.error_at_previous(&error);
                        return Err("Assembly failed due to errors.");
                    }
                };

                match token_type {
                    // Data movement.
                    TokenType::LoadString
                    | TokenType::LoadImmediate
                    | TokenType::LoadFile
                    | TokenType::Move => self.l_type(&token_type, op_code),
                    // Control flow.
                    TokenType::BranchEqual
                    | TokenType::BranchLess
                    | TokenType::BranchLessEqual
                    | TokenType::BranchGreater
                    | TokenType::BranchGreaterEqual => self.b_type(&token_type, op_code),
                    TokenType::Exit => self.zero_operand(&token_type, op_code),
                    TokenType::Label => self.label(),
                    // I/O.
                    TokenType::Out => self.single_operand(&token_type, op_code),
                    // Generative, cognitive, and guardrails operations.
                    TokenType::Morph
                    | TokenType::Project
                    | TokenType::Distill
                    | TokenType::Correlate
                    | TokenType::Audit
                    | TokenType::Similarity => self.r_type(&token_type, op_code),
                    // Context operations.
                    TokenType::ContextClear | TokenType::ContextDrop => {
                        self.zero_operand(&token_type, op_code)
                    }
                    TokenType::ContextSnapshot
                    | TokenType::ContextRestore
                    | TokenType::ContextPush
                    | TokenType::ContextPop => self.single_operand(&token_type, op_code),
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
