use std::io::Write;
use std::fmt::Debug;
use std::hash::Hash;

// fn say_hello(out: &mut Write) -> std::io::Result<()> // use trait object
fn say_hello<W: Write>(out: &mut W) -> std::io::Result<()> { // use generic function
    out.write_all(b"hello world\n")?;
    out.flush()
}

fn top_ten<T: Debug + Hash + Eq, D>(values: &Vec<T>) {
    ;
}

fn top_two<T, D>(values: &Vec<T>) -> i32
    where T: Debug + Hash + Eq,
          D: Debug + Write               
{
    32
}

fn main() {
    use std::io::Write;
    let mut buf: Vec<u8> = vec![];
    buf.write_all(b"helo").expect("error");
    say_hello(&mut buf).expect("error");
    {
        let write: &mut Write = &mut buf;
    }
    println!("{:?}", buf);
}
