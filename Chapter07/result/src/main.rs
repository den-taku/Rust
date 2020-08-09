use std::io::{self, BufRead};

type GenError = Box<std::error::Error>;
type GenResult<T> = Result<T, GenError>;

/// Read integers from a text file.
/// The file should have one number on each line.
fn read_numbers(file: &mut BufRead) -> Result<Vec<i64>, io::Error>{
    let mut numbers = vec![];
    for line_result in file.lines() {
        let line = line_result?;       // reading lines can fail
        numbers.push(line.parse()?);   // parsing integers cae fail
    }
    Ok(numbers)
}

fn main() {
    println!("Hello, world!");

    // let io_error = io::Error::new(
    //     io"::ErrorKind::Other, "timed out");
    // return Err(GenError::from(io_error));
}
