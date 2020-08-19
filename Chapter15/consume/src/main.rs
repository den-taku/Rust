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
    println!("{}", match [-2, 0, 1, 0, 2, -5].iter().max(){Some(r)=>*r,_=>0});
}
