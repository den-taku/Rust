use std::io::{self, Write, Result};

// /// Trait for values to which you can send HTML.
// trait WriteHtml {
//     fn write_html(&mut self, &HJtmlDocument) -> Result<()>;
// }

// /// You can write HTML to any std::io writer.
// impl<W: Write> WriteHtml for W {
//     fn write_html(&mut self, html:&mut HtmlDocument) -> Result<()> {
//         ...
//     }
// }

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

trait IsEmoji {
    fn is_emoji(&self) -> bool;
}

/// Implement IsEmoji for tje built-in character type.
impl IsEmoji for char {
    fn is_emoji(&self) -> bool {
        false
    }
}

fn main() {
    println!("{}", 'h'.is_emoji());
    println!("Hello, world!");
}
