use std::rc::Rc;

fn main() {
    #[derive(Copy, Clone)]
    struct Label { number: u32 }
 
    fn print(l: Label) { println!("STAMP: {}", l.number); }

    let l = Label { number: 3 };
    print(l);
    println!("My label number is : {}", l.number);

    let s: Rc<String> = Rc::new("shirataki".to_string());
    let _t: Rc<String> = s.clone();
    let u: Rc<String> = s.clone();

    println!("{} are quite chewy, almost bouncy, but lack flavor", u);

}
