#[macro_use]
extern crate lalrpop_util;

mod cli;
mod compilation;

use compilation::frontend::lexical_analysis::vocabulary;
use compilation::frontend::syntax_analysis::grammar;
use logos::Logos;

#[tokio::main]
async fn main() {
    let mut lexer = vocabulary::Token::lexer("use someid1.someid2;")
        .spanned()
        .map(vocabulary::Token::to_lalr_triple);
    let parser = grammar::StatementParser::new();
    let _ = parser.parse(lexer).unwrap();
}
