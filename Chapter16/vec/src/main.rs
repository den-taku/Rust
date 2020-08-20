use std::collections::HashSet;

fn main() {
    // Create an empty vector
    let mut numbers: Vec<i32> = vec![];

    // Create a vector with given contents
    let words = vec!["step", "on", "no", "pets"];
    let mut buffer = vec![0u8; 1024]; // 1024 zeroed-out bytes
    let lines = vec!["hoge".to_string(); 10];
    
    // Create another collection to a vector
    // let _my_vec = my_set.into_iter().collect::<Vec<String>>();

    // Get a reference to an element
    let first_word = &lines[0];
    
    // Get a copy of an element
    // let fifth_number = numbers[4]; // requires Copy
    let second_line = lines[1].clone(); // requires Clone

    // Get a reference to a slice
    let my_ref = &buffer[4..12];

    // Get a copy for a slice
    let my_copy = buffer[4..12].to_vec(); // requires Clone

    if let Some(item) = words.first() {
        println!("{}", item);
    }

    let slice = [0, 1, 2, 3];
    assert_eq!(slice.get(2), Some(&2));
    assert_eq!(slice.get(4), None);

    let mut slice = [0, 1, 2, 3];
    {
        let last = slice.last_mut().unwrap(); // type of last: &mut i32
        assert_eq!(*last, 3);
        *last = 100;
    }
    assert_eq!(slice, [0, 1, 2, 100]);
    let v = [1, 2, 3, 4, 5, 6, 7, 8, 9];
    println!("Count!");
    for e in v.to_vec() {
        println!("{}", e);
    }

    let mut byte_vec = b"Misssssssssissssipppppi".to_vec();
    byte_vec.dedup();
    assert_eq!(&byte_vec, b"Misisipi");

    let mut byte_vec = b"Misssssssssissssipppppi".to_vec();
    let mut seen = HashSet::new();
    byte_vec.retain(|r| seen.insert(*r));
    assert_eq!(&byte_vec, b"Misp");

    assert_eq!([[1,2], [3,4], [5,6]].concat(), vec![1,2,3,4,5,6]);
    assert_eq!([[1,2], [3,4], [5,6]].join(&0), vec![1,2,0,3,4,0,5,6]);

    let mut i: usize = 0;
    let mut j: usize = 2;

    let v = vec![0, 1, 2, 3];
    let a = &v[i];
    let b = &v[j];

    let mid = v.len() / 2;
    let front_half = &v[..mid];
    let back_half = &v[mid..];

    let j = 0;

    let mut v = vec![0, 1, 2, 3];
    let a = &mut v[i];
    let b = &mut v[j];

    // *a = 8; // error

    let daily_high_temperatures = vec![12,23,53,32,54,425,532,425,43,523,43,53,42,54,75];
    let changes = daily_high_temperatures
                  .windows(2)
                  .map(|w| w[1] - w[0])
                  .collect::<Vec<_>>();
    for c in changes {
        println!("{}", c);
    }

    let mut my_vec = vec![1, 3, 5, 7, 9]; 
    my_vec.retain(|&val| val <= 4);
    for e in &my_vec {
        println!("{}", e);
    }
}




































































































