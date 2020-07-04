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
}
