fn main() {
    let pot = "pasta".to_string();
    let plate;

    plate = pot;

    let mut noodles = vec!["udon".to_string()];
    let soba = "soba".to_string();
    let last;

    noodles.push(soba);

    last = noodles.pop().unwrap();
}
