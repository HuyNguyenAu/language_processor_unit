#[derive(Debug, PartialEq, Clone)]
pub enum TokenType {
    // Single-character.
    COMMA,

    // Literals.
    IDENTIFIER,
    STRING,
    NUMBER,

    // Keywords.
    MOV,
    ADD,
    SUB,
    SIM,
    LABEL,
    JEQ,
    JLT,
    JLE,
    JGT,
    JGE,
    OUT,

    // Misc.
    EOF,
    ERROR,
}

#[derive(Clone, Debug)]
pub struct Token {
    token_type: TokenType,
    start: usize,
    length: usize,
    line: usize,
    column: usize,
    error: Option<&'static str>,
}

impl Token {
    pub fn new(
        token_type: TokenType,
        start: usize,
        length: usize,
        line: usize,
        column: usize,
        error: Option<&'static str>,
    ) -> Token {
        Token {
            token_type,
            start,
            length,
            line,
            column,
            error,
        }
    }

    pub fn token_type(&self) -> &TokenType {
        return &self.token_type;
    }

    pub fn start(&self) -> usize {
        return self.start;
    }

    pub fn length(&self) -> usize {
        return self.length;
    }

    pub fn line(&self) -> usize {
        return self.line;
    }

    pub fn column(&self) -> usize {
        return self.column;
    }

    pub fn error(&self) -> Option<&'static str> {
        return self.error;
    }
}
