extern crate rayon;
use rayon::prelude::*;
use std::collections::HashMap;
use std::io;

type GigabyteMap = HashMap<char, String>;

fn process_files_in_parallel(filename: Vec<String>, glossary: &GigabyteMap)
    -> io::Result<()>
{
    filename.par_iter()
        .map(|filename| process_file(filename, glossary))
        .reduce_with(|r1, r2| {
            if r1.is_err() { r1 } else { r2 }
        })
        .unwrap_or(Ok(()))
}

fn main() {
    // "do 2 things in parallel"
    let (v1, v2) = rayon::join(fn1, fn2);

    // "do N things in parallel"
    giant_vector.par_iter().for_each(|value| {
        do_thing_with_value(value);
    });
    
    println!("Hello, world!");
}
