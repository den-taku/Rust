use std::io::{Write, Result};

/// A Writer that ignores whatever data you write to it.
pub struct Sink;

impl Write for Sink {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        // Claim to have successfully written the whole buffer.
        Ok(buf.len())
    }

    fn flush(&mut self) -> Result<()> {
        Ok(())
    }
}

fn main() {
    println!("Hello, world!");
}
