trait StrignSet {
    /// Return a new empty set.
    fn new() -> Self
        where Self: Sized;

    /// Return a set that contains all the strings in `strings`.
    fn from_slice(strings: &[&str]) -> Self
        where Self: Sized;

    /// Find out if this set contains a particular `value`.
    fn contains(&self, string: &str) -> bool;

    /// Add a string to this set.
    fn add(&mut self, string: &str);
}

/// Return the set of words in `document` that aren't in `wordlist`.
fn unknown_words<S: StringSet>(document: &Vec<String>, wordlist: &S) -> S {
    let mut unknowns = S::new();
    for word in document {
        if !wordlist.contains(word) {
            unknowns.add(word);
        }
    }
    unknowns
}

fn main() {
    // Create sets of two hypothetical types that impl StringSet:
    let set1 = SortedStrignSet::new();
    let set2 = HashedStringSet::new();    

    println!("Hello, world!");
    println!("Hello, world!");
}
