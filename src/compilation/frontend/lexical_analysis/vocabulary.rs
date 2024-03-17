use logos::Logos;

#[derive(Logos)]
pub enum Token<'source> {
    #[token("use")]
    KeywordUse,

    #[token("true")]
    LiteralBoolTrue,

    #[token("false")]
    LiteralBoolFalse,

    #[regex(r#""[^"]*""#)]
    LiteralString(&'source str),
}
