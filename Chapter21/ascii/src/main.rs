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

        /// Construsct an `Ascii` value from `bytes`, without checking
        /// whether `bytes` actually contains well-formed ASCII.
        /// 
        /// This constructor is infallible, and returns an `Ascii` directly,
        /// rather than a `Result<Ascii, NotAsciiError>` as the `from_bytes`
        /// constructor does.
        /// 
        /// # Safety
        /// 
        /// The caller must ensure that `bytes` contains only ASCII
        /// characters: bytes no greater than 0x7f. Otherwise, the effect is undefined.
        /// 
        pub unsafe fn from_bytes_unchecked(bytes: Vec<u8>) -> Ascii {
            Ascii(bytes)
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

fn very_trustworthy(shared: &i32) {
    unsafe {
        // Turn the shared reference into a mutable pointer.
        // This is undefined behavior.
        let mutable = shared as *const i32 as *mut i32;
        *mutable = 20;
    }
}

pub unsafe trait Zeroable {}

unsafe impl Zeroable for u8 {}
unsafe impl Zeroable for i32 {}
unsafe impl Zeroable for usize {}
// and so on for all the integer types



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

    // Imagine that this vector is the result of some complicated process
    // that we expected to produce ASCII. Something went wrong!
    let bytes = vec![0xf7, 0xbf, 0xbf, 0xbf];

    let ascii = unsafe {
        // This unsafe function's contract is violated
        // when `bytes` holds non-ASCII bytes.
        Ascii::from_bytes_unchecked(bytes)
    };

    let bogus: String = ascii.into();

    // `bogus` now holds ill-formed UTF-8. Parsing its first character
    // produces a `char` that is not a valid Unicode point.
    println!("{}", bogus.chars().next().unwrap());

    // very_trustworthy(&10);
    println!("{}", 1000);

    let i = 10;
    very_trustworthy(&i);
    println!("{}", i * 100);
}
