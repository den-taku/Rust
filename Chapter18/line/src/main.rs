use std::io;
use std::io::prelude::*;

fn grep<R>(target: &str, reader: R) -> io::Result<()> 
    where R: BufRead
{
    for line_result in reader.lines() {
        let line = line_result?;
        if line.contains(target) {
            println!("{}", line);
        }
    }
    Ok(())
}

fn main() {
    let target = "hoge"; 

    let stdin = io::stdin();
    grep(&target, stdin.lock()).expect("err");

    // let f = File::open(file).expect("err");
    // grep(&target, BufReader::new(f)).expect("err");
}
