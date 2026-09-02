fn main() {
    
    let p: f64 = 520_000_000.0;

    
    let r: f64 = 10.0;

   
    let n: u32 = 5;

    
    let a = p * (1.0 + (r / 100.0)).powi(n as i32);

    
    let ci = a - p;

    
    
    
    println!("Compound Interest (CI) : ₦{:.2}", ci);
}