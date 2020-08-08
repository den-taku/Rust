fn main() {
    let int = 2;
    match int {
        0 ... 2 => println!("0...2"),
        _       => ()
    }
    let tpl = (32, 32);
    match tpl {
        (a, b) if b < 30 => { println!("qawsedrftgyhujiolp( {}, {} )", a, b); }
        (a, b) if b > 30 => { println!("( {}, {} )", a, b); }
        _      => {}
    }
}
