use std::thread::spawn;
use std::io;

fn process_files_in_parallel(filename: Vec<String>) -> io::Result<()> {
    // Divede the work into several chunks.
    const NTHREADS: usize = 8;
    let worklists = split_vec_into_chunks(filename, NTHREADS);

    // Fork: Spawn a thread to handle each chunk.
    let mut thread_handles = vec![];
    for worklist in worklists {
        thread_handles.push(
            spawn(move || process_files(worklist))
        );
    }

    // Join: Wait for all threads to finish.
    for handle in thread_handles {
        handle.join().unwrap()?;
    }

    Ok(())
}

fn main() {
    spawn(|| {
        println!("Hello, world!");
    });
}
