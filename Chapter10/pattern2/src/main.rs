fn main() {
    let int = 2;
    match int {
        0 ... 2 => println!("0...2"),
        _       => ()
    }
}
