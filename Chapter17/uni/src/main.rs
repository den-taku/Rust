fn latin1_to_char(latin1: u8) -> char {
    latin1 as char
}

fn char_to_latin1(c:char) -> Option<u8> {
    if c as u32 <= 0xff {
        Some(c as u8)
    } else {
        None
    }
}

fn main() {
    println!("Hello, world!");
}
