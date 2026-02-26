#[derive(Debug, PartialEq, Clone)]
pub enum TokenType {
    // Single-character.
    Comma,
    // Literals.
    Identifier,
    String,
    Number,
    // Data movement keywords.
    LoadString,
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

impl TryFrom<&str> for TokenType {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, <TokenType as TryFrom<&str>>::Error> {
        match value {
            // Data movement.
            "ls" => Ok(TokenType::LoadString),
            "lf" => Ok(TokenType::LoadFile),
            "li" => Ok(TokenType::LoadImmediate),
            "mv" => Ok(TokenType::Move),
            // Control flow.
            "beq" => Ok(TokenType::BranchEqual),
            "ble" => Ok(TokenType::BranchLessEqual),
            "blt" => Ok(TokenType::BranchLess),
            "bge" => Ok(TokenType::BranchGreaterEqual),
            "bgt" => Ok(TokenType::BranchGreater),
            "exit" => Ok(TokenType::Exit),
            // I/O.
            "out" => Ok(TokenType::Out),
            // Generative operations.
            "mrf" => Ok(TokenType::Morph),
            "prj" => Ok(TokenType::Project),
            // Cognitive operations.
            "dst" => Ok(TokenType::Distill),
            "cor" => Ok(TokenType::Correlate),
            // Guardrails operations.
            "aud" => Ok(TokenType::Audit),
            "sim" => Ok(TokenType::Similarity),
            _ => Err("String does not correspond to any known token type."),
        }
    }
}

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
