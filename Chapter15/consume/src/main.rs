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

    let packed =  "Helen of Troy";
    let spaced =  "Helen  of  Troy";
    let obscure = "Helen of Sandusky"; // nice person, just not famous
    assert!(packed != spaced);
    assert!(packed.split_whitespace().eq(spaced.split_whitespace()));
    assert!(spaced < obscure);
    assert!(spaced.split_whitespace().gt(obscure.split_whitespace()));

    let text = "Xerxes";
    assert_eq!(text.chars().position(|c| c == 'e'), Some(1));
    assert_eq!(text.chars().position(|c| c == 'z'), None);

    let a = [5, 6, 7, 8, 9, 10];
    println!("{}", a.iter().fold(0, |n, _| n+1));
    println!("{}", a.iter().fold(0, |n, i| n+i));
    println!("{}", a.iter().fold(1, |n, i| n*i));
    println!("{}", a.iter().fold(i32::min_value(), |m, &i| std::cmp::max(m,i)));

    let a = ["Pack ", "my ", "box ", "with ", "five ", "dozen ", "liquor ", "jugs"];
    let pangram = a.iter().fold(String::new(), |mut s, &w| { s.push_str(w); s });
    println!("{}", pangram);

    let mut squares = (0..10).map(|i| i*i);
    if let Some(n) = squares.nth(4) {
        println!("{}", n);
    }
    if let Some(n) = squares.nth(0) {
        println!("{}", n);
    }
    if let Some(n) = squares.nth(6) {
        println!("{}", n);
    } else {
        println!("None");
    }
}
