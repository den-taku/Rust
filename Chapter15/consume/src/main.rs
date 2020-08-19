// use std::io::prelude::*;

fn triangle(n: u64) -> u64 {
    (1..n+1).sum()
}

fn factorial(n: u64) -> u64 {
    (1..n+1).product()
}

fn main() {
    // let stdin = std::io::stdin();
    // println!("{}", stdin.lock().lines().count());
    println!("{}", triangle(20));
    println!("{}", factorial(10));
}
