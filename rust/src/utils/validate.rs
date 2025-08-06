use std::process::exit;
use std::fs;

pub fn process_exit(filename: &str) {
    println!("The file {filename} does not exist -_- Are you serious ?");
    exit(0);
}

pub fn check_file(filename: &str) -> bool{
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
