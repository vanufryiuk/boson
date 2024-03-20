use logos::Logos;
use std::fmt::Display;
use std::ops::Range;

#[derive(Logos, Debug, Clone)]
pub enum Token<'source> {
    #[regex(r#"\s+"#)]
    Whitespace,

    #[token("use")]
    KeywordUse,

    #[regex(r#"[a-zA-Z_]+[a-zA-Z0-9_]*"#)]
    Identity(&'source str),

    #[token(".")]
    OperatorDot,

    #[token(",")]
    OperatorComma,

    #[token("*")]
    OperatorAsterisk,

    #[token("{")]
    OperatorBraceLeft,

    #[token("}")]
    OperatorBraceRight,

    #[token(";")]
    OperatorSemicolon,
}

#[derive(Debug)]
pub enum CompilationError {
    LexicalError { range: Range<usize> },
    NumericalError { range: Range<usize> },
    UnterminatedStringLiteral { range: Range<usize> },
}

impl<'source> Token<'source> {
    pub fn to_lalr_triple(
        (t, r): (Result<Token<'source>, ()>, Range<usize>),
    ) -> Result<(usize, Token<'source>, usize), CompilationError> {
        Ok((r.start, t.unwrap(), r.end))
    }
}

impl<'source> Display for Token<'source> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#?}", self)
    }
}
