fn main() {
    let s1 = "hello".to_string();
    let s2 = "hello".to_string();
    println!("{:p}", &s1 as &str);
    println!("{:p}", &s2 as &str);
}
