fn call_twice<F>(mut clousure: F) where F: FnMut() {
    clousure();
    clousure();
}

fn main() {
    let mut i = 0;
    let incr = || {
        i += 1;
        println!("Ding! i is now: {}", i);
    };
    call_twice(incr);
     let mut i = 0;
     call_twice(|| i+=1);
     assert_eq!(i, 2);
}
