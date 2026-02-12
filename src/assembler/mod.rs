use std::collections::HashMap;
use std::hash::{DefaultHasher, Hash, Hasher};

use crate::assembler::opcode::OpCode;
use crate::assembler::operand::{Operand, OperandType};
use crate::assembler::scanner::Scanner;
use crate::assembler::scanner::token::{Token, TokenType};

pub mod opcode;
pub mod operand;
mod scanner;

struct UnitialisedLabel {
    current_byte_code_indices: Vec<usize>,
    token: Token,
}

pub struct Assembler {
    byte_code: Vec<[u8; 4]>,

    source: &'static str,
    scanner: Scanner,

    previous: Option<Token>,
    current: Option<Token>,

    current_byte_code_index: usize,
    byte_code_indices: HashMap<u64, usize>,
    uninitialised_labels: HashMap<u64, UnitialisedLabel>,

    had_error: bool,
    panic_mode: bool,
}

impl Assembler {
    pub fn new(source: &'static str) -> Self {
        return Assembler {
            byte_code: Vec::new(),
            source,
            scanner: Scanner::new(source),
            previous: None,
            current: None,
            current_byte_code_index: 0,
            byte_code_indices: HashMap::new(),
            uninitialised_labels: HashMap::new(),
            had_error: false,
            panic_mode: false,
        };
    }

    fn lexeme(&self, token: &Token) -> String {
        return self
            .source
            .chars()
            .skip(token.start())
            .take(token.end() - token.start())
            .collect::<String>();
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
        let token = match &self.current {
            Some(token) => token.clone(),
            None => panic!(
                "Failed to handle error at current token.\nError: {}",
                message
            ),
        };

        self.error_at(&token, message);
    }

    fn advance(&mut self) {
        self.previous = self.current.clone();

        loop {
            let current_token = self.scanner.scan_token();

            self.current = Some(current_token.clone());

            if current_token.token_type() != &TokenType::ERROR {
                return;
            }

            self.error_at_current("");
        }
    }

