#[derive(Debug, PartialEq, Clone)]
pub enum TokenType {
    // Single-character.
    Comma,

    // Literals.
    Identifier,
    String,
    Number,

    // Data movement keywords.
    Li,
    Lf,
    Mv,
    // Semantic operations keywords.
    Add,
    Sub,
    Mul,
    Div,
    Inf,
    Adt,
    // Heuristic operations keywords.
    Eqv,
    Int,
    Hal,
    Sim,
    // Control flow keywords.
    Beq,
    Ble,
    Blt,
    Bge,
    Bgt,
    Label,
    // I/O keywords.
    Out,

    // Misc.
    Eof,
    Error,
    Exit,
}

pub static TOKEN_TYPE_MAPPING: [(TokenType, &str); 20] = [
    // Data movement.
    (TokenType::Li, "li"),
    (TokenType::Lf, "lf"),
    (TokenType::Mv, "mv"),
    // Semantic operations.
    (TokenType::Add, "add"),
    (TokenType::Sub, "sub"),
    (TokenType::Mul, "mul"),
    (TokenType::Div, "div"),
    (TokenType::Inf, "inf"),
    (TokenType::Adt, "adt"),
    // Heuristic operations.
    (TokenType::Eqv, "eqv"),
    (TokenType::Int, "int"),
    (TokenType::Hal, "hal"),
    (TokenType::Sim, "sim"),
    // Control flow.
    (TokenType::Beq, "beq"),
    (TokenType::Blt, "blt"),
    (TokenType::Ble, "ble"),
    (TokenType::Bge, "bge"),
    (TokenType::Bgt, "bgt"),
    // I/O.
    (TokenType::Out, "out"),
    // Misc.
    (TokenType::Exit, "exit"),
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
