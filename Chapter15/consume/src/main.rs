// use std::io::prelude::*;
use std::cmp::{PartialOrd, Ordering};

fn triangle(n: u64) -> u64 {
    (1..n+1).sum()
}

fn factorial(n: u64) -> u64 {
    (1..n+1).product()
}

// Compare two f64 values. Panic if given a NaN.
fn cmp(lhs: &&f64, rhs: &&f64) -> Ordering {
    lhs.partial_cmp(rhs).unwrap()
}

fn main() {
    // let stdin = std::io::stdin();
    // println!("{}", stdin.lock().lines().count());
    println!("{}", triangle(20));
    println!("{}", factorial(10));
    println!("{}", match [-2, 0, 1, 0, 2, -5].iter().max(){Some(r)=>*r,_=>0});

    let numbers = [1.0, 4.0, 2.0];
    assert_eq!(numbers.iter().max_by(cmp), Some(&4.0));
    assert_eq!(numbers.iter().min_by(cmp), Some(&1.0));
    // let numbers = [1.0, 4.0, std::f64::NAN, 2.0];
    assert_eq!(numbers.iter().max_by(cmp), Some(&4.0));
}
