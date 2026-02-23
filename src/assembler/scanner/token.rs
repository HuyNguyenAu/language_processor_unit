#[derive(Debug, PartialEq, Clone)]
pub enum TokenType {
    // Single-character.
    Comma,
    // Literals.
    Identifier,
    String,
    Number,
    // Data movement keywords.
    LoadImmediate,
    LoadFile,
    Move,
    // Control flow keywords.
    BranchEqual,
    BranchLessEqual,
    BranchLess,
    BranchGreaterEqual,
    BranchGreater,
    Exit,
    // I/O keywords.
    Out,
    // Generative operations keywords.
    Morph,
    Project,
    // Cognitive operations keywords.
    Distill,
    Correlate,
    // Guardrails operations keywords.
    Audit,
    Similarity,
    // Misc keywords.
    Label,
    Eof,
    Error,
}

pub static TOKEN_TYPE_MAPPING: [(TokenType, &str); 16] = [
    // Data movement.
    (TokenType::LoadImmediate, "li"),
    (TokenType::LoadFile, "lf"),
    (TokenType::Move, "mv"),
    // Control flow.
    (TokenType::BranchEqual, "beq"),
    (TokenType::BranchLessEqual, "ble"),
    (TokenType::BranchLess, "blt"),
    (TokenType::BranchGreaterEqual, "bge"),
    (TokenType::BranchGreater, "bgt"),
    (TokenType::Exit, "exit"),
    // I/O.
    (TokenType::Out, "out"),
    // Generative operations.
    (TokenType::Morph, "mrf"),
    (TokenType::Project, "prj"),
    // Cognitive operations.
    (TokenType::Distill, "dst"),
    (TokenType::Correlate, "cor"),
    // Guardrails operations.
    (TokenType::Audit, "aud"),
    (TokenType::Similarity, "sim"),
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
