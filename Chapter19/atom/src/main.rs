use std::sync::atomic::{Ordering, AtomicBool, AtomicIsize};
use std::sync::Arc;

fn main() {
    let atom = AtomicIsize::new(0);
    atom.fetch_add(1, Ordering::SeqCst);

    let cancel_flag = Arc::new(AtomicBool::new(false));
    let worker_cancel_flag = cancel_flag.clone();

    let worker_handle = spawn(move || {
        for pixel in animation.pixels_mut() {
            render(pixel);
            if worker_cancel_flag.load(Ordering::SeqCst) {
                return None;
            }
        }
        Some(animation)
    });

    println!("Hello, world!");
}
