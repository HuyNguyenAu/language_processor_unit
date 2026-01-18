#[derive(Debug, PartialEq, Clone)]
pub enum TokenType {
    // Single-character.
    COMMA,
    // SEMICOLON,
    // MINUS,
    // PLUS,
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
    LABEL,
    JLT,
    // JMP,
    // IN,
    OUT,
    // HALT,

    // Misc.
    EOF,
    ERROR,
}

#[derive(Clone, Debug)]
pub struct Token {
    pub token_type: TokenType,
    pub start: usize,
    pub length: usize,
    pub line: usize,
    pub column: usize,
    pub error: Option<&'static str>,
}
