use std::fs;
use std::path::PathBuf;
use std::process::exit;

use clap::Args;

#[derive(Args, Debug)]
pub struct MultiplesOf3and5 {
    // The filename containing test cases.
    #[arg(short, long)]
    pub filename: Option<PathBuf>,

    // A number to calculate the multiples for.
    #[arg(short, long)]
    pub number: Option<u64>,
}

fn process_exit(filename: &str) {
    println!("The file {filename} does not exist -_- Are you serious ?");
    exit(0);
}

fn check_file(filename: &str) -> bool{
    let contents: String = fs::read_to_string(filename).expect("Unable to read the file.");
    // Assuming the file contains one test case, then it must be of n, n lines, so total n+1 lines, 
    // Check that condition.
    let length = contents.lines().nth(0).expect("Not able to read the line.");
    let length: u8 = length.parse().unwrap();
    let lines = contents.lines().count() as u8;
    if lines == length + 1 as u8 {
        return true;
    }
    return false;
}

fn solvefile(filename: PathBuf){
    let filename = filename.to_str().expect("Unable to read the filename.");
    match fs::exists(filename) {
        Ok(true) => (),
        _ => process_exit(filename),
    }
    match check_file(filename) {
        true => (),
        false => exit(1),
    }
    println!("Problem :");
    println!("-------------------------------- :");
    println!("  For each test case, you get a number N. Your task is to find the sum of all natural numbers less than N that are multiples of 3 or 5.");
    println!("");
    let contents: String = fs::read_to_string(filename)
        .expect("Should have been able to read the file");
    for line in contents.lines().skip(1) {
        let number: u64  = line.parse().expect("Not a number.");
        let sum: u64 = sum_of_multiples(3, number) + sum_of_multiples(5, number) - sum_of_multiples(15, number);
        println!("Number : {number} and Sum : {}", sum);
    }
}

fn sum_of_multiples(factor: u64, n: u64) -> u64 {
    let limit: u64;
    if n%factor == 0 {
        let n = n -1 ;
        limit = (n - n%factor)/factor;
    }
    else {
        limit = (n - n%factor)/factor;
    }
    let sum =  factor * limit * (limit + 1 )/2;
    return sum;
}   

fn solvenum(n: u64) {
    println!("Problem :");
    println!("-------------------------------- :");
    println!("  For each test case, you get a number N. Your task is to find the sum of all natural numbers less than N that are multiples of 3 or 5.");
    println!("");
    let sum: u64 = sum_of_multiples(3, n) + sum_of_multiples(5, n) - sum_of_multiples(15, n);
    println!("Number : {n} and Sum : {}", sum);
}

pub fn solve(arguments: MultiplesOf3and5){
    match arguments.filename {
        Some(filename) => solvefile(filename),
        None => (),
    }
    
    match arguments.number {
        Some(num) => solvenum(num),
        None => (),
    }



    // let mut numbers: Vec<u64> = vec![];
    // let mut answers: HashMap<u64, u64> = HashMap::new();
    // for line in contents.lines().skip(1) {
    //     let number: u64 = line.parse().unwrap();
    //     numbers.push(number);
        
    // }
    // let mut numbers_ = numbers.clone();
    // numbers_.sort();
    // let (mut number, mut result) = (0 as u64,0 as u64);
    // for n in numbers_.iter(){
    //     let multiples: Vec<u64> = (number..*n)
    //     .filter(|n| n % 3 == 0 || n % 5 == 0)
    //     .collect();
    //     let sum: u64 = multiples.iter().sum();
    //     // println!("{}", sum + result);
    //     answers.insert(*n, sum + result);
    //     result = sum;
    //     number = *n;

    // }
    // println!("{:?}", answers);
    // for n in numbers.iter(){
    //     let value = answers.get(n).unwrap();
    //     println!("{value}");
    // }
    
}   