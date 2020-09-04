mod my_ascii {
    use std::ascii::AsciiExt; // for u8::is_ascii

    /// An ASCII-encoded string.
    #[derive(Debug, Eq, PartialEq)]
    pub struct Ascii(
        // This must hold only well_formed ASCII text:
        // bytes from `0` to `0x7f`.
        Vec<u8>
    );

    impl Ascii {
        /// Create an `Ascii` from the ASCII text in `bytes`. Return a
        /// `NotAsciiError` error if `bytes` contains any non_ASCII
        /// characters.
        pub fn from_bytes(bytes: Vec<u8>) -> Result<Ascii, NotAsciiError> {
            if bytes.iter().any(|&byte| !byte.is_ascii()) {
                return Err(NotAsciiError(bytes));
            }
            Ok(Ascii(bytes))
        }
    }

    // When conversion fails, we gibe back the vector we couldn't convert.
    // This should implement `std::error::Error`; omitted for brevity.
    #[derive(Debug, Eq, PartialEq)]
    pub struct NotAsciiError(pub Vec<u8>);

    // Safe, efficient conversion, implemented using unsafe code.
    impl From<Ascii> for String {
        fn from(ascii: Ascii) -> String {
            // If this module has no bugs, this is safe, because
            // well-formed ASCII text is also well_formed UTF-8.
            unsafe { String::from_utf8_unchecked(ascii.0) }
        }
    }
}

fn main() {
    use my_ascii::Ascii;

    let bytes: Vec<u8> = b"ASCII and ye shall receive".to_vec();

    // This call entaild no allocations or text copies, just a scan.
    let ascii: Ascii = Ascii::from_bytes(bytes)
        .unwrap(); // We know these chosen bytes aer ok.

    // This call is zero-cost: no allocation, copies, or scans.
    let string = String::from(ascii);

    println!("{}", string);

    println!("Hello, world!");
}
