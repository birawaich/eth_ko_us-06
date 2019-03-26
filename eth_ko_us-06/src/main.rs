extern crate num;
extern crate rand;

use num::Complex;
use rand::distributions::{Distribution, Uniform};


/// usage:
/// eth_ko_us-06
fn main() {
    println!("Hello, world!");
    let my_complex = Complex{re: 0.0, im: 0.0};
    println!("Test result: {}",function_1(my_complex));
    println!("Monte Carlo Intergration with Function 1: {}",monte_carlo_intergration(1_u8, my_complex));
}

/// does monte carlo intergration
/// @param function_id what function to calculate
/// @param z0 what is z0
/// @return Complex that is result
fn monte_carlo_intergration(function_id: u8, z0: Complex<f64>) -> Complex<f64> {
	//constants
	const N: u16 = 1000; //from exercise
	const RADIUS: f64 = 1.0; //from exercise
	const I: Complex<f64> = Complex{re: 0.0, im: 1.0};
	
	//variables
	let mut function_id = 1_u8; //get function ID, to test just 1
	let mut result = Complex{ re: 0.0, im: 0.0};
	let mut rng = rand::thread_rng(); //don't really know what I'm doing --> https://rust-lang-nursery.github.io/rust-cookbook/algorithms/randomness.html
	let random_pool = Uniform::from(0.0_f64..1.0_f64);
	
	//calculation
	for i in 1..N {
		let random_uniform_number = random_pool.sample(&mut rng);
		result += function_1(z0+RADIUS*(2.0*std::f64::consts::PI*I*random_uniform_number).exp());
	}
	result /= Complex{re: (N as f64), im: 0.0};
	
	result
}

/// function (1) in task
fn function_1(z: Complex<f64>) -> Complex<f64> {
	(z*z*z-z.sin()).cos()
}
