extern crate num;

use num::Complex;

/// usage:
/// eth_ko_us-06
fn main() {
    println!("Hello, world!");
    let my_complex = Complex{re: 0.0, im: 0.0};
    println!("Test result: {}",function_1(my_complex));
}

/// does monte carlo intergration
/// @param function_id what function to calculate
/// @param z0 what is z0
/// @return Complex that is result
fn monte_carlo_intergration(function_id: u8, z0: Complex<f64>) -> Complex<f64> {
	//constants
	const N: u16 = 1000; //from exercise
	
	//variables
	let function_id = 1_u8; //get function ID, to test just 1
	let result = Complex{ re: 0.0, im: 0.0};
	
	//calculation
	for i in 1..N {
		
	}
	result
}

/// function (1) in task
fn function_1(z: Complex<f64>) -> Complex<f64> {
	(z*z*z-z.sin()).cos()
}
