#[macro_use] extern crate lazy_static;

use std::sync::atomic::{AtomicUsize, ATOMIC_USIZE_INIT, Ordering};
use std::sync::Mutex;

/// Number of packets the server has seccessfully handled.
static PACKETS_SERVED: AtomicUsize = ATOMIC_USIZE_INIT;

// trial
// static HOGE: AtomicUsize = AtomicUsize::new(0);

lazy_static! {
    static ref HOSTNAME: Mutex<String> = Mutex::new(String::new());
}

fn main() {
    for _ in 1..100 {
        PACKETS_SERVED.fetch_add(1, Ordering::SeqCst);
    }
}
