use std::process::{Command, Stdio};

fn main() {
    let mut child = 
        Command::new("grep")
        .arg("-e")
        .arg("a.*e.*i.*o.*u")
        .stdin(Stdio::piped())
        .spawn().expect("command  error");

    let mut to_child = child.stdin.take().unwrap();
    let my_words = ["Ziburi", "inaka", "nostalgy"];
    for word in &my_words {
        writeln!(to_child, "{}", word).expect("write error");
    }
    drop(to_child);
    child.wait().expect("wait error");
}
