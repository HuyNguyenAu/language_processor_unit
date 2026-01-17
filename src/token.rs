#[derive(Debug, PartialEq, Clone)]
pub enum TokenType {
    // Single-character.
    COMMA,
    // SEMICOLON,
    MINUS,
    PLUS,
    // STAR,
    // SLASH,

    // Literals.
    IDENTIFIER,
    STRING,
    NUMBER,

    // Keywords.
    VAR,
    ADD,
    SUB,
    SIM,
    BGE,
    JMP,
    STOP,

    // Misc.
    LABEL,
    EOF,
    ERROR,
}

#[derive(Clone, Debug)]
pub struct Token {
    pub token_type: TokenType,
    pub start: usize,
    pub length: usize,
    pub line: usize,
    pub error: Option<&'static str>,
}
