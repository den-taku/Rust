fn main() {
    let text = "I see the eigenvalue in thine eye";
    let (head, tail) = text.split_at(21);
    println!("{}",head);
    println!("{}",tail);
    let t = (12, "eggs");
    let _b = Box::new(t);
}
