static mut STASH: &i32 = &10;
static WORTH_POINTING_AT: i32 = 1000;
fn f(p: &'static i32) { 
    unsafe {
        STASH = p;
    }
}

fn main() {
    f(&WORTH_POINTING_AT);
    let x = 1;
    {
        let r = &x;
        assert_eq!(*r, 1);
    }
}
