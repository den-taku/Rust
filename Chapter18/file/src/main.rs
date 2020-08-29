use std::fs::OpenOptions;

fn main() {
    let log = OpenOptions::new()
        .append(true) // if file exists, add to the end
        .open("server.log").expect("log error");

    let file = OpenOptions::new()
        .write(true)
        .create_new(true) // fail if file exists
        .open("new_file.txt").expect("file open error");
}
