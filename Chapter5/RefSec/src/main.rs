fn main() {
    let x = 1;
    {
        let r = &x;
        assert_eq!(*r, 1);
    }
}
