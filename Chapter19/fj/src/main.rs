use std::thread::spawn;
use std::io;
use std::collections::HashMap;
use std::sync::Arc;

type GigabyteMap = HashMap<char, String>;

fn process_files_in_parallel(filename: Vec<String>,
                             glossary: Arc<GigabyteMap>)
    -> io::Result<()> 
{
    // Divede the work into several chunks.
    const NTHREADS: usize = 8;
    let worklists = split_vec_into_chunks(filename, NTHREADS);

    // Fork: Spawn a thread to handle each chunk.
    let mut thread_handles = vec![];
    for worklist in worklists {
        // This call to .clone() only clones the Arc and bumps the 
        // reference count. It does not clone the GigabyteMap.
        let glossary_for_child = glossary.clone();
        thread_handles.push(
            spawn(move || process_files(worklist, &glossary_for_child))
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
