static mut STASH: &i32 = &10;
static WORTH_POINTING_AT: i32 = 1000;

fn f(p: &'static i32) { 
    unsafe {
        STASH = p;
    }
}

fn _smallest<'a>(v: &'a [i32]) -> &'a i32 {
    let mut s = &v[0];
    for r in &v[1..]{
        if *r < *s { s = r; }
    }
    s
}

fn ff<'a, 'b>(r: &'a i32, s: &'b i32) -> &'a i32 { r }

fn main() {
    f(&WORTH_POINTING_AT);
    let x = 1;
    {
        let r = &x;
        assert_eq!(*r, 1);
    }

    struct S<'b>{
        r: &'b i32
    }

    {
        let s;
        let  x = 10;
        s = S { r: &x };
        assert_eq!(*s.r, 10);
    }
    struct _T<'hoge> {
        s: S<'hoge>
    }

    struct U<'a, 'b> {
        x: &'a i32,
        y: &'b i32,
    }

    let x = 10;
    let r;
    {
        let y = 20;
        {
            let s = U {x: &x, y: &y};
            r = s.x;
        }
    }

    struct StringTable {
        elements: Vec<String>,
    }

    impl StringTable {
        fn find_by_prefix<'a, 'b>(&'a self, prefix: &'b str) -> Option<&'a String> {
            for i in 0..self.elements.len() {
                if self.elements[i].starts_with(prefix) {
                    return Some(&self.elements[i]);
                }
            }
            None
        }
    }

}
