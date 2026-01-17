use crate::token::{Token, TokenType};

pub struct Scanner {
    source: &'static str,
    start: usize,
    current: usize,
    line: usize,
}

impl Scanner {
    pub fn new(source: &'static str) -> Self {
        Scanner {
            source,
            current: 0,
            start: 0,
            line: 1,
        }
    }

    fn is_alpha(&self, char: char) -> bool {
        return (char >= 'a' && char <= 'z') || (char >= 'A' && char <= 'Z') || char == '_';
    }

    fn is_digit(&self, char: char) -> bool {
        return char >= '0' && char <= '9';
    }

    fn is_at_end(&self) -> bool {
        return self.source.chars().nth(self.current).is_none();
    }

    fn advance(&mut self) -> char {
        self.current += 1;

        return self.source.chars().nth(self.current - 1).expect(
            format!(
                "Tried to advance past end of source. Source length: {}, current: {}",
                self.source.len(),
                self.current - 1
            )
            .as_str(),
        );
    }

    fn peek(&self) -> char {
        return self.source.chars().nth(self.current).expect(
            format!(
                "Tried to peek past end of source. Source length: {}, current: {}",
                self.source.len(),
                self.current
            )
            .as_str(),
        );
    }

    fn peek_next(&self) -> char {
        return self.source.chars().nth(self.current + 1).expect(
            format!(
                "Tried to peek next past end of source. Source length: {}, current: {}",
                self.source.len(),
                self.current + 1
            )
            .as_str(),
        );
    }

    fn make_token(&self, token_type: TokenType) -> Token {
        return Token {
            token_type,
            start: self.start,
            length: self.current - self.start,
            line: self.line,
            error: None,
        };
    }

    fn make_error(&self, message: &'static str) -> Token {
        return Token {
            token_type: TokenType::ERROR,
            start: self.start,
            length: self.current - self.start,
            line: self.line,
            error: Some(message),
        };
    }

    fn skip_whitespace(&mut self) {
        while !self.is_at_end() {
            match self.peek() {
                ' ' | '\r' | '\t' => {
                    self.advance();

                    return;
                }
                '\n' => {
                    self.line += 1;
                    self.advance();

                    return;
                }
                '/' => {
                    if self.peek_next() == '/' {
                        while self.peek() != '\n' && !self.is_at_end() {
                            self.advance();
                        }
                    } else {
                        return;
                    }
                }
                _ => return,
            }
        }
    }

    fn identifier(&mut self) -> Token {
        while let char = self.peek()
            && (self.is_alpha(char) || self.is_digit(char))
        {
            self.advance();
        }

        let identifier = &self.source[self.start..self.current];

        return match identifier.to_lowercase().as_str() {
            "var" => self.make_token(TokenType::VAR),
            "add" => self.make_token(TokenType::ADD),
            "sub" => self.make_token(TokenType::SUB),
            "sim" => self.make_token(TokenType::SIM),
            "bge" => self.make_token(TokenType::BGE),
            "label" => self.make_token(TokenType::LABEL),
            "jmp" => self.make_token(TokenType::JMP),
            "stop" => self.make_token(TokenType::STOP),
            _ => self.make_token(TokenType::IDENTIFIER),
        };
    }

    fn number(&mut self) -> Token {
        while let char = self.peek()
            && self.is_digit(char)
        {
            self.advance();
        }

        // Look for a fractional part.
        if self.peek() == '.'
            && let next_char = self.peek_next()
            && self.is_digit(next_char)
        {
            // Consume the decimal point.
            self.advance();

            while let char = self.peek()
                && self.is_digit(char)
            {
                self.advance();
            }
        }

        return self.make_token(TokenType::NUMBER);
    }

    fn string(&mut self) -> Token {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }

            self.advance();
        }

        if self.is_at_end() {
            return self.make_error("Unterminated string.");
        }

        // Consume the closing quote.
        self.advance();

        return self.make_token(TokenType::STRING);
    }

    pub fn scan_token(&mut self) -> Token {
        self.skip_whitespace();

        self.start = self.current;

        if self.is_at_end() {
            return self.make_token(TokenType::EOF);
        }

        let char = self.advance();

        if self.is_alpha(char) {
            return self.identifier();
        }

        if self.is_digit(char) {
            return self.number();
        }

        return match char {
            // Single-character tokens.
            ',' => self.make_token(TokenType::COMMA),
            // ';' => self.make_token(TokenType::SEMICOLON),
            '-' => self.make_token(TokenType::MINUS),
            '+' => self.make_token(TokenType::PLUS),
            // '*' => self.make_token(TokenType::STAR),
            // '/' => self.make_token(TokenType::SLASH),
            '"' => return self.string(),
            _ => return self.make_error("Unexpected character"),
        };
    }
}
