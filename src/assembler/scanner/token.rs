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
    // Control flow keywords.
    Beq,
    Ble,
    Blt,
    Bge,
    Bgt,
    Exit,
    // I/O keywords.
    Out,
    // Generative operations keywords.
    Sum,
    Xpn,
    Trn,
    // Cognitive operations keywords.
    Cmp,
    Syn,
    Flt,
    Prd,
    // Guardrails operations keywords.
    Vfy,
    Sim,

    // Misc keywords.
    Label,
    Eof,
    Error,
}

pub static TOKEN_TYPE_MAPPING: [(TokenType, &str); 19] = [
    // Data movement.
    (TokenType::Li, "li"),
    (TokenType::Lf, "lf"),
    (TokenType::Mv, "mv"),
    // Control flow.
    (TokenType::Beq, "beq"),
    (TokenType::Ble, "ble"),
    (TokenType::Blt, "blt"),
    (TokenType::Bge, "bge"),
    (TokenType::Bgt, "bgt"),
    (TokenType::Exit, "exit"),
    // I/O.
    (TokenType::Out, "out"),
    // Generative operations.
    (TokenType::Sum, "add"),
    (TokenType::Xpn, "sub"),
    (TokenType::Trn, "mul"),
    // Cognitive operations.
    (TokenType::Cmp, "div"),
    (TokenType::Syn, "inf"),
    (TokenType::Flt, "adt"),
    (TokenType::Prd, "eqv"),
    // Guardrails operations.
    (TokenType::Vfy, "int"),
    (TokenType::Sim, "hal"),
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
