mod Variable {
    pub static mut INNER: u32 = 0;
}

impl Tpl {
    pub fn new(tmp1: u32, tmp2: u64, tmp3: f32) -> Self {
        Tpl(tmp1, tmp2, tmp3)
    }
}

struct Tpl(u32, u64, f32);

fn main() {
    println!("Hello, world!");
    let hoge = &(println!("Hello!"));
    assert_eq!(hoge, &());
    unsafe{
        println!("{}", Variable::INNER);
    }
    let tpl = Tpl(32, 64, 32.0);
    println!("{}", tpl.0);
    println!("{}", Tpl::new(54, 65, 90.0).2);
} 

