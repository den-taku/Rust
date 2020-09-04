fn main() {
    let ascii = vec![34, 56];
    let hoge =
    unsafe {
        String::from_utf8_unchecked(ascii)
    };
    println!("{:?}", hoge);

    // let mut a: usize = 0;
    // let ptr = &mut a as *mut usize;
    // unsafe {
    //     *ptr.offset(3) = 0x7ffff72f484c;
    // }
}
