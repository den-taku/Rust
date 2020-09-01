use std::sync::Mutex;

fn main() {
    let mutex = Mutex::new(vec![1]);
    
    let lock1 = mutex.lock().unwrap();
    let lock2 = mutex.lock().unwrap(); // deadlock
    println!("Hello, world!");
}
