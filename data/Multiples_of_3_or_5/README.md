# Multiples of 3 and 5
---
## Problem:
If we list all the natural numbers below **10** that are multiples of **3** or **5**, we get and . The sum of these multiples is **23** . Similarly find the sum of all the multiples of **3** or **5** below *N* . 
## Solution and Approaches:
---
### Noob way:
Well, we know  that we have to find all such numbers that's a multiple of **3** and **5** , so obviously the first solution will be to use for loops and sum the numbers if they match a condition.
```rust
let mut sum: u64 = 0;
for number in 1..n {
    if number%3 ==0 || number%5 ==0 {
        sum = sum + number;
    }
}
println!("{}", sum);
```
This is a simple solution that can just work. But in case of multiple test cases or examples we might need to optimize it better. Take the example given the `data.txt`
```bash
2
10
100
```
The first line `2` is the number of test cases, and the following numbers are the *N* for our problem. In the case that the number of test cases range up to  $10^5$ , then to keep the program execution with 7 seconds is a impossible for this logic.

### Mathematical Approach:
The glaring problem in the previous piece of code is that we are looping over all numbers and checking them that if they are an multiple, This is not efficient. Since there is a pattern for the multiples ( 3, 6, 9 increments of 3), we can loop over with that step and sum it up for an answer. The problem here is that we need to account for repeats (15, 30, 45). We can do this either by checking the number in one of the loops, or running a entire for loop for increment 15 and then subtracting the result.
```rust
let mut sum: u64 = 0;
for number in (0..n).step_by(3) {
    sum = sum + number;
}
for number in (0..n).step_by(5) {
    sum = sum + number;
}
for number in (0..n).step_by(15) {
    sum = sum - number;
}
println!("{}", sum);
```
Although we optimized the logic for speed, it's nowhere near the time we need ( 7 seconds ),  Assume we get multiple test cases within a single execution of the program, we can also sort the test cases and then use the previous index as the starting point of the for loop to avoid repeating loop over ranges. 
```rust
let stdin = io::stdin();
let mut stdin_iterator = stdin.lock().lines();

let t = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();
let mut numbers: Vec<u64> = vec![];
let mut answers: HashMap<u64, u64> = HashMap::new();
for _ in 0..t {
    let number = stdin_iterator.next().unwrap().unwrap().trim().parse::<u64>().unwrap();
    numbers.push(number);
}
let mut numbers_ = numbers.clone();
numbers_.sort();
let (mut number, mut result) = (0 as u64,0 as u64);
for n in numbers_.iter(){
    let multiples: Vec<u64> = (number..*n)
    .filter(|n| n % 3 == 0 || n % 5 == 0)
    .collect();
    let sum: u64 = multiples.iter().sum();
    answers.insert(*n, sum + result);
    result = sum;
    number = *n;

}
for n in numbers.iter(){
    let value = answers.get(n).unwrap();
    println!("{value}");
}
```
Did it help ?  - No.  Some test cases failed because the results were wrong. Let's go back to basics and see if we can figure out better optimization.
### True Solution:
