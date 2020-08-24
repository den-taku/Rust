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
    assert_eq!("うどん: udon".as_bytes(), 
               &[0xe3, 0x81, 0x86, // う
                 0xe3, 0x81, 0xa9, // ど
                 0xe3, 0x82, 0x93, // ん
                 0x3a, 0x20, 0x75, 0x64, 0x6f, 0x6e // : udon
               ]);
}
