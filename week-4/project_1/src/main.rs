// Rust program to find quadratic roots
use std::io;

fn main(){
    println!("enter the value of a");
    let mut input1 = String::new();
    io::stdin().read_line(&mut input1).expect("failed to read input");
    let a:f64 = input1.trim().parse().expect("failed to input");

    println!("enter the value of b");
    let mut input2 = String::new();
    io::stdin().read_line(&mut input2).expect("failed to read input");
    let b:f64 = input2.trim().parse().expect("failed to input");

    println!("enter the value of c");
    let mut input3 = String::new();
    io::stdin().read_line(&mut input3).expect("Ffailed to read input");
    let c:f64 = input3.trim().parse().expect("failed to input");

    let d = b*b - 4.0*a*c;

    if d > 0.0 {
        let root1 = (-b + d.sqrt()) / (2.0*a);
        let root2 = (-b - d.sqrt()) / (2.0*a);
        println!("two distinct roots");
        println!("root 1 = {}", root1);
        println!("root 2 = {}", root2);
    } else if d == 0.0 {
        let root = -b / (2.0*a);
        println!("exactly one real root");
        println!("root = {}", root);
    } else {
        println!("no real roots");
    }
}