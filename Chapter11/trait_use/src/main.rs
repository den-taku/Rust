fn main() {
    use std::io::Write;
    let mut buf: Vec<u8> = vec![];
    buf.write_all(b"helo").expect("error");
    let write: &mut Write = &mut buf;
}
