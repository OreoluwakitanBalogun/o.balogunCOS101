fn main() {
    
    let p: f64 = 210_000.00;

    
    let r: f64 = 5.00;

   
    let n: f64 = 3.00;

    
    let a = p * (1.0 - (r / 100.0)).powi(n as i32);

    
   

    
    
    
    println!("Value of TV after 3 years (a) : ₦{:.2}", a);
}