use thiserror::Error;

#[derive(Error, Debug)]
pub(crate) enum LexError {
    #[error("Empty JSON file is invalid.")]
    EmptyFile,

    #[error("[line {line:?}] Trailing Comma.")]
    TrailingComa { line: i16 },

    #[error("Expected opening curly braces '{{'")]
    OpeningCurly,

    #[error("Expected closing curly braces '}}'")]
    ClosingCurly,

    #[error("[line {line:?}] Unexpected token {token:?}")]
    UnexpectedToken { line: i16, token: char },

    #[error("[line {line:?}] Invalid Syntax, Expected {expected:?}.")]
    InvalidSyntax { line: i16, expected: String },

    #[error("[line {line:?}] Invalid keyword {keyword:?}, Maybe you mean {maybe:?}.")]
    InvalidKeyword {
        line: i16,
        keyword: String,
        maybe: String,
    },

    #[error("[line {line:?}] Missing closing quote.")]
    MissingClosingQuote { line: i16 },

    #[error("[line {line:?}] Unterminated string literal.")]
    UnterminatedString { line: i16 },

    #[error("[line {line:?}] Unterminated number.")]
    UnterminatedNumber { line: i16 },

    #[error("[line {line:?}] Unterminated Array, Expected ].")]
    UnterminatedArray { line: i16 },
}
