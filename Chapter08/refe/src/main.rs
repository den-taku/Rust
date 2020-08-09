fn main() {
    let mut v = vec![3, 4, 5, 6];
    {
        let u = &mut v;
        u.push(7);
    }
    for u in &v {
        println!("{}", u);
    }
}
