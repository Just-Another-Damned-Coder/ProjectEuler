extern crate clap;
use clap::{Parser, Subcommand};
use sysinfo::{System, RefreshKind, CpuRefreshKind};
use std::thread::sleep;


mod solutuions;

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
    // Add other tasks with names "2", "3", etc.
}
fn main() {
    let mut sys = System::new_all();
    let pid = sysinfo::get_current_pid().expect("Unable to get Current PID.");
    // First update to get accurate initial CPU stats
    sleep(sysinfo::MINIMUM_CPU_UPDATE_INTERVAL);
    sys.refresh_all();

    let arguments: Args = Args::parse();
    match arguments.command{
        Command::One(arguments) => solutuions::one::solve(arguments),
    }

    // Refresh all stats again before reading process info
    sys.refresh_all();

    // Get process info by PID
    if let Some(process) = sys.process(pid) {
        println!("Stats for process with PID {}:", pid);
        println!("CPU usage: {:.2}%", process.cpu_usage());
        println!("Memory usage: {} MB", process.memory()as f64 / 1024.0);
        println!("Virtual memory usage: {} MB", process.virtual_memory()as f64 / 1024.0);
    } else {
        println!("No process found with PID {}", pid);
    }
    println!("Total memory: {} MB", sys.total_memory() as f64 / 1024.0);
}
