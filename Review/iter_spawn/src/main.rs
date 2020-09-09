use std::sync::mpsc;
use std::thread::spawn;

pub trait OffThreadExt: Iterator {
    fn off_thread(self) -> mpsc::IntoIter<Self::Item>;
}

impl<T> OffThreadExt for T
    where T: Iterator + Send + 'static,
          T::Item: Send + 'static
{
    fn off_thread(self) -> mpsc::IntoIter<Self::Item> {
        let (sender, receiver) = mpsc::sync_channel(1024);

        spawn(move || {
            for item in self {
                if sender.send(item).is_err() {
                    break;
                }
            }
        });

        receiver.into_iter()
    }
}

fn main() {
    println!("Hello, world!");
}
