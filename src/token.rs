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
    JLT,
    // OUT,

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
    pub error: Option<String>,
}
