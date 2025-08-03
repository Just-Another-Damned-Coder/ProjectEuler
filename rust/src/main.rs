use std::path::PathBuf;
extern crate clap;
use clap::Parser;
mod solutuions;


#[derive(Parser, Debug)]
#[command(about)]
struct Args {
    // The problem number that we want to solve.
    #[arg(short, long, default_value_t=1)]
    number: u8,

    // The file with example test cases.
    #[arg(short, long, value_name="data/one.txt")]
    filename: PathBuf,
}

fn main() {
    
    let arguments: Args = Args::parse();
    match arguments.number{
        1 => solutuions::one::solve(arguments.filename.to_str().expect("Did not recieve correct filename, please check again.")),
        _ => solutuions::one::solve("data/one.txt"),
    }
}
