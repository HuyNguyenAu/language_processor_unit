use std::collections::HashMap;

use crate::instruction::OpCode;
use crate::instruction::Operand;
use crate::instruction::OperandType;
use crate::scanner::Scanner;
use crate::token::{Token, TokenType};

pub struct Assembler {
    bytecode: Vec<u8>,

    source: &'static str,
    scanner: Scanner,

    previous: Option<Token>,
    current: Option<Token>,

    current_bytecode_index: usize,
    bytecode_indices: HashMap<String, usize>,

    had_error: bool,
    panic_mode: bool,
}

impl Assembler {
    pub fn new(source: &'static str) -> Self {
        return Assembler {
            bytecode: Vec::new(),
            source,
            scanner: Scanner::new(source),
            previous: None,
            current: None,
            current_bytecode_index: 0,
            bytecode_indices: HashMap::new(),
            had_error: false,
            panic_mode: false,
        };
    }

    fn error_at(&mut self, token: &Token, message: &str) {
        if self.panic_mode {
            return;
        }

        self.panic_mode = true;

        eprint!("[Line {}:{}] Error:", token.line, token.column);

        if token.token_type == TokenType::ERROR
            && let Some(error) = &token.error
        {
            eprint!(" {}", error);
        }

        eprintln!(
            " at '{}'.",
            &self.source[token.start..token.start + token.length]
        );

        eprintln!("{}", message);

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

    fn error_at_previous(&mut self, message: &str) {
        let token = match self.previous.clone() {
            Some(token) => token,
            None => panic!(
                "Failed to handle error at previous token.\nError: {}",
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

            if current_token.token_type != TokenType::ERROR {
                return;
            }

            self.error_at_current("");
        }
    }

    fn previous_lexeme(&self) -> &str {
        if let Some(token) = &self.previous {
            return &self.source[token.start..token.start + token.length];
        }

        panic!("Failed to get token lexeme. Previous token is None.");
    }

    fn consume(&mut self, token_type: TokenType, message: &str) {
        if let Some(current_token) = self.current.clone()
            && current_token.token_type == token_type
        {
            self.advance();

            return;
        }

        self.error_at_current(message);
    }

    fn advance_stack_level(&mut self) {
        self.current_bytecode_index = self.bytecode.len() - 1;
    }

    fn number(&mut self, message: &str) -> Result<u8, &'static str> {
        self.consume(TokenType::NUMBER, message);

        return match self.previous_lexeme().parse() {
            Ok(value) => Ok(value),
            Err(_) => Err("Failed to parse number."),
        };
    }

    fn register(&mut self, message: &str) -> Result<u8, &'static str> {
        self.consume(TokenType::IDENTIFIER, message);

        return match self
            .previous_lexeme()
            .chars()
            .skip(1)
            .collect::<String>()
            .parse()
        {
            Ok(value) => Ok(value),
            Err(_) => Err("Failed to parse register."),
        };
    }

    fn string(&mut self, message: &str) -> Result<String, &'static str> {
        self.consume(TokenType::STRING, message);

        // Remove surrounding quotes.
        let mut value = self.previous_lexeme().chars();
        value.next();
        value.next_back();

