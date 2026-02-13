use crate::assembler::scanner::token::{Token, TokenType};

pub mod token;

pub struct Scanner {
    source: &'static str,
    start: usize,
    current: usize,
    line: usize,
    column: usize,
}

impl Scanner {
    pub fn new(source: &'static str) -> Self {
        Scanner {
            source,
            current: 0,
            start: 0,
            line: 1,
            column: 0,
        }
    }

    fn is_alpha(char: char) -> bool {
        return (char >= 'a' && char <= 'z')
            || (char >= 'A' && char <= 'Z')
            || char == '_'
            || char == ':';
    }

    fn is_digit(char: char) -> bool {
        return char >= '0' && char <= '9';
    }

    fn is_at_end(&self) -> bool {
        return self.current >= self.source.chars().count();
    }

    fn advance(&mut self) -> char {
        self.current += 1;
        self.column += 1;

        return self.source.chars().nth(self.current - 1).expect(
            format!(
                "Tried to advance past end of source. Source length: {}, current: {}",
                self.source.chars().count(),
                self.current - 1
            )
            .as_str(),
        );
    }

    fn peek(&self) -> char {
        return self.source.chars().nth(self.current).expect(
            format!(
                "Tried to peek past end of source. Source length: {}, current: {}",
                self.source.chars().count(),
                self.current
            )
            .as_str(),
        );
    }

    fn peek_next(&self) -> char {
        return self.source.chars().nth(self.current + 1).expect(
            format!(
                "Tried to peek next past end of source. Source length: {}, current: {}",
                self.source.chars().count(),
                self.current + 1
            )
            .as_str(),
        );
    }

    fn make_token(&self, token_type: TokenType) -> Token {
        return Token::new(
            token_type,
            self.start,
            self.current,
            self.line,
            self.column,
            None,
        );
    }

    fn make_error(&self, message: &'static str) -> Token {
        return Token::new(
            TokenType::ERROR,
            self.start,
            self.current,
            self.line,
            self.column,
            Some(message),
        );
    }

    fn skip_whitespace(&mut self) {
        while !self.is_at_end() {
            match self.peek() {
                ' ' | '\r' | '\t' => {
                    self.advance();
                }
                '\n' => {
                    self.line += 1;
                    self.column = 0;

                    self.advance();
                }
                ';' => {
                    while !self.is_at_end() && self.peek() != '\n' {
                        self.advance();
                    }
                }
                _ => return,
            }
        }
    }

    fn label(&mut self) -> Token {
        let token = self.make_token(TokenType::LABEL);

        // Consume the ':'.
        self.advance();

        return token;
    }

    fn identifier(&mut self) -> Token {
        while !self.is_at_end()
            && let char = self.peek()
            && (Self::is_alpha(char) || Self::is_digit(char))
        {
            self.advance();
        }

        let identifier = &self.source[self.start..self.current];

        if identifier.ends_with(':') {
            return self.label();
        }

        return match identifier.to_lowercase().as_str() {
            "li" => self.make_token(TokenType::LI),
            "lf" => self.make_token(TokenType::LF),
            "mv" => self.make_token(TokenType::MV),
            "add" => self.make_token(TokenType::ADD),
            "sub" => self.make_token(TokenType::SUB),
            "mul" => self.make_token(TokenType::MUL),
            "div" => self.make_token(TokenType::DIV),
            "sim" => self.make_token(TokenType::SIM),
            "beq" => self.make_token(TokenType::BEQ),
            "ble" => self.make_token(TokenType::BLE),
            "blt" => self.make_token(TokenType::BLT),
            "bge" => self.make_token(TokenType::BGE),
            "bgt" => self.make_token(TokenType::BGT),
            "out" => self.make_token(TokenType::OUT),
            _ => self.make_token(TokenType::IDENTIFIER),
        };
    }

    fn number(&mut self) -> Token {
        while !self.is_at_end()
            && let char = self.peek()
            && Self::is_digit(char)
        {
            self.advance();
        }

        // Look for a fractional part.
        if self.peek() == '.'
            && let next_char = self.peek_next()
            && Self::is_digit(next_char)
        {
            // Consume the decimal point.
            self.advance();

            while !self.is_at_end()
                && let char = self.peek()
                && Self::is_digit(char)
            {
                self.advance();
            }
        }

        return self.make_token(TokenType::NUMBER);
    }

    fn string(&mut self) -> Token {
        while !self.is_at_end() && self.peek() != '"' {
            if self.peek() == '\n' {
                self.line += 1;
                self.column = 0;
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

        if Self::is_alpha(char) {
            return self.identifier();
        }

        if Self::is_digit(char) {
            return self.number();
        }

        return match char {
            // Single-character tokens.
            ',' => self.make_token(TokenType::COMMA),
            '"' => return self.string(),
            _ => return self.make_error("Unexpected character"),
        };
    }
}
