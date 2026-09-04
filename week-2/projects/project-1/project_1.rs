fn main() {
    
    let p: f64 = 520_000_000.0;

    
    let r: f64 = 10.0;

   
    let n: f64 = 5.00;

    
    let a = p * (1.0 + (r / 100.0)).powi(n as i32);

    
    let ci = a - p;

    
    
    
    println!("Compound Interest  : ₦{:.2}", ci);
}