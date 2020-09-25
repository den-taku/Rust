fn main() {
    let v = (0..30).collect::<Vec<i32>>();
    println!("{}", v.iter().peekable().fold(0, |sum, e| {sum + e + peek().expect("hoge")}));
}
