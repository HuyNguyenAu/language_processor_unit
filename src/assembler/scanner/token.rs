#[derive(Debug, PartialEq, Clone)]
pub enum TokenType {
    // Single-character.
    COMMA,

    // Literals.
    IDENTIFIER,
    STRING,
    NUMBER,

    // Data movement keywords.
    LI,
    LF,
    MV,
    // Semantic operations keywords.
    ADD,
    SUB,
    MUL,
    DIV,
    INF,
    ADT,
    // Heuristic operations keywords.
    EQV,
    INT,
    HAL,
    SIM,
    // Control flow keywords.
    BEQ,
    BLE,
    BLT,
    BGE,
    BGT,
    LABEL,
    // I/O keywords.
    OUT,

    // Misc.
    EOF,
    ERROR,
    EXIT,
}

pub static TOKEN_TYPE_MAPPING: [(TokenType, &str); 20] = [
    // Data movement.
    (TokenType::LI, "li"),
    (TokenType::LF, "lf"),
    (TokenType::MV, "mv"),
    // Semantic operations.
    (TokenType::ADD, "add"),
    (TokenType::SUB, "sub"),
    (TokenType::MUL, "mul"),
    (TokenType::DIV, "div"),
    (TokenType::INF, "inf"),
    (TokenType::ADT, "adt"),
    // Heuristic operations.
    (TokenType::EQV, "eqv"),
    (TokenType::INT, "int"),
    (TokenType::HAL, "hal"),
    (TokenType::SIM, "sim"),
    // Control flow.
    (TokenType::BEQ, "beq"),
    (TokenType::BLT, "blt"),
    (TokenType::BLE, "ble"),
    (TokenType::BGE, "bge"),
    (TokenType::BGT, "bgt"),
    // I/O.
    (TokenType::OUT, "out"),
    // Misc.
    (TokenType::EXIT, "exit"),
];

#[derive(Clone, Debug)]
pub struct Token {
    token_type: TokenType,
    start: usize,
    end: usize,
    line: usize,
    column: usize,
    error: Option<&'static str>,
}

impl Token {
    pub fn new(
        token_type: TokenType,
        start: usize,
        end: usize,
        line: usize,
        column: usize,
        error: Option<&'static str>,
    ) -> Token {
        Token {
            token_type,
            start,
            end,
            line,
            column,
            error,
        }
    }

    pub fn token_type(&self) -> &TokenType {
        &self.token_type
    }

    pub fn start(&self) -> usize {
        self.start
    }

    pub fn end(&self) -> usize {
        self.end
    }

    pub fn line(&self) -> usize {
        self.line
    }

    pub fn column(&self) -> usize {
        self.column
    }

    pub fn error(&self) -> Option<&'static str> {
        self.error
    }
}
