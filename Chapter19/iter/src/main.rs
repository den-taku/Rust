use std::sync::mpsc;
use std::thread::spawn;

pub trait OffThreadExt: Iterator {
    /// Transform this iterrator into an off_thread iterator: the
    /// `next()` calls happen on a separete worker thread, so the
    /// iterator and the body of your loop run concurrently.
    fn off_thread(self) -> mpsc::IntoIter<Self::Item>;
}

impl<T> OffThreadExt for T
    where T: Iterator + Send + 'static,
          T::Item: Send + 'static
{
    fn off_thread(self) -> mpsc::IntoIter<Self::Item> {
        // Create a channel to transfer items from the worker thread.
        let (sender, receiver) = mpsc::sync_channel(1024);

        // Move this iterator to a new worker thread and run it there.
        spawn(move || {
            for item in self {
                if sender.send(item).is_err() {
                    break;
                }
            }
        });
        // Return an iterator that pulls values from the channel.
        receiver.into_iter()
    }
}

fn main() {
    println!("Hello, world!");
}
