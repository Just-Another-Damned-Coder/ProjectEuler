extern crate clap;
use clap::{Parser, Subcommand};


mod solutuions;
mod utils;

#[derive(Parser)]
#[command(name="Project Euler", about)]
struct Args {
    #[command(subcommand)]
    command: Command,
}
#[derive(Subcommand)]
enum Command {
    #[command(name = "Multiples of 3 or 5")]  // Maps argument to the function
    One(solutuions::one::MultiplesOf3and5),
    #[command(name = "Even Fibonacci Numbers")]
    Two(solutuions::two::EvenFibonacchi),

    // Add other tasks with names "2", "3", etc.
}
fn main() {
    let arguments: Args = Args::parse();
    match arguments.command{
        Command::One(arguments) => solutuions::one::solve(arguments),
        Command::Two(arguments) => solutuions::two::solve(arguments),
    }


}
