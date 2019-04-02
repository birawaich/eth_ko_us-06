extern crate num;
extern crate rand;

use num::Complex;
use rand::distributions::{Distribution, Uniform};
use std::f64::consts::PI;

// constants
const RADIUS: f64 = 1.0; //from exercise
const I: Complex<f64> = Complex { re: 0.0, im: 1.0 };
const ZERO: Complex<f64> = Complex { re: 0f64, im: 0f64 };

/// usage:
/// eth_ko_us-06 <no arguments>
fn main() {
    let z0 = ZERO;
    println!("ETH: Komplexe Analysis: Übungsserie 06, Aufgabe 5");
    println!("-------------------------------------------------");
    println!("z0 = {}", z0);
    println!("f1(z) = cos(z^3 - sin(z))");
    println!("f2(z) = tan(z^7 + PI/4)");
    println!();
    println!("f1(z0) = {}", function_1(z0));
    println!("f2(z0) = {}", function_2(z0));
    println!();
    println!("(a)");
    println!(
        "Monte Carlo Integration f1(z0): {}",
        monte_carlo_integration(1_u8, z0)
    );
    println!(
        "Monte Carlo Integration f2(z0): {}",
        monte_carlo_integration(2_u8, z0)
    );
    println!();
    println!("(b)");
    println!("Approximation f1(z0): {}", approximation(1_u8, z0));
    println!("Approximation f2(z0): {}", approximation(2_u8, z0));
    println!("-------------------------------------------------");
}

/// does monte carlo integration
/// @param function_id what function to calculate
/// @param z0 what is z0
/// @return Complex that is result
fn monte_carlo_integration(function_id: u8, z0: Complex<f64>) -> Complex<f64> {
    //constants
    const N: u16 = 1000;

    //variables
    let mut result = ZERO;
    let mut rng = rand::thread_rng(); //don't really know what I'm doing --> https://rust-lang-nursery.github.io/rust-cookbook/algorithms/randomness.html
    let random_pool = Uniform::from(0.0_f64..1.0_f64);
    let mut random_arr = [0f64; N as usize];

    // generate array of random floats
    for i in 1..random_arr.len() {
        random_arr[i as usize] = random_pool.sample(&mut rng);
    }
    // iterate over the elements, adding intermediate results:
    for elt in random_arr.iter() {
        result += func1or2(function_id, *elt, z0);
    }

    // even better: pure functional approach with no state:
    let _result2 = random_arr
        .iter()
        .fold(ZERO, |acc, x| acc + func1or2(function_id, *x, z0));

    result /= Complex {
        re: f64::from(N),
        im: 0.0,
    };

    result
}

/// does approximation
/// @param function_id what function to calculate
/// @param z0 what is z0
/// @return Complex that is result
fn approximation(function_id: u8, z0: Complex<f64>) -> Complex<f64> {
    //constants
    const N: u8 = 50; //from exercise

    //variables
    let mut result = ZERO;

    // build array
    let mut array = [0f64; N as usize];
    for (i, elt) in array.iter_mut().enumerate().skip(1) {
        *elt = i as f64;
    }

    //iterative calculation
    for i in 1..N {
        result += func1or2(function_id, f64::from(i - 1) / f64::from(N), z0);
    }

    // even better: pure functional approach with no state:
    let _result2 = array.iter().fold(ZERO, |acc, x| {
        acc + func1or2(function_id, *x - 1f64 / f64::from(N), z0)
    });

    result /= Complex {
        re: f64::from(N),
        im: 0f64,
    };

    result
}

/// function (1) in task
/// @param z input
/// @return value of function
fn function_1(z: Complex<f64>) -> Complex<f64> {
    (z * z * z - z.sin()).cos()
}

/// function (2) in task
/// @param z input
/// @return value of function
fn function_2(z: Complex<f64>) -> Complex<f64> {
    (z.powf(7.) + PI / 4.).tan()
}

/// condense two functions into one
fn func1or2(function_id: u8, r: f64, z0: Complex<f64>) -> Complex<f64> {
    match function_id {
        1 => function_1(z0 + RADIUS * (2.0 * PI * I * r).exp()),
        2 => function_2(z0 + RADIUS * (2.0 * PI * I * r).exp()),
        _ => {
            eprintln!("Did not find function, will return a result of 0");
            ZERO
        }
    }
}
