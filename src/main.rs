mod cli;

use clap::Parser;
use cli::BosonCli;
use std::fs::File;

fn main() {
    let args = BosonCli::parse();
    println!("{args:?}");
    let f = File::open("path");
}
