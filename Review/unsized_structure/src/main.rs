// This programm cannot be compiled.

mod static_str;
use static_str::hoge;

struct UnsizedStructure { 
    field1: i32,
    field2: str
}

// const hoge: &str = "hoge";

fn main() {
    println!("Hello, world!");
    let s: &str = "hoge";
    let a: &UnsizedStructure = 
        &UnsizedStructure { field1: 32, field2: *hoge };
}
