use std::io::Write;
use std::fs::File;

fn type_of<T>(_: T) -> String {
    std::any::type_name::<T>().to_string()
}

fn say_hello(out: &mut Write) -> std::io::Result<()> {
    out.write_all(b"hello world\n")?;
    out.flush()
}

/// Given two values, pick whichever one is less.
fn min<T: Ord>(value1: T, value2: T) -> T {
    if value1 <= value2 {
        value1
    } else {
        value2
    }
}

fn main() {
    println!("{}", type_of(File::create("hello.txt")));
//    let mut local_file = File::create("hello.txt")?;
//    say_hello(&mut local_file)?;
    
//    let mut bytes = vec![];
//    say_hello(&mut bytes)?;
//    assert_eq!(bytes, b"hello world\n");
}
