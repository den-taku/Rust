use std::path::PathBuf;
use std::borrow::Cow;

fn describe(error: &Error) -> Cow<'static, str> {
    match *error {
        Error::OutOfMemory   => "out of memory".into(),
        Error::StackOverflow => "stack overflow".into(),
        Error::MachineOnFire => "machine on fire".into(),
        Error::Unfathomable  => "machine bewildered".into(),
        Error::FileNotFind(ref path) => {
            format!("file not found: {}", path.display()).into()
        }
    }
}

fn main() {
    println!("Disaster has struck: {}", describe(&error));
}

