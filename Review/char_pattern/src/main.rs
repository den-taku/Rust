fn main() {
    let string = "ewzxsrdtffewhuqibgfuequvxbhjrkxy325   guo".to_string();
    let pattern: [char;12] = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l'];
    if string.contains(&pattern as &[char]) {
        println!("Your understandment win!");
    }
    println!("{:?}", string.chars());
    println!("{:?}", string.bytes());
    println!("{:?}", &pattern);
    println!("{:?}", &pattern as &[char]);
}
