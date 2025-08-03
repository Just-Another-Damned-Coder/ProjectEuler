use std::{fs, vec};
use std::process::exit;

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


pub fn solve(filename: &str){
    match fs::exists(filename) {
        Ok(true) => (),
        _ => process_exit(filename),
    }
    match check_file(filename) {
        true => (),
        false => exit(1),
    }
    println!("For each test case, you get a number N. Your task is to find the sum of all natural numbers less than N that are multiples of 3 or 5.");
    let contents: String = fs::read_to_string(filename)
        .expect("Should have been able to read the file");
    for line in contents.lines().skip(1) {
        let number: u8 = line.parse().unwrap();
        let mut vectors: Vec<u16> = vec![];
        for n in 1..number{
            if n%3 ==0 || n%5 ==0 {
                vectors.push(n as u16);
            }
        }
        let sum:u16 = vectors.iter().sum();
        println!("{}", sum);
    }
}   