pub mod parse;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "boson")]
pub enum BosonCli {
    Parse(parse::Parse),
}
