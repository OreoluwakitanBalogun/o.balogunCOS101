fn main () {
	let P:f64 = 520_000_000.00;
	let R:f64 =10.00;
	let n:f64 = 10.00;
	let A= P*(1+ (R/100.00)).powf(n);
	println!("A is {} ", A) 
}