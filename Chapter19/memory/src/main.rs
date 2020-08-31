use std::thread::spawn;
use std::rc::Rc;

fn main() {
    let rc1 = Rc::new("hello threads".to_string());
    let rc2 = rc1.clone();
    spawn(move || {
        rc2.clone();
    });
    rc1.clone();
    println!("Hello, world!");
}
