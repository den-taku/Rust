macro_rules! assert_eq2 {
    ($left:expr, $right:expr) => ({
        match (&$left, &$right) {
            (left_var, right_var) => {
                if !(*left_var == *right_var) {
                    panic!("assertion failed: `(left == right)` \
                            (left: `{:?}`, right: `{:?}`)",
                            left_var, right_var)
                }
            }
        }
    });
}

// macro_rules! STAY {
//     (tmp: i32) => ( tmp );
// }

/*hoge*/

// fn hoge(expr: expr) -> () {
//     expr;
// }

fn main() {
    assert_eq2!(1,2);
    assert_eq!(1,2);
    println!("Hello, world!");
    // println!("{}", STAY!(89_i32));
}
