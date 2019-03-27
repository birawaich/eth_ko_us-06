extern crate num;
extern crate rand;

use num::Complex;
use rand::distributions::{Distribution, Uniform};


/// usage:
/// eth_ko_us-06 <no arguments>
fn main() {
	let z0 = Complex{re: 0.0, im: 0.0};

	println!("ETH: Komplexe Analysis: Übungsserie 06, Aufgabe 5");
	println!("-------------------------------------------------");
	println!("z0 = {}",z0);
	println!("f1(z) = cos(z^3 - sin(z))");
	println!("f2(z) = tan(z^7 + PI/4)");
	println!();
	println!("f1(z0) = {}",function_1(z0));
	println!("f2(z0) = {}",function_2(z0));
	println!();
	println!("(a)");
	println!("Monte Carlo Intergration f1(z): {}",monte_carlo_intergration(1_u8, z0));
	println!("Monte Carlo Intergration f2(z): {}",monte_carlo_intergration(2_u8, z0));
	println!();
	println!("(b)");
	println!("Approximation f1(z): {}",approximation(1_u8, z0));
	println!("Approximation f2(z): {}",approximation(2_u8, z0));
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
	let mut result = Complex{ re: 0.0, im: 0.0};
	let mut rng = rand::thread_rng(); //don't really know what I'm doing --> https://rust-lang-nursery.github.io/rust-cookbook/algorithms/randomness.html
	let random_pool = Uniform::from(0.0_f64..1.0_f64);
	
	//calculation
	for _i in 1..N {
		let random_uniform_number = random_pool.sample(&mut rng);
		match function_id {
			1 => {result += function_1(z0+RADIUS*(2.0*std::f64::consts::PI*I*random_uniform_number).exp());},
			2 => {result += function_2(z0+RADIUS*(2.0*std::f64::consts::PI*I*random_uniform_number).exp());},
			_ => {println!("Did not find function, will return a result of 0");},
		}
	}
	result /= Complex{re: (N as f64), im: 0.0};
	
	result
}

/// does approximation
/// @param function_id what function to calculate
/// @param z0 what is z0
/// @return Complex that is result
fn approximation(function_id: u8, z0: Complex<f64>) -> Complex<f64> {
	//constants
	const N: u8 = 50; //from exercise
	const RADIUS: f64 = 1.0; //from exercise
	const I: Complex<f64> = Complex{re: 0.0, im: 1.0};
	
	//variables
	let mut result = Complex{ re: 0.0, im: 0.0};
	
	//calculation
	for i in 1..N {
		match function_id {
			1 => {result += function_1(z0+RADIUS*(2.0*std::f64::consts::PI*I*((i-1) as f64)/(N as f64)).exp());},
			2 => {result += function_2(z0+RADIUS*(2.0*std::f64::consts::PI*I*((i-1) as f64)/(N as f64)).exp());},
			_ => {println!("Did not find function, will return a result of 0");},
		}
	}
	result /= Complex{re: (N as f64), im: 0.0};
	
	result
}

/// function (1) in task
/// @param z input
/// @return value of function
fn function_1(z: Complex<f64>) -> Complex<f64> {
	(z*z*z-z.sin()).cos()
}

/// function (2) in task
/// @param z input
/// @return value of function
fn function_2(z: Complex<f64>) -> Complex<f64> {
	(z.powf(7.) + std::f64::consts::PI/4.).tan()
}




