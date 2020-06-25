fn main() {
    assert_eq!(false as i32, 0);
    assert_eq!(true  as i32, 1);
    println!("Hello, world!");

    println!("{}", '∫' as u32);
    let point = match std::char::from_u32(8747){
        Some(c) => {c} 
        None => {'0'}
    };
    println!("{}", point)
}
