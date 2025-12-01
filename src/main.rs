mod cli;
use clap::Parser;

fn main() {
    let args = cli::Args::parse();
    println!("Parsed args: {:#?}", args);
}
