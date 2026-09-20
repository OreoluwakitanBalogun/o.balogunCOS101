// Rust program for Incentive Calculator
use std::io;

fn main(){
    println!("Enter experience (1 for experienced, 0 for not experienced)");
    let mut input1 = String::new();
    io::stdin().read_line(&mut input1).expect("Failed to read input");
    let experienced:i32 = input1.trim().parse().expect("Failed to input");

    println!("Enter age");
    let mut input2 = String::new();
    io::stdin().read_line(&mut input2).expect("Failed to read input");
    let age:i32 = input2.trim().parse().expect("Failed to input");

    if experienced == 1 {
        if age >= 40 {
            println!("Annual incentive = N1,560,000");
        } else if age >= 30 && age <= 39 {
            println!("Annual incentive = N1,480,000");
        } else if age < 28 {
            println!("Annual incentive = N1,300,000");
        } else {
            println!("No matching incentive for this age");
        }
    } else {
        println!("Annual incentive = N100,000");
    }
}