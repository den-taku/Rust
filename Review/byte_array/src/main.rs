fn main() {
    let array = [b'R', b'u', b's', b't', b' ', b'i', b's', b' ', b'p', b'e', b'r', b'f', b'e', b'c', b't'];
    let string: Vec<char> = array.iter().map(|ch| {*ch as char}).collect();
    println!("{:?}", array);
    println!("{:?}", string);
    let uni: char = '\u{673}';
    println!("{}", uni);
}
