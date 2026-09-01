fn main(){
	let p:f64 = 210_000.00;
	let r:f64 = 5.0;
	let n:f64 = 3.0;

	//compound interest
	let a = p * (1.0 - (r / 100.0)).powf(n); //powf is power for float 64
	let dp = p - a;
	println!("Amount is {} and depreciation is {}", a, dp);
}