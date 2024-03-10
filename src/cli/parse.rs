use clap::Args;

#[derive(Args, Debug)]
#[command(name = "parse")]
pub struct Parse {
    #[arg(long = "input", short = 'i')]
    pub input: String,
}
