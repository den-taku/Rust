fn main() {
    println!("{}", '4'.is_numeric());
    println!("{}", '\n'.is_control());
    println!("{}", ' '.is_whitespace());
    if let Some(num) = 'a'.to_digit(11) {
        println!("{}", num);
    }
    if let Some(ch) = std::char::from_digit(11, 16) {
        println!("{}", ch);
    }
}