        return Ok(value.collect());
    }

    fn identifier(&mut self, message: &str) -> Result<String, &'static str> {
        self.consume(TokenType::IDENTIFIER, message);

        let value = self.previous_lexeme().to_string();

        return Ok(value);
    }

    fn operand(&mut self, message: &str) -> Result<Operand, &'static str> {
        let current_type = match self.current {
            Some(ref token) => &token.token_type,
            None => return Err("Failed to parse operand. Current token is None."),
        };

        match current_type {
            TokenType::NUMBER => match self.number(message) {
                Ok(value) => return Ok(Operand::Number(value)),
                Err(e) => return Err(e),
            },
            TokenType::STRING => match self.string(message) {
                Ok(value) => return Ok(Operand::Text(value)),
                Err(e) => return Err(e),
            },
            TokenType::IDENTIFIER => match self.register(message) {
                Ok(value) => return Ok(Operand::Register(value)),
                Err(e) => return Err(e),
            },
            _ => return Err("Expected number, string, or register as operand."),
        }
    }

    fn emit_number_bytecode(&mut self, value: u8) {
        self.bytecode.push(value);
    }

    fn emit_op_code_bytecode(&mut self, op_code: OpCode) {
        self.bytecode.push(op_code as u8);
    }

    fn emit_register_bytecode(&mut self, register: u8) {
        self.bytecode.push(register);
    }

    fn emit_operand_bytecode(&mut self, operand: &Operand) {
        match operand {
            Operand::Number(value) => {
                self.bytecode.push(OperandType::NUMBER as u8);
                self.bytecode.push(1);
                self.bytecode.push(*value);
            }
            Operand::Text(value) => {
                self.bytecode.push(OperandType::TEXT as u8);

                let bytes = value.as_bytes();

                self.bytecode.push(bytes.len() as u8);
                self.bytecode.extend(bytes);
            }
            Operand::Register(value) => {
                self.bytecode.push(OperandType::REGISTER as u8);
                self.bytecode.push(1);
                self.bytecode.push(*value);
            }
        }
    }

    fn _move(&mut self) {
        self.consume(TokenType::MOV, "Expected 'mov' keyword.");

        let register = match self.register("Expected register name.") {
            Ok(name) => name,
            _ => return,
        };

        self.consume(TokenType::COMMA, "Expected ',' after register name.");

        let variable_value = match self.operand("Expected operand after ','.") {
            Ok(value) => value,
            _ => return,
        };

        self.emit_op_code_bytecode(OpCode::MOV);
        self.emit_register_bytecode(register);
        self.emit_operand_bytecode(&variable_value);

        self.advance_stack_level();
    }

    fn label(&mut self) {
        self.consume(TokenType::LABEL, "Expected label name.");

        let label_name = self.previous_lexeme().to_string();
        let value = label_name.trim_end_matches(':');

        self.bytecode_indices
            .insert(value.to_string(), self.current_bytecode_index + 1);
    }

    fn subtract(&mut self) {
        self.consume(TokenType::SUB, "Expected 'sub' keyword.");

        let operand_1 = match self.operand("Expected first operand after 'sub'.") {
            Ok(op) => op,
            _ => return,
        };

        self.consume(TokenType::COMMA, "Expected ',' after operand.");

        let operand_2 = match self.operand("Expected second operand after ','.") {
            Ok(op) => op,
            _ => return,
        };

        self.consume(TokenType::COMMA, "Expected ',' after second operand.");

        let destination = match self.register("Expected destination register after ','.") {
            Ok(name) => name,
            _ => return,
        };

        self.emit_op_code_bytecode(OpCode::SUB);
        self.emit_operand_bytecode(&operand_1);
        self.emit_operand_bytecode(&operand_2);
        self.emit_register_bytecode(destination);

        self.advance_stack_level();
    }

    fn addition(&mut self) {
        self.consume(TokenType::ADD, "Expected 'add' keyword.");

        let operand_1 = match self.operand("Expected first operand after 'add'.") {
            Ok(op) => op,
            _ => return,
        };

        self.consume(TokenType::COMMA, "Expected ',' after operand.");

        let operand_2 = match self.operand("Expected second operand after ','.") {
            Ok(op) => op,
            _ => return,
        };

        self.consume(TokenType::COMMA, "Expected ',' after second operand.");

        let destination = match self.register("Expected destination register after ','.") {
            Ok(name) => name,
            _ => return,
        };

        self.emit_op_code_bytecode(OpCode::ADD);
        self.emit_operand_bytecode(&operand_1);
        self.emit_operand_bytecode(&operand_2);
        self.emit_register_bytecode(destination);

        self.advance_stack_level();
    }

    fn similarity(&mut self) {
        self.consume(TokenType::SIM, "Expected 'sim' keyword.");

        let operand_1 = match self.operand("Expected first operand after 'sim'.") {
            Ok(op) => op,
            _ => return,
        };

        self.consume(TokenType::COMMA, "Expected ',' after operand.");

        let operand_2 = match self.operand("Expected second operand after ','.") {
            Ok(op) => op,
            _ => return,
        };

        self.consume(TokenType::COMMA, "Expected ',' after second operand.");

        let destination = match self.register("Expected destination register after ','.") {
            Ok(name) => name,
            _ => return,
        };

        self.emit_op_code_bytecode(OpCode::SIM);
        self.emit_operand_bytecode(&operand_1);
        self.emit_operand_bytecode(&operand_2);
        self.emit_register_bytecode(destination);

        self.advance_stack_level();
    }

    fn jump_less_than(&mut self) {
        self.consume(TokenType::JLT, "Expected 'jlt' keyword.");

        let operand_1 = match self.operand("Expected first operand after 'jlt'.") {
            Ok(op) => op,
            _ => return,
        };

        self.consume(TokenType::COMMA, "Expected ',' after operand.");

        let operand_2 = match self.operand("Expected second operand after ','.") {
            Ok(op) => op,
            _ => return,
        };

        self.consume(TokenType::COMMA, "Expected ',' after second operand.");

        let label = match self.identifier("Expected label name after ','.") {
            Ok(name) => name,
            _ => return,
        };

        let current_bytecode_index = match self.bytecode_indices.get(&label) {
            Some(level) => *level,
            None => {
                self.error_at_previous("Undefined label.");
                return;
            }
        };

        self.emit_op_code_bytecode(OpCode::JLT);
        self.emit_operand_bytecode(&operand_1);
        self.emit_operand_bytecode(&operand_2);
        self.emit_number_bytecode(current_bytecode_index as u8);

        self.advance_stack_level();
    }

    pub fn assemble(&mut self) -> Option<Vec<u8>> {
        self.advance();

        while !self.panic_mode {
            if let Some(current_token) = &self.current {
                match current_token.token_type {
                    TokenType::MOV => self._move(),
                    TokenType::LABEL => self.label(),
                    TokenType::SUB => self.subtract(),
                    TokenType::ADD => self.addition(),
                    TokenType::SIM => self.similarity(),
                    TokenType::JLT => self.jump_less_than(),
                    TokenType::EOF => break,
                    _ => self.error_at_current("Unexpected keyword."),
                }
            } else {
                panic!("Failed to assemble. Current token is None.")
            }
        }

        if self.had_error {
            return None;
        }

        return Some(self.bytecode.clone());
    }
}
