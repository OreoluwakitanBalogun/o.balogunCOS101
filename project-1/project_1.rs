fn main() {
    // Principal amount (P)
    let p: f64 = 520_000_000.0;

    // Rate of interest (R)
    let r: f64 = 10.0;

    // Time in years (n)
    let n: u32 = 5;

    // Calculate Amount using the formula: A = P [1 + (R/100)]^n
    let a = p * (1.0 + (r / 100.0)).powi(n as i32);

    // Calculate Compound Interest: CI = A - P
    let ci = a - p;

    // Display the results
    println!("Principal (P)          : ₦{:.2}", p);
    println!("Rate (R)               : {}%", r);
    println!("Time (n)               : {} years", n);
    println!("--------------------------------");
    println!("Amount (A)             : ₦{:.2}", a);
    println!("Compound Interest (CI) : ₦{:.2}", ci);
}