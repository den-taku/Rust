fn main() {
    // let text = "I see the eigenvalue in thine eye";
    // let (head, tail) = text.split_at(21);
    // println!("{}",head);
    // println!("{}",tail);
    // let t = (12, "eggs");
    // let _b = Box::new(t);

    let mut sieve = [true; 10000];
    for i in 2..100 {
        if sieve[i] {
            let mut j = i * i;
            while j < 10000 {
                sieve[j] = false;
                j += i;
            }
        }
    }
    sieve.sort();
    // for i in 1..100 {
    //     println!("{}", sieve[i]);
    // }
    // println!("{}", sieve);
    
    let mut v = vec![2, 3, 5, 7];
    v.push(11);
    v.push(13);
    println!("{}", v.len());
    println!("{}", v.capacity());
    println!("{}", v.iter().fold(1, |a, b| a* b));

    let languages: Vec<String> = std::env::args().skip(1).collect();
    for l in languages {
        println!("{}: {}", l,
                 if l.len() % 2 == 0 {
                     "functional"
                 } else {
                     "imperative"
                 });
    }
}
