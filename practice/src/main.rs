fn main() {
    let p: f64 = 520_000_000.00;
    let r: f64 = 10.00;
    let n: f64 = 10.00;
    let a = p * (1.0 + (r / 100.00)).powf(n as f64);
    println!("a is {} ", a)
}
