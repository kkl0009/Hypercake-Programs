/*
* Name: Kollin Labowski
* Course: CS 310
* Description: This program implements the HyperCake problem using the Rust programming language.
* Academic Integrity: This program complies with the academic integrity policies of WVU and CS 310
*/

//Used for getting user input
use std::io;
use std::io::*;

//Main method
fn main() {
    let mut inputA = String::new();
    println!("Enter the amount of cuts (n): ");
    io::stdin().read_line(&mut inputA).unwrap();
    let n: i128 = inputA.trim().parse().unwrap();
    let mut inputB = String::new();
    println!("Enter the amount of dimensions (k): ");
    io::stdin().read_line(&mut inputB).unwrap();
    let k: i128 = inputB.trim().parse().unwrap();
    
    println!("The answer is: {}", hypercake(n, k));
}

//hypercake method uses recursion and two embedded methods combinations and factorial
fn hypercake(n: i128, k: i128) -> i128{
    fn combinations(n: i128, r: i128) -> i128{
        fn factorial(n: i128) -> i128{
            if n <= 1 {
                return 1;
            }
            
            return n * factorial(n - 1);
        }
        
        let result = factorial(n) / (factorial(r) * factorial(n - r));
        return result;
    }
    
    if k <= 0 {
        return combinations(n, k);
    }
    return combinations(n, k) + hypercake(n, k-1);
}