    fn previous_lexeme(&self) -> String {
        if let Some(token) = &self.previous {
            return self.lexeme(&token);
        }

        panic!("Failed to get token lexeme. Previous token is None.");
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

    fn advance_stack_level(&mut self) {
        self.current_byte_code_index = self.byte_code.len() - 1;
    }

    fn number(&mut self, message: &str) -> u32 {
        self.consume(&TokenType::NUMBER, message);

        return match self.previous_lexeme().parse() {
            Ok(value) => value,
            _ => panic!(
                "{}",
                format!(
                    "Failed to parse number from lexeme '{}'.",
                    self.previous_lexeme()
                )
            ),
        };
    }

    fn register(&mut self, message: &str) -> u32 {
        self.consume(&TokenType::IDENTIFIER, message);

        let lexeme = self.previous_lexeme();
        let register = lexeme[1..].to_string(); // Remove the 'r' prefix.

        return match register.parse() {
            Ok(value) => value,
            _ => panic!(
                "{}",
                format!("Failed to parse register from lexeme '{}'.", lexeme)
            ),
        };
    }

    fn string(&mut self, message: &str) -> String {
        self.consume(&TokenType::STRING, message);

        let lexeme = self.previous_lexeme();

        // Remove surrounding quotes.
        return lexeme
            .chars()
            .skip(1)
            .take(lexeme.chars().count() - 2)
            .collect::<String>();
    }

    fn identifier(&mut self, message: &str) -> String {
        self.consume(&TokenType::IDENTIFIER, message);

        return self.previous_lexeme();
    }

    fn operand(&mut self, message: &str) -> Operand {
        let current_type = match self.current {
            Some(ref token) => token.token_type(),
            None => panic!("Failed to parse operand. Current token is None."),
        };

        return match current_type {
            TokenType::NUMBER => Operand::Number(self.number(message)),
            TokenType::STRING => Operand::Text(self.string(message).to_string()),
            TokenType::IDENTIFIER => Operand::Register(self.register(message)),
            _ => panic!("Expected number, string, or register as operand."),
        };
    }

    fn emit_number_bytecode(&mut self, value: u32) {
        self.byte_code.push(value.to_be_bytes());
    }

    fn emit_op_code_bytecode(&mut self, op_code: OpCode) {
        self.byte_code.push(OpCode::to_be_bytes(&op_code));
    }

    fn emit_register_bytecode(&mut self, register: u32) {
        self.byte_code.push(register.to_be_bytes());
    }

    fn emit_operand_bytecode(&mut self, operand: &Operand) {
        match operand {
            Operand::Number(value) => {
                self.byte_code
                    .push(OperandType::to_be_bytes(&OperandType::NUMBER));

                self.byte_code.push(1u32.to_be_bytes());
                self.byte_code.push(value.to_be_bytes());
            }
            Operand::Text(value) => {
                let value_be_bytes = value
                    .bytes()
                    .map(|byte| u32::from(byte).to_be_bytes())
                    .collect::<Vec<[u8; 4]>>();
                let value_be_bytes_length: u32 = match value_be_bytes.len().try_into() {
                    Ok(length) => length,
                    _ => panic!(
                        "Failed to get byte length of text operand. Byte length exceeds {}. Found byte length: {}.",
                        u32::MAX,
                        value_be_bytes.len()
                    ),
                };

                self.byte_code
                    .push(OperandType::to_be_bytes(&OperandType::TEXT));
                self.byte_code
                    .push((value_be_bytes_length).to_be_bytes()); // Length in 4-byte characters.
                self.byte_code.extend(value_be_bytes);
            }
            Operand::Register(value) => {
                self.byte_code
                    .push(OperandType::to_be_bytes(&OperandType::REGISTER));
                self.byte_code.push(1u32.to_be_bytes());
                self.byte_code.push(value.to_be_bytes());
            }
        }
    }

    fn load(&mut self) {
        self.consume(&TokenType::LOAD, "Expected 'load' keyword.");

        let register = self.register("Expected register name.");

        self.consume(&TokenType::COMMA, "Expected ',' after register name.");

        let file_path = self
            .string("Expected file path string after ','.")
            .to_string();

        self.emit_op_code_bytecode(OpCode::LOAD);
        self.emit_register_bytecode(register);
        self.emit_operand_bytecode(&Operand::Text(file_path));

        self.advance_stack_level();
    }

    fn _move(&mut self) {
        self.consume(&TokenType::MOV, "Expected 'mov' keyword.");

        let register = self.register("Expected register name.");

        self.consume(&TokenType::COMMA, "Expected ',' after register name.");

        let variable_value = self.operand("Expected operand after ','.");

        self.emit_op_code_bytecode(OpCode::MOV);
        self.emit_register_bytecode(register);
        self.emit_operand_bytecode(&variable_value);

        self.advance_stack_level();
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

        // Backpatch any uninitialised labels.
        if let Some(uninitialised_labels) = self.uninitialised_labels.remove(&key) {
            for current_byte_code_index in uninitialised_labels.current_byte_code_indices {
                let value: u32 = match jump_destination_byte_code_index.try_into() {
                    Ok(value) => value,
                    _ => panic!(
                        "Failed to convert bytecode index to u32 for backpatching. Bytecode index exceeds {}. Found bytecode index: {}.",
                        u32::MAX,
                        jump_destination_byte_code_index
                    ),
                };

                println!("Backpatching label '{}' at bytecode index {} with value {}.", value, current_byte_code_index, value);

                self.byte_code[current_byte_code_index] = value.to_be_bytes();
            }
        }

        self.byte_code_indices
            .insert(key, jump_destination_byte_code_index);
    }

    fn subtract(&mut self) {
        self.consume(&TokenType::SUB, "Expected 'sub' keyword.");

        let operand_1 = self.operand("Expected first operand after 'sub'.");

        self.consume(&TokenType::COMMA, "Expected ',' after operand.");

        let operand_2 = self.operand("Expected second operand after ','.");

        self.consume(&TokenType::COMMA, "Expected ',' after second operand.");

        let destination = self.register("Expected destination register after ','.");

        self.emit_op_code_bytecode(OpCode::SUB);
        self.emit_operand_bytecode(&operand_1);
        self.emit_operand_bytecode(&operand_2);
        self.emit_register_bytecode(destination);

        self.advance_stack_level();
    }

    fn addition(&mut self) {
        self.consume(&TokenType::ADD, "Expected 'add' keyword.");

        let operand_1 = self.operand("Expected first operand after 'add'.");

        self.consume(&TokenType::COMMA, "Expected ',' after operand.");

        let operand_2 = self.operand("Expected second operand after ','.");

        self.consume(&TokenType::COMMA, "Expected ',' after second operand.");

        let destination = self.register("Expected destination register after ','.");

        self.emit_op_code_bytecode(OpCode::ADD);
        self.emit_operand_bytecode(&operand_1);
        self.emit_operand_bytecode(&operand_2);
        self.emit_register_bytecode(destination);

        self.advance_stack_level();
    }

    fn multiply(&mut self) {
        self.consume(&TokenType::MUL, "Expected 'mul' keyword.");

        let operand_1 = self.operand("Expected first operand after 'mul'.");

        self.consume(&TokenType::COMMA, "Expected ',' after operand.");

        let operand_2 = self.operand("Expected second operand after ','.");

        self.consume(&TokenType::COMMA, "Expected ',' after second operand.");

        let destination = self.register("Expected destination register after ','.");

        self.emit_op_code_bytecode(OpCode::MUL);
        self.emit_operand_bytecode(&operand_1);
        self.emit_operand_bytecode(&operand_2);
        self.emit_register_bytecode(destination);

        self.advance_stack_level();
    }

    fn divide(&mut self) {
        self.consume(&TokenType::DIV, "Expected 'div' keyword.");

        let operand_1 = self.operand("Expected first operand after 'div'.");

        self.consume(&TokenType::COMMA, "Expected ',' after operand.");

        let operand_2 = self.operand("Expected second operand after ','.");

        self.consume(&TokenType::COMMA, "Expected ',' after second operand.");

        let destination = self.register("Expected destination register after ','.");

        self.emit_op_code_bytecode(OpCode::DIV);
        self.emit_operand_bytecode(&operand_1);
        self.emit_operand_bytecode(&operand_2);
        self.emit_register_bytecode(destination);

        self.advance_stack_level();
    }

    fn similarity(&mut self) {
        self.consume(&TokenType::SIM, "Expected 'sim' keyword.");

        let operand_1 = self.operand("Expected first operand after 'sim'.");

        self.consume(&TokenType::COMMA, "Expected ',' after operand.");

        let operand_2 = self.operand("Expected second operand after ','.");

        self.consume(&TokenType::COMMA, "Expected ',' after second operand.");

        let destination = self.register("Expected destination register after ','.");

        self.emit_op_code_bytecode(OpCode::SIM);
        self.emit_operand_bytecode(&operand_1);
        self.emit_operand_bytecode(&operand_2);
        self.emit_register_bytecode(destination);

        self.advance_stack_level();
    }

    fn upsert_uninitialised_label(&mut self, key: u64) {
        let bytecode_index = self.byte_code.len() - 1;

        if let Some(uninitialised_label) = self.uninitialised_labels.get_mut(&key) {
            uninitialised_label
                .current_byte_code_indices
                .push(bytecode_index);
        } else {
            let previous_token = match self.previous.clone() {
                Some(token) => token,
                None => panic!("Failed to get current token for uninitialised label."),
            };

            self.uninitialised_labels.insert(
                key,
                UnitialisedLabel {
                    current_byte_code_indices: vec![bytecode_index],
                    token: previous_token,
                },
            );
        }
    }

    fn jump_compare(&mut self, token_type: &TokenType) {
        self.consume(
            token_type,
            format!("Expected '{:?}' keyword.", token_type).as_str(),
        );

        let operand_1 =
            self.operand(format!("Expected first operand after '{:?}'.", token_type).as_str());

        self.consume(&TokenType::COMMA, "Expected ',' after operand.");

        let operand_2 = self.operand("Expected second operand after ','.");

        self.consume(&TokenType::COMMA, "Expected ',' after second operand.");

        let label = self.identifier("Expected label name after ','.");
        let key = Self::hash(&label);

        let opcode = match token_type {
            TokenType::JEQ => OpCode::JEQ,
            TokenType::JLT => OpCode::JLT,
            TokenType::JLE => OpCode::JLE,
            TokenType::JGT => OpCode::JGT,
            TokenType::JGE => OpCode::JGE,
            _ => panic!("Unexpected token type for jump compare."),
        };

        self.emit_op_code_bytecode(opcode);
        self.emit_operand_bytecode(&operand_1);
        self.emit_operand_bytecode(&operand_2);

        if let Some(index) = self.byte_code_indices.get(&key) {
            let value: u32 = match index.clone().try_into() {
                Ok(value) => value,
                _ => panic!(
                    "Failed to convert bytecode index to u32 for jump compare. Bytecode index exceeds {}. Found bytecode index: {}.",
                    u32::MAX,
                    index
                ),
            };
            self.emit_number_bytecode(value);
        } else {
            // Placeholder for backpatching.
            self.emit_number_bytecode(0);
            // Record the current bytecode index for backpatching later.
            self.upsert_uninitialised_label(key);
        }

        self.advance_stack_level();
    }

    fn output(&mut self) {
        self.consume(&TokenType::OUT, "Expected 'out' keyword.");

        let operand = self.operand("Expected operand after 'out'.");

        self.emit_op_code_bytecode(OpCode::OUT);
        self.emit_operand_bytecode(&operand);

        self.advance_stack_level();
    }

    pub fn assemble(&mut self) -> Result<Vec<u8>, &'static str> {
        self.advance();

        while !self.panic_mode {
            if let Some(current_token) = &self.current {
                match current_token.token_type() {
                    TokenType::MOV => self._move(),
                    TokenType::LABEL => self.label(),
                    TokenType::SUB => self.subtract(),
                    TokenType::ADD => self.addition(),
                    TokenType::MUL => self.multiply(),
                    TokenType::DIV => self.divide(),
                    TokenType::SIM => self.similarity(),
                    TokenType::JEQ => self.jump_compare(&TokenType::JEQ),
                    TokenType::JLT => self.jump_compare(&TokenType::JLT),
                    TokenType::JLE => self.jump_compare(&TokenType::JLE),
                    TokenType::JGT => self.jump_compare(&TokenType::JGT),
                    TokenType::JGE => self.jump_compare(&TokenType::JGE),
                    TokenType::OUT => self.output(),
                    TokenType::LOAD => self.load(),
                    TokenType::EOF => break,
                    _ => self.error_at_current("Unexpected keyword."),
                }
            } else {
                panic!("Failed to assemble. Current token is None.")
            }
        }

        if self.had_error {
            return Err("Assembly failed due to errors.");
        }

        if let Some((_, uninitialised_label)) = self.uninitialised_labels.iter().nth(0) {
            let token = uninitialised_label.token.clone();

            self.error_at(&token, "Undefined label referenced here.");

            return Err("Assembly failed due to errors.");
        }

        return Ok(self
            .byte_code
            .iter()
            .flat_map(|bytes| bytes.iter())
            .cloned()
            .collect());
    }
}
