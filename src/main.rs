mod cli;

use clap::Parser;
use cli::BosonCli;

fn main() {
    let args = BosonCli::parse();
    println!("{args:?}");
}
