use std::collections::HashMap;

use crate::instruction::Operand;
use crate::scanner::Scanner;
use crate::token::{Token, TokenType};

pub struct Assembler {
    source: &'static str,
    scanner: Scanner,
    previous: Option<Token>,
    current: Option<Token>,
    stack_level: u32,
    stack_levels: HashMap<String, u32>,
    had_error: bool,
    panic_mode: bool,
}

impl Assembler {
    pub fn new(source: &'static str) -> Self {
        return Assembler {
            source,
            scanner: Scanner::new(source),
            previous: None,
            current: None,
            stack_level: 0,
            stack_levels: HashMap::new(),
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
        self.stack_level += 1;
    }

    // fn make_instruction(&self, op_code: OPCode) -> Instruction {
    //     return Instruction {
    //         op_code,
    //         operand_1: None,
    //         operand_2: None,
    //         operand_3: None,
    //     };
    // }

    fn number(&mut self, message: &str) -> Result<f32, &'static str> {
        self.consume(TokenType::NUMBER, message);

        if let Ok(value) = self.previous_lexeme().parse() {
            return Ok(value);
        }

        return Err("Failed to parse number.");
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
            TokenType::IDENTIFIER => match self.identifier(message) {
                Ok(value) => return Ok(Operand::Variable(value)),
                Err(e) => return Err(e),
            },
            _ => return Err("Expected number, string, or identifier as operand."),
        }
    }

    fn variable(&mut self) {
        self.advance_stack_level();
        self.consume(TokenType::VAR, "Expected 'var' keyword.");

        let variable_name = match self.identifier("Expected variable name.") {
            Ok(name) => name,
            _ => return,
        };

        self.consume(TokenType::COMMA, "Expected ',' after variable name.");

        let variable_value = match self.operand("Expected operand after ','.") {
            Ok(value) => value,
            _ => return,
        };

        println!(
            "[Stack Level {}] Variable: {} with value {:#?}",
            self.stack_level,
            variable_name,
            match variable_value {
                Operand::Number(value) => format!("number:{}", value),
                Operand::Text(value) => format!("text:{}", value),
                Operand::Variable(name) => format!("var:{}", name),
            }
        );
    }

    fn label(&mut self) {
        self.consume(TokenType::LABEL, "Expected 'label' keyword.");

        let label_name = match self.identifier("Expected label name.") {
            Ok(name) => name,
            _ => return,
        };

        self.stack_levels
            .insert(label_name.to_string(), self.stack_level);

        println!("[Stack Level {}] Label: {}", self.stack_level, label_name);
    }

    fn subtract(&mut self) {
        self.advance_stack_level();
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

        let destination = match self.identifier("Expected destination after ','.") {
            Ok(name) => name,
            _ => return,
        };

        println!(
            "[Stack Level {}] Subtract: {} - {} -> {}",
            self.stack_level,
            match operand_1 {
                Operand::Number(value) => format!("number:{}", value),
                Operand::Text(value) => format!("text:{}", value),
                Operand::Variable(name) => format!("var:{}", name),
            },
            match operand_2 {
                Operand::Number(value) => format!("number:{}", value),
                Operand::Text(value) => format!("text:{}", value),
                Operand::Variable(name) => format!("var:{}", name),
            },
            destination
        );
    }

    fn addition(&mut self) {
        self.advance_stack_level();
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

        let destination = match self.identifier("Expected destination after ','.") {
            Ok(name) => name,
            _ => return,
        };

        println!(
            "[Stack Level {}] Add: {} + {} -> {}",
            self.stack_level,
            match operand_1 {
                Operand::Number(value) => format!("number:{}", value),
                Operand::Text(value) => format!("text:{}", value),
                Operand::Variable(name) => format!("var:{}", name),
            },
            match operand_2 {
                Operand::Number(value) => format!("number:{}", value),
                Operand::Text(value) => format!("text:{}", value),
                Operand::Variable(name) => format!("var:{}", name),
            },
            destination
        );
    }

    fn similarity(&mut self) {
        self.advance_stack_level();
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

        let destination = match self.identifier("Expected destination after ','.") {
            Ok(name) => name,
            _ => return,
        };

        println!(
            "[Stack Level {}] Similarity: {} ~ {} -> {}",
            self.stack_level,
            match operand_1 {
                Operand::Number(value) => format!("number:{}", value),
                Operand::Text(value) => format!("text:{}", value),
                Operand::Variable(name) => format!("var:{}", name),
            },
            match operand_2 {
                Operand::Number(value) => format!("number:{}", value),
                Operand::Text(value) => format!("text:{}", value),
                Operand::Variable(name) => format!("var:{}", name),
            },
            destination
        );
    }

    fn jump_less_than(&mut self) {
        self.advance_stack_level();
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

        let stack_level = match self.stack_levels.get(&label) {
            Some(level) => *level,
            None => {
                self.error_at_previous("Undefined label.");
                return;
            }
        };

        println!(
            "[Stack Level {}] Jump if Less Than: {} < {} -> {}",
            self.stack_level,
            match operand_1 {
                Operand::Number(value) => format!("number:{}", value),
                Operand::Text(value) => format!("text:{}", value),
                Operand::Variable(name) => format!("var:{}", name),
            },
            match operand_2 {
                Operand::Number(value) => format!("number:{}", value),
                Operand::Text(value) => format!("text:{}", value),
                Operand::Variable(name) => format!("var:{}", name),
            },
            label
        );
    }

    pub fn assemble(&mut self) -> bool {
        self.advance();

        loop {
            if let Some(current_token) = &self.current {
                match current_token.token_type {
                    TokenType::VAR => self.variable(),
                    TokenType::LABEL => self.label(),
                    TokenType::SUB => self.subtract(),
                    TokenType::ADD => self.addition(),
                    TokenType::SIM => self.similarity(),
                    TokenType::JLT => self.jump_less_than(),
                    TokenType::EOF => return !self.had_error,
                    _ => {
                        self.error_at_current("Unexpected keyword.");

                        return !self.had_error;
                    }
                }
            } else {
                panic!("Failed to assemble. Current token is None.")
            }
        }
    }
}
