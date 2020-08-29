use std::io::{self, Write, BufWriter};
use std::io::prelude::*;
use std::fs::File;

fn main() {
    println!("Hello, world!");
    let numbers = vec![1, 2, 3, 4, 5];
    let d = 2;
    println!("The greatest common divisor of {:?} is {}", numbers, d);
    
    // writeln!(io::stderr(), "error: world no helloable").expect("writeln! err");
    // writeln!(&mut byte_vec, "The greatest common divisor of {:?} is {}", numbers, d);

    let file = File::create("tmp.txt").expect("err");
    let mut writer = BufWriter::new(file);
    writer.write_all(&[32, 33, 34, 35]);
    writer.flush().expect("err");
}
